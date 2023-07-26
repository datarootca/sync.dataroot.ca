use std::sync::Arc;

use crate::domain::city::model::CityModel;
use crate::domain::{
    city::{model::CityCreateModel, repository::CityRepository},
    error::DomainError,
};

pub async fn execute(
    city_repository: Arc<dyn CityRepository>,
    city_create_model: CityCreateModel,
) -> Result<CityModel, DomainError> {
    let city = city_repository.insert(&city_create_model).await?;
    Ok(city)
}

#[cfg(test)]
mod tests {
    use crate::domain::city::model::CityUpdateModel;

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

    mock! {
        pub FakeCityRepository { }

        #[async_trait]
        impl CityRepository for FakeCityRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<CityModel>, u32)>, DomainError>;
            async fn find_by_cityid(&self, id: &i32) -> Result<Option<CityModel>, DomainError>;
            async fn find_by_extids(&self, extids: Vec<String>) -> Result<Option<Vec<CityModel>>, DomainError>;
            async fn insert(&self,city_create_model: &CityCreateModel) -> Result<CityModel, DomainError>;
            async fn update_by_cityid(&self,id: &i32,city_update_model: &CityUpdateModel) -> Result<CityModel, DomainError>;
            async fn delete_by_cityid(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_city_created() {
        let mut city_repository = MockFakeCityRepository::new();

        city_repository
            .expect_insert()
            .return_once(|_| Ok(CityModel::mock_default()));

        let result = execute(
            Arc::new(city_repository),
            CityCreateModel::mock_default(),
        )
        .await;

        match result {
            Ok(_) => {}
            Err(err) => unreachable!("{err}"),
        }
    }
}
