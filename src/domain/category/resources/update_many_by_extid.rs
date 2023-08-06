use std::sync::Arc;

use crate::domain::article::model::ArticleModel;
use crate::domain::{
    article::{model::ArticleUpdateModel, repository::ArticleRepository},
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn ArticleRepository>,
    article_update_models: Vec<ArticleUpdateModel>,
) -> Result<Vec<ArticleModel>, DomainError> {
    let articles = article_repository.update_many(article_update_models).await?;
    Ok(articles)
}

#[cfg(test)]
mod tests {
    use crate::domain::article::model::ArticleUpdateModel;
    use crate::domain::article::model::ArticleCreateModel;
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
    async fn article_created() {
        let mut article_repository = MockFakeArticleRepository::new();

        article_repository
            .expect_update_many()
            .return_once(|_| Ok(vec![ArticleModel::mock_default()]));

        let result = execute(
            Arc::new(article_repository),
            vec![ArticleUpdateModel::mock_default()],

        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
