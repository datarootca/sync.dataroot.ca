use std::sync::Arc;

use crate::domain::{
    diff_group::{model::{
        DiffGroupModel
    }, 
        repository::DiffGroupRepository},
    error::DomainError,
};

pub async fn execute(
    article_repository: Arc<dyn DiffGroupRepository>,
    keys: &Vec<String>
) -> Result<Vec<DiffGroupModel>, DomainError> {
    let articles = article_repository.find_by_extids(keys.to_vec()).await?;
    Ok(articles)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    
    use crate::{api::lib::{BatchOperations, DiffOperations}};
    use crate::{domain::diff_group::model::{DiffGroupModel}, api::utils::random_string};

    mock! {
        pub FakeDiffGroupRepository { }

        #[async_trait]
        impl DiffGroupRepository for FakeDiffGroupRepository {
        }

        #[async_trait]
        impl BatchOperations<DiffGroupModel, DiffGroupModel, DiffGroupModel> for FakeDiffGroupRepository {
            async fn insert_many(&self, _items: Vec<DiffGroupModel>) -> Result<Vec<DiffGroupModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }

            async fn update_many(&self, _items: Vec<DiffGroupModel>) -> Result<Vec<DiffGroupModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }

        #[async_trait]
        impl DiffOperations<DiffGroupModel> for FakeDiffGroupRepository {
            async fn find_by_extids(&self, _extids: Vec<String>) -> Result<Vec<DiffGroupModel>, DomainError> {
                // Mock implementation
                Ok(Vec::new())
            }
        }

    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut article_repository = MockFakeDiffGroupRepository::new();
        article_repository
            .expect_find_by_extids()
            .return_once(|_| Ok(vec![DiffGroupModel::mock_default()]));

        let articles = execute(Arc::new(article_repository), &vec![random_string(5)])
            .await
            .unwrap();

        assert!(!articles.is_empty());
    }
}
