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
    id: i32,
    article_update_model: ArticleUpdateModel,
) -> Result<ArticleModel, DomainError> {
    let has_article = article_repository.find_by_articleid(&id).await?;
    if has_article.is_none() {
        return Err(DomainError::NotFound(String::from("Article id not found")));
    }

    let category = article_repository
        .update_by_articleid(&id, &article_update_model)
        .await?;

    Ok(category)
}

#[cfg(test)]
mod tests {
    use crate::{domain::article::model::ArticleCreateModel, api::utils::random_number};

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
    async fn it_should_return_article_updated() {
        let mut article_repository = MockFakeArticleRepository::new();

        let mock_article_model = ArticleModel::mock_default();
        let mut mock_request_article_update = ArticleUpdateModel::mock_default();
        mock_request_article_update.name = mock_article_model.name.clone();

        article_repository
            .expect_find_by_articleid()
            .return_once(|_| Ok(Some(mock_article_model)));

        article_repository
            .expect_update_by_articleid()
            .return_once(|_, _| Ok(ArticleModel::mock_default()));

        let response = execute(
            Arc::new(article_repository),
            random_number(),
            mock_request_article_update,
        )
        .await
        .unwrap();

        assert!(response.articleid != 0);
    }

    #[tokio::test]
    async fn it_should_return_error_not_found_article() {
        let mut article_repository = MockFakeArticleRepository::new();
        article_repository
            .expect_find_by_articleid()
            .return_once(|_| Ok(None));

        let result = execute(
            Arc::new(article_repository),
            random_number(),
            ArticleUpdateModel::mock_default(),
        )
        .await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
