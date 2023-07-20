use async_trait::async_trait;


use crate::domain::error::DomainError;

use super::model::{EventCreateModel, EventModel, EventUpdateModel};

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<EventModel>, u32)>, DomainError>;
    async fn find_by_eventid(&self, id: &i32) -> Result<Option<EventModel>, DomainError>;
    async fn insert(
        &self,
        event_create_model: &EventCreateModel,
    ) -> Result<EventModel, DomainError>;
    async fn update_by_eventid(
        &self,
        id: &i32,
        event_update_model: &EventUpdateModel,
    ) -> Result<EventModel, DomainError>;
    async fn delete_by_eventid(&self, id: &i32) -> Result<(), DomainError>;
}
