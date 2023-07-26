use async_trait::async_trait;


use crate::domain::error::DomainError;

use super::model::{CityCreateModel, CityModel, CityUpdateModel};

#[async_trait]
pub trait CityRepository: Send + Sync {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<CityModel>, u32)>, DomainError>;
    async fn find_by_cityid(&self, id: &i32) -> Result<Option<CityModel>, DomainError>;
    async fn find_by_extids(&self, extid: Vec<String>) -> Result<Option<Vec<CityModel>>, DomainError>;
    async fn insert(
        &self,
        city_create_model: &CityCreateModel,
    ) -> Result<CityModel, DomainError>;
    async fn update_by_cityid(
        &self,
        id: &i32,
        city_update_model: &CityUpdateModel,
    ) -> Result<CityModel, DomainError>;
    async fn delete_by_cityid(&self, id: &i32) -> Result<(), DomainError>;
}
