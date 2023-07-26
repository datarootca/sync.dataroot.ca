use std::sync::Arc;

use tokio::time::Interval;

use crate::{domain::{event::{adapter::EventAdapter, repository::EventRepository, model::{EventCreateModel, EventModel, EventUpdateModel}}, diff_event::{repository::DiffEventRepository, model::DiffEventModel}, error::DomainError, article::model::{Processable, Guidable}, group::{repository::GroupRepository, model::GroupModel}}, api::lib::{BatchOperations, DiffOperations}};



pub struct EventSync<A: EventAdapter> {
    adapter: A,
    repository: Arc<dyn EventRepository>,
    diff_repository: Arc<dyn DiffEventRepository>,
    group_repository: Arc<dyn GroupRepository>,
    interval: Interval,
}

impl<A: EventAdapter> EventSync<A> {
    pub fn new(
        adapter: A,
        repository: Arc<dyn EventRepository>,
        diff_repository: Arc<dyn DiffEventRepository>,
        group_repository: Arc<dyn GroupRepository>,
        interval: Interval
    ) -> EventSync<A> {
        EventSync {
            adapter,
            repository,
            diff_repository,
            group_repository,
            interval,
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
        println!("Syncing events...");
    
        let page_size = 100;
        let mut current_page = 1;
        let mut total = 0;
    
        loop {
            let authors_data = self.get_authors(current_page, page_size).await?;
            if let Some((authors, total_authors)) = authors_data {
                total += authors.len();
                self.process_all_events(authors).await?;
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

    async fn process_events(&self, group_models: Vec<GroupModel>) -> Result<(), DomainError> {
        let events = self.adapter.fetch(group_models).await?;
        self.process_items(events).await?;
        Ok(())
    }
    
    async fn get_authors(&self, page: u32, size: u32) -> Result<Option<(Vec<GroupModel>, u32)>, DomainError> {
        self.group_repository.find(&None, &page, &size).await
    }
    
    async fn process_all_events(&self, group_models: Vec<GroupModel>) -> Result<(), DomainError> {
        self.process_events(group_models).await?;
        Ok(())
    }
    
    fn is_end_of_authors(&self, total_processed: u32, total_authors: u32) -> bool {
        total_processed >= total_authors
    }

    async fn process_items(&self, items: Vec<EventCreateModel>) -> Result<(), DomainError> 
    where
        EventCreateModel: Processable + Guidable,
        dyn EventRepository: BatchOperations<EventCreateModel, EventUpdateModel, EventModel>, 
        dyn DiffEventRepository: BatchOperations<DiffEventModel, DiffEventModel, DiffEventModel> + DiffOperations<DiffEventModel>
    {
        let extids: Vec<String> = items.iter().map(|item| item.extid.clone()).collect();
        let existing_items = &self.diff_repository.find_by_extids(extids).await?;

        let mut items_to_insert: Vec<EventCreateModel> = Vec::new();
        let mut diff_items_to_insert: Vec<DiffEventModel> = Vec::new();
        let mut items_to_update: Vec<EventUpdateModel> = Vec::new();
        let mut diff_items_to_update: Vec<DiffEventModel> = Vec::new();

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
        item: &EventCreateModel, 
        existing_item: &DiffEventModel, 
        items_to_update: &mut Vec<EventUpdateModel>, 
        diff_items_to_update: &mut Vec<DiffEventModel>
    ) -> Result<(), DomainError> {
        if existing_item.get_checksum() != item.get_checksum() {
            items_to_update.push(item.to_update());
            diff_items_to_update.push(DiffEventModel::new(item.get_extid(),item.get_checksum()));
        }
        Ok(())
    }

    fn process_new_item(&self, 
        item: &EventCreateModel, 
        items_to_insert: &mut Vec<EventCreateModel>, 
        diff_items_to_insert: &mut Vec<DiffEventModel>
    ) -> Result<(), DomainError> {
        diff_items_to_insert.push(DiffEventModel::new(item.get_extid(),item.get_checksum()));
        items_to_insert.push(item.clone());
        Ok(())
    }

    async fn insert_items(&self, items_to_insert: &Vec<EventCreateModel>, diff_items_to_insert: &Vec<DiffEventModel>) -> Result<(), DomainError> {
        if !items_to_insert.is_empty() {
            let _ = &self.repository.insert_many(items_to_insert.clone()).await?;
            let _ = &self.diff_repository.insert_many(diff_items_to_insert.clone()).await?;
        }
        Ok(())
    }

    async fn update_items(&self, items_to_update: &Vec<EventUpdateModel>, diff_items_to_update: &Vec<DiffEventModel>) -> Result<(), DomainError> {
        if !items_to_update.is_empty() {
            let _ = &self.repository.update_many(items_to_update.clone()).await?;
            let _ = &self.diff_repository.update_many(diff_items_to_update.clone()).await?;
        }
        Ok(())
    }
}
