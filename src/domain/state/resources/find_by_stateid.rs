use std::sync::Arc;

use crate::domain::{
    state::{model::StateModel, repository::StateRepository},
    error::DomainError,
};

pub async fn execute(
    state_repository: Arc<dyn StateRepository>,
    id: i32,
) -> Result<Option<StateModel>, DomainError> {
    if let Some(state) = state_repository.find_by_stateid(&id).await? {
        return Ok(Some(state));
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::{domain::state::model::{StateCreateModel, StateUpdateModel}, api::utils::random_number};

    use super::*;

    mock! {
        pub FakeStateRepository { }

        #[async_trait]
        impl StateRepository for FakeStateRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<StateModel>, u32)>, DomainError>;
            async fn find_by_stateid(&self, id: &i32) -> Result<Option<StateModel>, DomainError>;
            async fn insert(&self,state_create_model: &StateCreateModel) -> Result<StateModel, DomainError>;
            async fn update_by_stateid(&self,id: &i32,state_update_model: &StateUpdateModel) -> Result<StateModel, DomainError>;
            async fn delete_by_stateid(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_state_finded() {
        let mut state_repository = MockFakeStateRepository::new();

        state_repository
            .expect_find_by_stateid()
            .return_once(|_| Ok(Some(StateModel::mock_default())));

        let result = execute(Arc::new(state_repository), random_number()).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_no_content_state() {
        let mut state_repository = MockFakeStateRepository::new();

        state_repository
            .expect_find_by_stateid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(state_repository), random_number()).await;

        match result {
            Ok(result) => {
                assert!(result.is_none())
            }
            Err(err) => unreachable!("{err}"),
        }
    }
}
