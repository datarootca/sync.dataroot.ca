use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};

use crate::domain::{
    registered_author::{
        model::{RegisteredAuthorCreateModel, RegisteredAuthorModel},
        repository::RegisteredAuthorRepository,
    },
    error::DomainError,
};

const QUERY_FIND_REGISTERED_AUTHOR: &str = "
    select
        ra.registered_authorid,
        ra.name,
        ra.source,
        ra.created_at,
        ra.updated_at,
        count(1) over ()::OID as count
    from
        registered_author ra";

const QUERY_FIND_REGISTERED_AUTHOR_BY_ID: &str = "
    select
        ra.registered_authorid,
        ra.name,
        ra.source,
        ra.created_at,
        ra.updated_at,
        count(1) over ()::OID as count
    from
        registered_author ra
    where 
    registered_authorid = $1;";

const QUERY_INSERT_REGISTERED_AUTHOR: &str = "
    insert into registered_author(name,source)
    values
        ($1,$2)
    returning
        registered_authorid
        name
        source
        created_at
        updated_at;";

const QUERY_DELETE_REGISTERED_AUTHOR_BY_ID: &str = "
            delete from
                registered_author 
            where
                registered_authorid = $1;";

pub struct PgRegisteredAuthorRepository {
    pool: Arc<Pool>,
}
impl PgRegisteredAuthorRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RegisteredAuthorRepository for PgRegisteredAuthorRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<RegisteredAuthorModel>, u32)>, DomainError> {
        let client = self.pool.get().await?;

        let mut queries: Vec<String> = vec![];
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(name) = name {
            queries.push(format!(
                "e.name like '%' || ${} || '%'",
                params.len() + 1
            ));
            params.push(name);
        }

        let mut query = String::from(QUERY_FIND_REGISTERED_AUTHOR);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let items: Vec<RegisteredAuthorModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((items, count)));
        }

        return Ok(None);
    }

    async fn find_by_id(&self, id: &i32) -> Result<Option<RegisteredAuthorModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_REGISTERED_AUTHOR_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        registered_author_create_model: &RegisteredAuthorCreateModel,
    ) -> Result<RegisteredAuthorModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_REGISTERED_AUTHOR).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &registered_author_create_model.name,
                    &registered_author_create_model.source,
                ],
            )
            .await?;

        Ok(result.into())
    }


    async fn delete_by_id(&self, id: &i32) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_REGISTERED_AUTHOR_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

impl From<&Row> for RegisteredAuthorModel {
    fn from(row: &Row) -> Self {
        Self {
            registered_authorid: row.get("registered_authorid"),
            name: row.get("name"),
            source: row.get("source"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
