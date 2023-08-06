use std::sync::Arc;



use crate::domain::{event::repository::EventRepository, error::DomainError};

pub async fn execute(
    event_repository: Arc<dyn EventRepository>,
    event_id: i32,
) -> Result<(), DomainError> {
    let has_event = event_repository.find_by_eventid(&event_id).await?;
    if has_event.is_none() {
        return Err(DomainError::NotFound(String::from("Event id not found")));
    }

    event_repository.delete_by_eventid(&event_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    
    use crate::api::lib::BatchOperations;
    use crate::{domain::event::model::{
        EventCreateModel, EventModel, EventUpdateModel,
    }, api::utils::random_number};

    use super::*;

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
    async fn it_should_return_void_event_deleted() {
        let mut event_repository = MockFakeEventRepository::new();

        event_repository
            .expect_find_by_eventid()
            .return_once(|_| Ok(Some(EventModel::mock_default())));

        event_repository
            .expect_delete_by_eventid()
            .return_once(|_| Ok(()));

        let result = execute(Arc::new(event_repository), random_number()).await;

        match result {
            Ok(()) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_event_not_found() {
        let mut event_repository = MockFakeEventRepository::new();

        event_repository
            .expect_find_by_eventid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(event_repository), random_number()).await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
