use std::sync::Arc;

use crate::domain::{
    article::{model::{
        ArticleModel
    }, 
        repository::ArticleRepository},
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn ArticleRepository>,
    name: Option<String>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<ArticleModel>, u32)>, DomainError> {
    let article = article_repository.find(&name, &page, &page_size).await?;

    if article.is_some() {
        return Ok(article);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::domain::article::model::{ArticleCreateModel, ArticleUpdateModel};

   use crate::api::lib::BatchOperations;
    mock! {
        pub FakeArticleRepository { }

        #[async_trait]
        impl ArticleRepository for FakeArticleRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<ArticleModel>, u32)>, DomainError>;
            async fn find_by_articleid(&self, id: &i32) -> Result<Option<ArticleModel>, DomainError>;
            async fn find_by_extid(&self, extid: &str) -> Result<Option<ArticleModel>, DomainError>;
            async fn insert(&self,article_create_model: &ArticleCreateModel) -> Result<ArticleModel, DomainError>;
            async fn update_by_extid(&self,article_update_model: &ArticleUpdateModel) -> Result<ArticleModel, DomainError>;
            async fn delete_by_articleid(&self, id: &i32) -> Result<(), DomainError>;
        }
        #[async_trait]
        impl BatchOperations<ArticleCreateModel, ArticleUpdateModel, ArticleModel> for FakeArticleRepository {
            async fn insert_many(&self, _items: Vec<ArticleCreateModel>) -> Result<Vec<ArticleModel>, DomainError> {
                // Your implementation here...
            }
        
            async fn update_many(&self, _items: Vec<ArticleUpdateModel>) -> Result<Vec<ArticleModel>, DomainError> {
                // Your implementation here...
            }
        }
    }

    #[tokio::test]
    async fn it_should_return_article_finded() {
        let mut article_repository = MockFakeArticleRepository::new();
        article_repository
            .expect_find()
            .return_once(|_, _, _| Ok(Some((vec![ArticleModel::mock_default()], 1))));

        let (article, count) = execute(Arc::new(article_repository), None, 1, 12)
            .await
            .unwrap()
            .unwrap();

        assert!(!article.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut article_repository = MockFakeArticleRepository::new();
        article_repository
            .expect_find()
            .return_once(|_, _, _| Ok(None));

        let response = execute(Arc::new(article_repository), None, 1, 12)
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
