use std::sync::Arc;

use tokio::time::Interval;

use crate::{domain::{article::{adapter::ArticleAdapter, repository::ArticleRepository, model::{ArticleCreateModel, ArticleModel, ArticleUpdateModel, Guidable, Processable}}, diff_article::{repository::DiffArticleRepository, model::DiffArticleModel}, error::DomainError, registered_author::{repository::{ RegisteredAuthorRepository}, model::RegisteredAuthorModel}}, api::lib::{BatchOperations, DiffOperations}};



pub struct ArticleSync<A: ArticleAdapter> {
    adapter: A,
    repository: Arc<dyn ArticleRepository>,
    diff_repository: Arc<dyn DiffArticleRepository>,
    registered_author_repository: Arc<dyn RegisteredAuthorRepository>,
    interval: Interval,
}

impl<A: ArticleAdapter> ArticleSync<A> {
    pub fn new(
        adapter: A,
        repository: Arc<dyn ArticleRepository>,
        diff_repository: Arc<dyn DiffArticleRepository>,
        registered_author_repository: Arc<dyn RegisteredAuthorRepository>,
        interval: Interval
    ) -> ArticleSync<A> {
        ArticleSync {
            adapter,
            interval,
            repository,
            diff_repository,
            registered_author_repository,
        }
    }

    pub async fn start(&mut self) -> Result<(), DomainError> {
        loop {
            // Wait until the next interval
            self.interval.tick().await;

            // Perform the sync
            self.sync().await?;
        }
    }

    async fn sync(&self) -> Result<(), DomainError> {
        println!("Syncing articles...");
    
        let page_size = 100;
        let mut current_page = 1;
        let mut total = 0;
    
        loop {
            let authors_data = self.get_authors(current_page, page_size).await?;
            if let Some((authors, total_authors)) = authors_data {
                total += authors.len();
                self.process_all_articles(authors).await?;
                if self.is_end_of_authors(total as u32, total_authors) {
                    break;
                } else {
                    current_page += 1;
                }
            } else {
                break;
            }
        }
    
        Ok(())
    }

    async fn process_articles(&self, author: String) -> Result<(), DomainError> {
        let articles = self.adapter.fetch(author).await?;
        self.process_items(articles).await?;
        Ok(())
    }
    
    async fn get_authors(&self, page: u32, size: u32) -> Result<Option<(Vec<RegisteredAuthorModel>, u32)>, DomainError> {
        self.registered_author_repository.find(&None, &page, &size).await
    }
    
    async fn process_all_articles(&self, authors: Vec<RegisteredAuthorModel>) -> Result<(), DomainError> {
        for author in authors {
            self.process_articles(author.name).await?;
        }
        Ok(())
    }
    
    fn is_end_of_authors(&self, total_processed: u32, total_authors: u32) -> bool {
        total_processed >= total_authors
    }

    async fn process_items(&self, items: Vec<ArticleCreateModel>) -> Result<(), DomainError> 
    where
        ArticleCreateModel: Processable + Guidable,
        dyn ArticleRepository: BatchOperations<ArticleCreateModel, ArticleUpdateModel, ArticleModel>, 
        dyn DiffArticleRepository: BatchOperations<DiffArticleModel, DiffArticleModel, DiffArticleModel> + DiffOperations<DiffArticleModel>
    {
        let extids: Vec<String> = items.iter().map(|item| item.extid.clone()).collect();
        let existing_items = &self.diff_repository.find_by_extids(extids).await?;

        let mut items_to_insert: Vec<ArticleCreateModel> = Vec::new();
        let mut diff_items_to_insert: Vec<DiffArticleModel> = Vec::new();
        let mut items_to_update: Vec<ArticleUpdateModel> = Vec::new();
        let mut diff_items_to_update: Vec<DiffArticleModel> = Vec::new();

        for item in items {
            match existing_items.iter().find(|existing_item| existing_item.get_extid() == item.get_extid()) {
                Some(existing_item) => {
                    self.process_existing_item(&item, existing_item, &mut items_to_update, &mut diff_items_to_update)?;
                }
                None => {
                    self.process_new_item(&item, &mut items_to_insert, &mut diff_items_to_insert)?;
                }
            }
        }

        self.insert_items(&items_to_insert, &diff_items_to_insert).await?;
        self.update_items(&items_to_update, &diff_items_to_update).await?;

        Ok(())
    }

    fn process_existing_item(&self, 
        item: &ArticleCreateModel, 
        existing_item: &DiffArticleModel, 
        items_to_update: &mut Vec<ArticleUpdateModel>, 
        diff_items_to_update: &mut Vec<DiffArticleModel>
    ) -> Result<(), DomainError> {
        if existing_item.get_checksum() != item.get_checksum() {
            items_to_update.push(item.to_update());
            diff_items_to_update.push(DiffArticleModel::new(item.get_extid(),item.get_checksum()));
        }
        Ok(())
    }

    fn process_new_item(&self, 
        item: &ArticleCreateModel, 
        items_to_insert: &mut Vec<ArticleCreateModel>, 
        diff_items_to_insert: &mut Vec<DiffArticleModel>
    ) -> Result<(), DomainError> {
        diff_items_to_insert.push(DiffArticleModel::new(item.get_extid(),item.get_checksum()));
        items_to_insert.push(item.clone());
        Ok(())
    }

    async fn insert_items(&self, items_to_insert: &Vec<ArticleCreateModel>, diff_items_to_insert: &Vec<DiffArticleModel>) -> Result<(), DomainError> {
        if !items_to_insert.is_empty() {
            let _ = &self.repository.insert_many(items_to_insert.clone()).await?;
            let _ = &self.diff_repository.insert_many(diff_items_to_insert.clone()).await?;
        }
        Ok(())
    }

    async fn update_items(&self, items_to_update: &Vec<ArticleUpdateModel>, diff_items_to_update: &Vec<DiffArticleModel>) -> Result<(), DomainError> {
        if !items_to_update.is_empty() {
            let _ = &self.repository.update_many(items_to_update.clone()).await?;
            let _ = &self.diff_repository.update_many(diff_items_to_update.clone()).await?;
        }
        Ok(())
    }
}
