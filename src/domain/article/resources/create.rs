use std::sync::Arc;

use crate::domain::article::model::ArticleModel;
use crate::domain::{
    article::{model::ArticleCreateModel, repository::ArticleRepository},
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn ArticleRepository>,
    article_create_model: ArticleCreateModel,
) -> Result<ArticleModel, DomainError> {
    let article = article_repository.insert(&article_create_model).await?;
    Ok(article)
}

#[cfg(test)]
mod tests {
    use crate::domain::article::model::ArticleUpdateModel;

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

    mock! {
        pub FakeArticleRepository { }

        #[async_trait]
        impl ArticleRepository for FakeArticleRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<ArticleModel>, u32)>, DomainError>;
            async fn find_by_articleid(&self, id: &i32) -> Result<Option<ArticleModel>, DomainError>;
            async fn insert(&self,article_create_model: &ArticleCreateModel) -> Result<ArticleModel, DomainError>;
            async fn update_by_articleid(&self,id: &i32,article_update_model: &ArticleUpdateModel) -> Result<ArticleModel, DomainError>;
            async fn delete_by_articleid(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn article_created() {
        let mut article_repository = MockFakeArticleRepository::new();

        article_repository
            .expect_insert()
            .return_once(|_| Ok(ArticleModel::mock_default()));

        let result = execute(
            Arc::new(article_repository),
            ArticleCreateModel::mock_default(),
        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
