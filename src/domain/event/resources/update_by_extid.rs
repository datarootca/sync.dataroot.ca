use std::sync::Arc;



use crate::domain::{
    event::{
        model::{EventModel, EventUpdateModel},
        repository::EventRepository,
    },
    error::DomainError,
};

pub async fn execute(
    event_repository: Arc<dyn EventRepository>,
    event_update_model: EventUpdateModel,
) -> Result<EventModel, DomainError> {
    let has_event = event_repository.find_by_extid(event_update_model.extid.clone()).await?;
    if has_event.is_none() {
        return Err(DomainError::NotFound(String::from("Event extid not found")));
    }

    let event = event_repository
        .update_by_extid(&event_update_model)
        .await?;

    Ok(event)
}

#[cfg(test)]
mod tests {
    use crate::api::lib::BatchOperations;
    use crate::{domain::event::model::EventCreateModel};

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

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
    async fn it_should_return_event_updated() {
        let mut event_repository = MockFakeEventRepository::new();

        let mock_event_model = EventModel::mock_default();
        let mut mock_request_event_update = EventUpdateModel::mock_default();
        mock_request_event_update.name = mock_event_model.name.clone();

        event_repository
            .expect_find_by_extid()
            .return_once(|_| Ok(Some(mock_event_model)));

        event_repository
            .expect_update_by_extid()
            .return_once(|_| Ok(EventModel::mock_default()));

        let response = execute(
            Arc::new(event_repository),
            mock_request_event_update,
        )
        .await
        .unwrap();

        assert!(response.eventid != 0);
    }

    #[tokio::test]
    async fn it_should_return_error_not_found_event() {
        let mut event_repository = MockFakeEventRepository::new();
        event_repository
            .expect_find_by_extid()
            .return_once(|_| Ok(None));

        let result = execute(
            Arc::new(event_repository),
            EventUpdateModel::mock_default(),
        )
        .await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
