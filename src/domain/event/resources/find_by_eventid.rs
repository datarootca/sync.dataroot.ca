use std::sync::Arc;



use crate::domain::{
    event::{model::EventModel, repository::EventRepository},
    error::DomainError,
};

pub async fn execute(
    event_repository: Arc<dyn EventRepository>,
    id: i32,
) -> Result<Option<EventModel>, DomainError> {
    if let Some(event) = event_repository.find_by_eventid(&id).await? {
        return Ok(Some(event));
    }

    Ok(None)
}
#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;

    use crate::{domain::event::model::{EventCreateModel, EventUpdateModel}, api::utils::random_number};

    use super::*;

    mock! {
        pub FakeEventRepository { }

        #[async_trait]
        impl EventRepository for FakeEventRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<EventModel>, u32)>, DomainError>;
            async fn find_by_eventid(&self, id: &i32) -> Result<Option<EventModel>, DomainError>;
            async fn insert(&self,event_create_model: &EventCreateModel) -> Result<EventModel, DomainError>;
            async fn update_by_eventid(&self,id: &i32,event_update_model: &EventUpdateModel) -> Result<EventModel, DomainError>;
            async fn delete_by_eventid(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_event_finded() {
        let mut event_repository = MockFakeEventRepository::new();

        event_repository
            .expect_find_by_eventid()
            .return_once(|_| Ok(Some(EventModel::mock_default())));

        let result = execute(Arc::new(event_repository), random_number()).await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_no_content_event() {
        let mut event_repository = MockFakeEventRepository::new();

        event_repository
            .expect_find_by_eventid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(event_repository), random_number()).await;

        match result {
            Ok(result) => {
                assert!(result.is_none())
            }
            Err(err) => unreachable!("{err}"),
        }
    }
}
