use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};
use crate::{api::lib::{BatchOperations, DiffOperations}, domain::{diff_group::{model::DiffGroupModel, repository::DiffGroupRepository}, error::DomainError}};


pub struct PgDiffGroupRepository {
    pool: Arc<Pool>,
}
impl PgDiffGroupRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

const QUERY_FIND_GROUP: &str = "
    select
        key,
        value
    from
        diff_group";


const QUERY_INSERT_GROUP: &str = "
        insert into diff_group(key,value)
        values
            ($1,$2);";
    
const QUERY_UPDATE_GROUP_BY_KEY: &str = "
        update
            diff_group 
        set
            \"value\"=$2
        where
            \"key\" = $1;";

impl DiffGroupRepository for PgDiffGroupRepository {
    
}
#[async_trait]
impl DiffOperations<DiffGroupModel> for PgDiffGroupRepository {
    async fn find_by_extids(&self, extids: Vec<String>) -> Result<Vec<DiffGroupModel>, DomainError> {
        let client = self.pool.get().await?;

        let placeholders: Vec<String> = (1..=extids.len()).map(|i| format!("${}", i)).collect();
        let placeholders_str = placeholders.join(",");
        
        let queries: Vec<String> = vec![
            format!(
                "diff_group.key in ({})",
                placeholders_str
            )
        ];
        let params: Vec<&(dyn ToSql + Sync)> = extids.iter().map(|x| x as &(dyn ToSql + Sync)).collect();

        let mut query = String::from(QUERY_FIND_GROUP);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let stmt = client.prepare_typed(&query, &[]).await?;
        let result = client.query(&stmt, &params[..]).await?;

        let diff_groups: Vec<DiffGroupModel> = result.iter().map(|row| row.into()).collect();

        return Ok(diff_groups);
    }
}
#[async_trait]
impl BatchOperations<DiffGroupModel,DiffGroupModel,DiffGroupModel> for PgDiffGroupRepository {
    async fn insert_many(&self, items: Vec<DiffGroupModel>) -> Result<Vec<DiffGroupModel>, DomainError> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(QUERY_INSERT_GROUP).await?;
        for group in &items {
            client.execute(&stmt, &[
                &group.key,
                &group.value,
            ]).await?;
        }
    
        Ok(items)
    }

    async fn update_many(&self, items: Vec<DiffGroupModel>) -> Result<Vec<DiffGroupModel>, DomainError> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(QUERY_UPDATE_GROUP_BY_KEY).await?;
        for group in &items {
            client.execute(&stmt, &[
                &group.key,
                &group.value,
            ]).await?;
        }
    
        Ok(items)
    }
}


impl From<&Row> for DiffGroupModel {
    fn from(row: &Row) -> Self {
        Self {
            key: row.get("key"),
            value: row.get("value"),
        }
    }
}

