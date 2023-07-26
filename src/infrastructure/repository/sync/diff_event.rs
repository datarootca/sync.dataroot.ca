use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};
use crate::{api::lib::{BatchOperations, DiffOperations}, domain::{diff_event::{model::DiffEventModel, repository::DiffEventRepository}, error::DomainError}};


pub struct PgDiffEventRepository {
    pool: Arc<Pool>,
}
impl PgDiffEventRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

const QUERY_FIND_EVENT: &str = "
    select
        key,
        value
    from
        diff_event";


const QUERY_INSERT_EVENT: &str = "
        insert into diff_event(key,value)
        values
            ($1,$2);";
    
const QUERY_UPDATE_EVENT_BY_KEY: &str = "
        update
            diff_event 
        set
            \"value\"=$2
        where
            \"key\" = $1;";

impl DiffEventRepository for PgDiffEventRepository {
    
}
#[async_trait]
impl DiffOperations<DiffEventModel> for PgDiffEventRepository {
    async fn find_by_extids(&self, extids: Vec<String>) -> Result<Vec<DiffEventModel>, DomainError> {
        let client = self.pool.get().await?;

        let placeholders: Vec<String> = (1..=extids.len()).map(|i| format!("${}", i)).collect();
        let placeholders_str = placeholders.join(",");
        
        let queries: Vec<String> = vec![
            format!(
                "diff_event.key in ({})",
                placeholders_str
            )
        ];
        let params: Vec<&(dyn ToSql + Sync)> = extids.iter().map(|x| x as &(dyn ToSql + Sync)).collect();

        let mut query = String::from(QUERY_FIND_EVENT);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let stmt = client.prepare_typed(&query, &[]).await?;
        let result = client.query(&stmt, &params[..]).await?;

        let diff_events: Vec<DiffEventModel> = result.iter().map(|row| row.into()).collect();

        return Ok(diff_events);
    }
}
#[async_trait]
impl BatchOperations<DiffEventModel,DiffEventModel,DiffEventModel> for PgDiffEventRepository {
    async fn insert_many(&self, items: Vec<DiffEventModel>) -> Result<Vec<DiffEventModel>, DomainError> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(QUERY_INSERT_EVENT).await?;
        for article in &items {
            client.execute(&stmt, &[
                &article.key,
                &article.value,
            ]).await?;
        }
    
        Ok(items)
    }

    async fn update_many(&self, items: Vec<DiffEventModel>) -> Result<Vec<DiffEventModel>, DomainError> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(QUERY_UPDATE_EVENT_BY_KEY).await?;
        for article in &items {
            client.execute(&stmt, &[
                &article.key,
                &article.value,
            ]).await?;
        }
    
        Ok(items)
    }
}


impl From<&Row> for DiffEventModel {
    fn from(row: &Row) -> Self {
        Self {
            key: row.get("key"),
            value: row.get("value"),
        }
    }
}

