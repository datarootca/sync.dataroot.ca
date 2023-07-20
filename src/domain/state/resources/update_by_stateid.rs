use std::sync::Arc;

use crate::domain::{
    state::{
        model::{StateModel, StateUpdateModel},
        repository::StateRepository,
    },
    error::DomainError,
};

pub async fn execute(
    state_repository: Arc<dyn StateRepository>,
    id: i32,
    state_update_model: StateUpdateModel,
) -> Result<StateModel, DomainError> {
    let has_state = state_repository.find_by_stateid(&id).await?;
    if has_state.is_none() {
        return Err(DomainError::NotFound(String::from("State id not found")));
    }

    let state = state_repository
        .update_by_stateid(&id, &state_update_model)
        .await?;

    Ok(state)
}

#[cfg(test)]
mod tests {
    use crate::{domain::state::model::StateCreateModel, api::utils::random_number};

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

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
    async fn it_should_return_state_updated() {
        let mut state_repository = MockFakeStateRepository::new();

        let mock_state_model = StateModel::mock_default();
        let mut mock_request_state_update = StateUpdateModel::mock_default();
        mock_request_state_update.name = mock_state_model.name.clone();

        state_repository
            .expect_find_by_stateid()
            .return_once(|_| Ok(Some(mock_state_model)));

        state_repository
            .expect_update_by_stateid()
            .return_once(|_, _| Ok(StateModel::mock_default()));

        let response = execute(
            Arc::new(state_repository),
            random_number().to_owned(),
            mock_request_state_update,
        )
        .await
        .unwrap();

        assert!(response.stateid != 0);
    }

    #[tokio::test]
    async fn it_should_return_error_not_found_state() {
        let mut state_repository = MockFakeStateRepository::new();
        state_repository
            .expect_find_by_stateid()
            .return_once(|_| Ok(None));

        let result = execute(
            Arc::new(state_repository),
            random_number().to_owned(),
            StateUpdateModel::mock_default(),
        )
        .await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
