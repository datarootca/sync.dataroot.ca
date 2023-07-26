use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};

use crate::{domain::{
    article::{
        model::{ArticleCreateModel, ArticleModel, ArticleUpdateModel},
        repository::ArticleRepository,
    },
    error::DomainError,
}, api::lib::BatchOperations};

const QUERY_FIND_ARTICLE: &str = "
    select
        articleid,
        extid,
        name,
        description,
        time_m,
        publish_at,
        source,
        link,
        author,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link,
        count(1) over ()::OID as count
    from
        article";

const QUERY_FIND_ARTICLE_BY_ID: &str = "
    select
        articleid,
        extid,
        name,
        description,
        time_m,
        publish_at,
        source,
        link,
        author,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link,
        count(1) over ()::OID as count
    from
        article
    where 
        articleid = $1;";

const QUERY_FIND_ARTICLE_BY_EXTID: &str = "
    select
        articleid,
        extid,
        name,
        description,
        time_m,
        publish_at,
        source,
        link,
        author,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link,
        count(1) over ()::OID as count
    from
        article
    where 
        extid = $1;";

const QUERY_INSERT_ARTICLE: &str = "
    insert into article(extid,name,description,time_m,source,link,author,highres_link,photo_link,thumb_link,publish_at)
    values
        ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
    returning
        articleid,
        extid,
        name,
        description,
        time_m,
        publish_at,
        source,
        link,
        author,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link;";

const QUERY_UPDATE_ARTICLE_BY_ID: &str = "
    update
        article 
    set
        name=$2,
        description=$3,
        time_m=$4,
        publish_at=$5,
        link=$6,
        author=$7,
        highres_link=$8,
        photo_link=$9,
        thumb_link=$10,
        updated_at=now()
    where
        extid = $1
    returning
        articleid,
        extid,
        name,
        description,
        time_m,
        publish_at,
        source,
        link,
        author,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link;";

const QUERY_DELETE_ARTICLE_BY_ID: &str = "
            delete from
                article 
            where
                articleid = $1;";

pub struct PgArticleRepository {
    pool: Arc<Pool>,
}
impl PgArticleRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArticleRepository for PgArticleRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<ArticleModel>, u32)>, DomainError> {
        let client = self.pool.get().await?;

        let mut queries: Vec<String> = vec![];
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(name) = name {
            queries.push(format!(
                "article.name like '%' || ${} || '%'",
                params.len() + 1
            ));
            params.push(name);
        }
        
        let mut query = String::from(QUERY_FIND_ARTICLE);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let articles: Vec<ArticleModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((articles, count)));
        }

        return Ok(None);
    }

    async fn find_by_articleid(&self, id: &i32) -> Result<Option<ArticleModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_ARTICLE_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn find_by_extid(&self, extid: &str) -> Result<Option<ArticleModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_ARTICLE_BY_EXTID).await?;

        if let Some(result) = client.query_opt(&stmt, &[&extid]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        article_create_model: &ArticleCreateModel,
    ) -> Result<ArticleModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_ARTICLE).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &article_create_model.extid,
                    &article_create_model.name,
                    &article_create_model.description,
                    &article_create_model.time_m,
                    &article_create_model.source,
                    &article_create_model.link,
                    &article_create_model.author,
                    &article_create_model.highres_link,
                    &article_create_model.photo_link,
                    &article_create_model.thumb_link,
                    &article_create_model.publish_at,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn update_by_extid(
        &self,
        article_update_model: &ArticleUpdateModel,
    ) -> Result<ArticleModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_UPDATE_ARTICLE_BY_ID).await?;
        let result = &client
        
            .query_one(
                &stmt,
                &[
                    &article_update_model.extid,
                    &article_update_model.name,
                    &article_update_model.description,
                    &article_update_model.time_m,
                    &article_update_model.publish_at,
                    &article_update_model.link,
                    &article_update_model.author,
                    &article_update_model.highres_link,
                    &article_update_model.photo_link,
                    &article_update_model.thumb_link,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn delete_by_articleid(&self, id: &i32) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_ARTICLE_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

#[async_trait]
impl BatchOperations<ArticleCreateModel,ArticleUpdateModel,ArticleModel> for PgArticleRepository {
    async fn insert_many(&self, items: Vec<ArticleCreateModel>) -> Result<Vec<ArticleModel>, DomainError> {
        let mut inserted_articles = Vec::new();

        for article in items {
            let inserted_article = self.insert(&article).await?;
            inserted_articles.push(inserted_article);
        }
    
        Ok(inserted_articles)
    }

    async fn update_many(&self, items: Vec<ArticleUpdateModel>) -> Result<Vec<ArticleModel>, DomainError> {
        let mut updated_articles = Vec::new();

        for article in items {
            let updated_article = self.update_by_extid(&article).await?;

            updated_articles.push(updated_article);
        }
    
        Ok(updated_articles)
    }
}

impl From<&Row> for ArticleModel {
    fn from(row: &Row) -> Self {
        Self {
            articleid: row.get("articleid"),
            extid: row.get("extid"),
            name: row.get("name"),
            description: row.get("description"),
            time_m: row.get("time_m"),
            publish_at: row.get("publish_at"),
            source: row.get("source"),
            link: row.get("link"),
            author: row.get("author"),
            highres_link: row.get("highres_link"),
            photo_link: row.get("photo_link"),
            thumb_link: row.get("thumb_link"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
