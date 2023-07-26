use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};
use crate::{api::lib::{BatchOperations, DiffOperations}, domain::{diff_article::{model::DiffArticleModel, repository::DiffArticleRepository}, error::DomainError}};


pub struct PgDiffArticleRepository {
    pool: Arc<Pool>,
}
impl PgDiffArticleRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

const QUERY_FIND_ARTICLE: &str = "
    select
        key,
        value
    from
        diff_article";


const QUERY_INSERT_ARTICLE: &str = "
        insert into diff_article(key,value)
        values
            ($1,$2);";
    
const QUERY_UPDATE_ARTICLE_BY_KEY: &str = "
        update
            diff_article 
        set
            \"value\"=$2
        where
            \"key\" = $1;";

impl DiffArticleRepository for PgDiffArticleRepository {
    
}
#[async_trait]
impl DiffOperations<DiffArticleModel> for PgDiffArticleRepository {
    async fn find_by_extids(&self, extids: Vec<String>) -> Result<Vec<DiffArticleModel>, DomainError> {
        let client = self.pool.get().await?;

        let placeholders: Vec<String> = (1..=extids.len()).map(|i| format!("${}", i)).collect();
        let placeholders_str = placeholders.join(",");
        
        let queries: Vec<String> = vec![
            format!(
                "diff_article.key in ({})",
                placeholders_str
            )
        ];
        let params: Vec<&(dyn ToSql + Sync)> = extids.iter().map(|x| x as &(dyn ToSql + Sync)).collect();

        let mut query = String::from(QUERY_FIND_ARTICLE);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let stmt = client.prepare_typed(&query, &[]).await?;
        let result = client.query(&stmt, &params[..]).await?;

        let diff_articles: Vec<DiffArticleModel> = result.iter().map(|row| row.into()).collect();

        return Ok(diff_articles);
    }
}
#[async_trait]
impl BatchOperations<DiffArticleModel,DiffArticleModel,DiffArticleModel> for PgDiffArticleRepository {
    async fn insert_many(&self, items: Vec<DiffArticleModel>) -> Result<Vec<DiffArticleModel>, DomainError> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(QUERY_INSERT_ARTICLE).await?;
        for article in &items {
            client.execute(&stmt, &[
                &article.key,
                &article.value,
            ]).await?;
        }
    
        Ok(items)
    }

    async fn update_many(&self, items: Vec<DiffArticleModel>) -> Result<Vec<DiffArticleModel>, DomainError> {
        let client = self.pool.get().await?;

        let stmt = client.prepare(QUERY_UPDATE_ARTICLE_BY_KEY).await?;
        for article in &items {
            client.execute(&stmt, &[
                &article.key,
                &article.value,
            ]).await?;
        }
    
        Ok(items)
    }
}


impl From<&Row> for DiffArticleModel {
    fn from(row: &Row) -> Self {
        Self {
            key: row.get("key"),
            value: row.get("value"),
        }
    }
}

