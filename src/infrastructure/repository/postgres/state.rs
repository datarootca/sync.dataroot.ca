use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};
use crate::domain::{
    state::{
        model::{StateCreateModel, StateModel, StateUpdateModel},
        repository::StateRepository,
    },
    error::DomainError,
};

const QUERY_FIND_STATE: &str = "
    select
        stateid,
        name,
        symbol,
        extid,
        highres_link,
        photo_link,
        thumb_link,
        created_at,
        updated_at,
        count(1) over ()::OID as count
    from
        state";

const QUERY_FIND_STATE_BY_ID: &str = "
    select
        stateid,
        name,
        symbol,
        extid,
        highres_link,
        photo_link,
        thumb_link,
        created_at,
        updated_at,
        count(1) over ()::OID as count
    from
        state
    where 
        stateid = $1;";

const QUERY_INSERT_STATE: &str = "
    insert into state(name,symbol,extid,highres_link,photo_link,thumb_link)
    values
        ($1,$2,$3,$4,$5,$6)
    returning
        stateid,
        name,
        symbol,
        extid,
        highres_link,
        photo_link,
        thumb_link,
        created_at,
        updated_at;";

const QUERY_UPDATE_STATE_BY_ID: &str = "
    update
        state 
    set
        name=$2,
        symbol=$3,
        highres_link=$4,
        photo_link=$5,
        thumb_link=$6,
        updated_at=now()
    where
        stateid = $1
    returning
        stateid,
        name,
        symbol,
        extid,
        highres_link,
        photo_link,
        thumb_link,
        created_at,
        updated_at;";

const QUERY_DELETE_STATE_BY_ID: &str = "
            delete from
                state 
            where
                stateid = $1;";

pub struct PgStateRepository {
    pool: Arc<Pool>,
}
impl PgStateRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StateRepository for PgStateRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<StateModel>, u32)>, DomainError> {
        let client = self.pool.get().await?;

        let mut queries: Vec<String> = vec![];
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(name) = name {
            queries.push(format!(
                "state.name like '%' || ${} || '%'",
                params.len() + 1
            ));
            params.push(name);
        }

        let mut query = String::from(QUERY_FIND_STATE);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let states: Vec<StateModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((states, count)));
        }

        return Ok(None);
    }

    async fn find_by_stateid(&self, id: &i32) -> Result<Option<StateModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_STATE_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        state_create_model: &StateCreateModel,
    ) -> Result<StateModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_STATE).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &state_create_model.name,
                    &state_create_model.symbol,
                    &state_create_model.extid,
                    &state_create_model.highres_link,
                    &state_create_model.photo_link,
                    &state_create_model.thumb_link,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn update_by_stateid(
        &self,
        stateid: &i32,
        state_update_model: &StateUpdateModel,
    ) -> Result<StateModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_UPDATE_STATE_BY_ID).await?;
        let result = &client
        
            .query_one(
                &stmt,
                &[
                    stateid,
                    &state_update_model.name,
                    &state_update_model.symbol,
                    &state_update_model.highres_link,
                    &state_update_model.photo_link,
                    &state_update_model.thumb_link,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn delete_by_stateid(&self, id: &i32) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_STATE_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

impl From<&Row> for StateModel {
    fn from(row: &Row) -> Self {
        Self {
            stateid: row.get("stateid"),
            name: row.get("name"),
            symbol: row.get("symbol"),
            highres_link: row.get("highres_link"),
            photo_link: row.get("photo_link"),
            thumb_link: row.get("thumb_link"),
            extid: row.get("extid"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
