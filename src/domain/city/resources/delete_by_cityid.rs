use std::sync::Arc;



use crate::domain::{city::repository::CityRepository, error::DomainError};

pub async fn execute(
    city_repository: Arc<dyn CityRepository>,
    city_id: i32,
) -> Result<(), DomainError> {
    let has_category = city_repository.find_by_cityid(&city_id).await?;
    if has_category.is_none() {
        return Err(DomainError::NotFound(String::from("Category id not found")));
    }

    city_repository.delete_by_cityid(&city_id).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::{domain::city::model::{
        CityCreateModel, CityModel, CityUpdateModel,
    }, api::utils::random_number};

    use super::*;

    mock! {
        pub FakeCityRepository { }

        #[async_trait]
        impl CityRepository for FakeCityRepository {
            async fn find(&self,name: &Option<String>,page: &u32,page_size: &u32) -> Result<Option<(Vec<CityModel>, u32)>, DomainError>;
            async fn find_by_cityid(&self, id: &i32) -> Result<Option<CityModel>, DomainError>;
            async fn insert(&self,city_create_model: &CityCreateModel) -> Result<CityModel, DomainError>;
            async fn update_by_cityid(&self,id: &i32,city_update_model: &CityUpdateModel) -> Result<CityModel, DomainError>;
            async fn delete_by_cityid(&self, id: &i32) -> Result<(), DomainError>;
        }
    }

    #[tokio::test]
    async fn it_should_return_void_city_deleted() {
        let mut city_repository = MockFakeCityRepository::new();

        city_repository
            .expect_find_by_cityid()
            .return_once(|_| Ok(Some(CityModel::mock_default())));

        city_repository
            .expect_delete_by_cityid()
            .return_once(|_| Ok(()));

        let result = execute(Arc::new(city_repository), random_number()).await;

        match result {
            Ok(()) => {}
            Err(err) => unreachable!("{err}"),
        }
    }

    #[tokio::test]
    async fn it_should_return_error_city_not_found() {
        let mut city_repository = MockFakeCityRepository::new();

        city_repository
            .expect_find_by_cityid()
            .return_once(|_| Ok(None));

        let result = execute(Arc::new(city_repository), random_number()).await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
