use async_trait::async_trait;


use crate::{domain::error::DomainError, api::lib::BatchOperations};

use super::model::{EventCreateModel, EventModel, EventUpdateModel};

#[async_trait]
pub trait EventRepository: BatchOperations<EventCreateModel,EventUpdateModel,EventModel> + Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<EventModel>, u32)>, DomainError>;
    async fn find_by_eventid(&self, id: &i32) -> Result<Option<EventModel>, DomainError>;
    async fn find_by_extid(&self, extid: String) -> Result<Option<EventModel>, DomainError>;
    async fn insert(
        &self,
        event_create_model: &EventCreateModel,
    ) -> Result<EventModel, DomainError>;
    async fn update_by_extid(&self,event_update_model: &EventUpdateModel) -> Result<EventModel, DomainError>;
    async fn delete_by_eventid(&self, id: &i32) -> Result<(), DomainError>;
}
