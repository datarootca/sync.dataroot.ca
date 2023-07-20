use std::sync::Arc;

use crate::domain::{
    city::{model::CityModel, repository::CityRepository},
    error::DomainError,
};

pub async fn execute(
    city_repository: Arc<dyn CityRepository>,
    name: Option<String>,
    page: u32,
    page_size: u32,
) -> Result<Option<(Vec<CityModel>, u32)>, DomainError> {
    let article = city_repository.find(&name, &page, &page_size).await?;

    if article.is_some() {
        return Ok(article);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    use async_trait::async_trait;
    use mockall::mock;
    

    use crate::domain::city::model::{CityCreateModel, CityUpdateModel};

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
    async fn it_should_return_article_finded() {
        let mut city_repository = MockFakeCityRepository::new();

        city_repository
            .expect_find()
            .return_once(|_, _, _| Ok(Some((vec![CityModel::mock_default()], 1))));

        let (article, count) = execute(Arc::new(city_repository), None, 1, 12)
            .await
            .unwrap()
            .unwrap();

        assert!(!article.is_empty());
        assert!(count == 1);
    }

    #[tokio::test]
    async fn it_should_return_none_finded() {
        let mut city_repository = MockFakeCityRepository::new();
        city_repository
            .expect_find()
            .return_once(|_, _, _| Ok(None));

        let response = execute(Arc::new(city_repository), None, 1, 12)
            .await
            .unwrap();

        assert!(response.is_none());
    }
}
