use std::sync::Arc;



use crate::domain::{
    article::{
        model::{ArticleModel, ArticleUpdateModel},
        repository::ArticleRepository,
    },
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn ArticleRepository>,
    article_update_model: ArticleUpdateModel,
) -> Result<ArticleModel, DomainError> {
    let has_article = article_repository.find_by_extid(&article_update_model.extid).await?;
    if has_article.is_none() {
        return Err(DomainError::NotFound(String::from("Article id not found")));
    }

    let article = article_repository
        .update_by_extid(&article_update_model)
        .await?;

    Ok(article)
}

#[cfg(test)]
mod tests {
    use crate::{domain::article::model::ArticleCreateModel};

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

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
    async fn it_should_return_article_updated() {
        let mut article_repository = MockFakeArticleRepository::new();

        let mock_article_model = ArticleModel::mock_default();
        let mut mock_request_article_update = ArticleUpdateModel::mock_default();
        mock_request_article_update.name = mock_article_model.name.clone();
        mock_request_article_update.extid = mock_article_model.extid.clone();

        article_repository
            .expect_find_by_extid()
            .return_once(|_| Ok(Some(mock_article_model)));

        article_repository
            .expect_update_by_extid()
            .return_once(|_| Ok(ArticleModel::mock_default()));

        let response = execute(
            Arc::new(article_repository),
            mock_request_article_update,
        )
        .await
        .unwrap();

        assert!(!response.extid.is_empty());
    }
}
