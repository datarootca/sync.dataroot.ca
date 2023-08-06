use std::sync::Arc;

use crate::domain::{
    event::{model::EventModel, repository::EventRepository},
    error::DomainError,
};

pub async fn execute(
    event_repository: Arc<dyn EventRepository>,
    name: Option<String>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<EventModel>, u32)>, DomainError> {
    let event = event_repository.find(&name, &page, &page_size).await?;

    if event.is_some() {
        return Ok(event);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    
    use crate::api::lib::BatchOperations;
    use crate::domain::event::model::{EventCreateModel, EventUpdateModel};

    mock! {
        pub FakeEventRepository { }

        #[async_trait]
        impl EventRepository for FakeEventRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<EventModel>, u32)>, DomainError>;
            async fn find_by_eventid(&self, id: &i32) -> Result<Option<EventModel>, DomainError>;
            async fn find_by_extid(&self, extid: String) -> Result<Option<EventModel>, DomainError>;
            async fn insert(&self,event_create_model: &EventCreateModel) -> Result<EventModel, DomainError>;
            async fn update_by_extid(&self,event_update_model: &EventUpdateModel) -> Result<EventModel, DomainError>;
            async fn delete_by_eventid(&self, id: &i32) -> Result<(), DomainError>;
        }

        #[async_trait]
        impl BatchOperations<EventCreateModel, EventUpdateModel, EventModel> for FakeEventRepository {
            async fn insert_many(&self, _items: Vec<EventCreateModel>) -> Result<Vec<EventModel>, DomainError> {
                // Your implementation here...
            }
        
            async fn update_many(&self, _items: Vec<EventUpdateModel>) -> Result<Vec<EventModel>, DomainError> {
                // Your implementation here...
            }
        }
    }

    #[tokio::test]
    async fn it_should_return_event_finded() {
        let mut event_repository = MockFakeEventRepository::new();

        event_repository
            .expect_find()
            .return_once(|_, _, _| Ok(Some((vec![EventModel::mock_default()], 1))));

        let (event, count) = execute(Arc::new(event_repository), None, 1, 12)
            .await
            .unwrap()
            .unwrap();

        assert!(!event.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut event_repository = MockFakeEventRepository::new();
        event_repository
            .expect_find()
            .return_once(|_, _, _| Ok(None));

        let response = execute(Arc::new(event_repository), None, 1, 12)
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
