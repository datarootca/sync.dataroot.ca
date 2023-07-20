use std::sync::Arc;

use crate::domain::{state::repository::StateRepository, error::DomainError};

pub async fn execute(
    state_repository: Arc<dyn StateRepository>,
    state_id: i32,
) -> Result<(), DomainError> {
    let has_state = state_repository.find_by_stateid(&state_id).await?;
    if has_state.is_none() {
        return Err(DomainError::NotFound(String::from("State id not found")));
    }

    state_repository.delete_by_stateid(&state_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    use crate::api::utils::random_number;

    use crate::domain::state::model::{
        StateCreateModel, StateModel, StateUpdateModel,
    };

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
    async fn it_should_return_void_state_deleted() {
        let mut state_repository = MockFakeStateRepository::new();

        state_repository
            .expect_find_by_stateid()
            .return_once(|_| Ok(Some(StateModel::mock_default())));

        state_repository
            .expect_delete_by_stateid()
            .return_once(|_| Ok(()));

        let result = execute(Arc::new(state_repository), random_number()).await;

        match result {
            Ok(()) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_state_not_found() {
        let mut state_repository = MockFakeStateRepository::new();

        state_repository
            .expect_find_by_stateid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(state_repository), random_number()).await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
