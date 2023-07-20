use std::sync::Arc;



use crate::domain::{
    city::{
        model::{CityModel, CityUpdateModel},
        repository::CityRepository,
    },
    error::DomainError,
};

pub async fn execute(
    city_repository: Arc<dyn CityRepository>,
    id: i32,
    city_update_model: CityUpdateModel,
) -> Result<CityModel, DomainError> {
    let has_category = city_repository.find_by_cityid(&id).await?;
    if has_category.is_none() {
        return Err(DomainError::NotFound(String::from("Category id not found")));
    }

    let category = city_repository
        .update_by_cityid(&id, &city_update_model)
        .await?;

    Ok(category)
}

#[cfg(test)]
mod tests {
    use crate::{domain::city::model::CityCreateModel, api::utils::random_number};

    use super::*;

    use async_trait::async_trait;
    use mockall::mock;

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
    async fn it_should_return_city_updated() {
        let mut city_repository = MockFakeCityRepository::new();

        let mock_city_model = CityModel::mock_default();
        let mut mock_request_city_update = CityUpdateModel::mock_default();
        mock_request_city_update.name = mock_city_model.name.clone();

        city_repository
            .expect_find_by_cityid()
            .return_once(|_| Ok(Some(mock_city_model)));

        city_repository
            .expect_update_by_cityid()
            .return_once(|_, _| Ok(CityModel::mock_default()));

        let response = execute(
            Arc::new(city_repository),
            random_number(),
            mock_request_city_update,
        )
        .await
        .unwrap();

        assert!(response.cityid != 0);
    }

    #[tokio::test]
    async fn it_should_return_error_not_found_category() {
        let mut city_repository = MockFakeCityRepository::new();
        city_repository
            .expect_find_by_cityid()
            .return_once(|_| Ok(None));

        let result = execute(
            Arc::new(city_repository),
            random_number(),
            CityUpdateModel::mock_default(),
        )
        .await;

        match result {
            Err(DomainError::NotFound(_)) => {}
            _ => unreachable!(),
        }
    }
}
