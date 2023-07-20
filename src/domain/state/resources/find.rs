use std::sync::Arc;

use crate::domain::{
    state::{model::StateModel, repository::StateRepository},
    error::DomainError,
};

pub async fn execute(
    state_repository: Arc<dyn StateRepository>,
    name: Option<String>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<StateModel>, u32)>, DomainError> {
    let state = state_repository.find(&name, &page, &page_size).await?;

    if state.is_some() {
        return Ok(state);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

    use crate::domain::state::model::{StateCreateModel, StateUpdateModel};

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
            .expect_find()
            .return_once(|_, _, _| Ok(Some((vec![StateModel::mock_default()], 1))));

        let (state, count) = execute(Arc::new(state_repository), None, 1, 12)
            .await
            .unwrap()
            .unwrap();

        assert!(!state.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut state_repository = MockFakeStateRepository::new();
        state_repository
            .expect_find()
            .return_once(|_, _, _| Ok(None));

        let response = execute(Arc::new(state_repository), None, 1, 12)
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
