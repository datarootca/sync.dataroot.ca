use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};

use crate::domain::{
    city::{
        model::{CityCreateModel, CityModel, CityUpdateModel},
        repository::CityRepository,
    },
    error::DomainError,
};

const QUERY_FIND_CITY: &str = "
    select
        cityid,
        stateid,
        name,
        slug,
        extid,
        highres_link,
        photo_link,
        thumb_link,
        created_at,
        updated_at,
        count(1) over ()::OID as count
    from
        city";

const QUERY_FIND_CITY_BY_ID: &str = "
    select
        cityid,
        stateid,
        name,
        slug,
        extid,
        highres_link,
        photo_link,
        thumb_link,
        created_at,
        updated_at,
        count(1) over ()::OID as count
    from
        city
    where 
        cityid = $1;";

const QUERY_INSERT_CITY: &str = "
    insert into city(stateid,name,slug,extid,highres_link,photo_link,thumb_link)
    values
        ($1,$2,$3,$4,$5,$6,$7)
    returning
        cityid,
        stateid,
        name,
        slug,
        extid,
        highres_link,
        photo_link,
        thumb_link,
        created_at,
        updated_at;";

const QUERY_UPDATE_CITY_BY_ID: &str = "
    update
        city 
    set
        name=$2,
        slug=$3,
        stateid=$4,
        highres_link=$5,
        photo_link=$6,
        thumb_link=$7,
        updated_at=now()
    where
        cityid = $1
    returning
        cityid,
        stateid,
        name,
        slug,
        extid,
        highres_link,
        photo_link,
        thumb_link,
        created_at,
        updated_at;";

const QUERY_DELETE_CITY_BY_ID: &str = "
            delete from
                city 
            where
                cityid = $1;";

pub struct PgCityRepository {
    pool: Arc<Pool>,
}
impl PgCityRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CityRepository for PgCityRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<CityModel>, u32)>, DomainError> {
        let client = self.pool.get().await?;

        let mut queries: Vec<String> = vec![];
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(name) = name {
            queries.push(format!(
                "city.name like '%' || ${} || '%'",
                params.len() + 1
            ));
            params.push(name);
        }

        let mut query = String::from(QUERY_FIND_CITY);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let city_items: Vec<CityModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((city_items, count)));
        }

        return Ok(None);
    }

    async fn find_by_cityid(&self, id: &i32) -> Result<Option<CityModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_CITY_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        city_create_model: &CityCreateModel,
    ) -> Result<CityModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_CITY).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &city_create_model.stateid,
                    &city_create_model.name,
                    &city_create_model.slug,
                    &city_create_model.extid,
                    &city_create_model.highres_link,
                    &city_create_model.photo_link,
                    &city_create_model.thumb_link,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn update_by_cityid(
        &self,
        cityid: &i32,
        city_update_model: &CityUpdateModel,
    ) -> Result<CityModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_UPDATE_CITY_BY_ID).await?;
        let result = &client
        
            .query_one(
                &stmt,
                &[
                    cityid,
                    &city_update_model.name,
                    &city_update_model.slug,
                    &city_update_model.stateid,
                    &city_update_model.highres_link,
                    &city_update_model.photo_link,
                    &city_update_model.thumb_link,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn delete_by_cityid(&self, id: &i32) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_CITY_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

impl From<&Row> for CityModel {
    fn from(row: &Row) -> Self {
        Self {
            cityid: row.get("cityid"),
            stateid: row.get("stateid"),
            name: row.get("name"),
            slug: row.get("slug"),
            highres_link: row.get("highres_link"),
            photo_link: row.get("photo_link"),
            thumb_link: row.get("thumb_link"),
            extid: row.get("extid"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
