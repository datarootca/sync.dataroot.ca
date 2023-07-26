use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};

use crate::domain::{
    registered_group::{
        model::{RegisteredGroupCreateModel, RegisteredGroupModel},
        repository::RegisteredGroupRepository,
    },
    error::DomainError,
};

const QUERY_FIND_REGISTERED_GROUP: &str = "
    select
        rg.registered_groupid,
        rg.name,
        rg.source,
        rg.created_at,
        rg.updated_at,
        count(1) over ()::OID as count
    from
        registered_group rg";

const QUERY_FIND_REGISTERED_GROUP_BY_ID: &str = "
    select
        rg.registered_groupid,
        rg.name,
        rg.source,
        rg.created_at,
        rg.updated_at,
        count(1) over ()::OID as count
    from
        registered_group rg
    where 
    registered_groupid = $1;";

const QUERY_INSERT_REGISTERED_GROUP: &str = "
    insert into registered_group(name,source)
    values
        ($1,$2)
    returning
        registered_groupid
        name
        source
        created_at
        updated_at;";

const QUERY_DELETE_REGISTERED_GROUP_BY_ID: &str = "
            delete from
                registered_group 
            where
                registered_groupid = $1;";

pub struct PgRegisteredGroupRepository {
    pool: Arc<Pool>,
}
impl PgRegisteredGroupRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegisteredGroupRepository for PgRegisteredGroupRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<RegisteredGroupModel>, u32)>, DomainError> {
        let client = self.pool.get().await?;

        let mut queries: Vec<String> = vec![];
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(name) = name {
            queries.push(format!(
                "rg.name like '%' || ${} || '%'",
                params.len() + 1
            ));
            params.push(name);
        }

        let mut query = String::from(QUERY_FIND_REGISTERED_GROUP);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let items: Vec<RegisteredGroupModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((items, count)));
        }

        return Ok(None);
    }

    async fn find_by_id(&self, id: &i32) -> Result<Option<RegisteredGroupModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_REGISTERED_GROUP_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        registered_group_create_model: &RegisteredGroupCreateModel,
    ) -> Result<RegisteredGroupModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_REGISTERED_GROUP).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &registered_group_create_model.name,
                    &registered_group_create_model.source,
                ],
            )
            .await?;

        Ok(result.into())
    }


    async fn delete_by_id(&self, id: &i32) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_REGISTERED_GROUP_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

impl From<&Row> for RegisteredGroupModel {
    fn from(row: &Row) -> Self {
        Self {
            registered_groupid: row.get("registered_groupid"),
            name: row.get("name"),
            source: row.get("source"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
