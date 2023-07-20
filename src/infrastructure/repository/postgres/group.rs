use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};

use crate::domain::{
    group::{
        model::{GroupCreateModel, GroupModel, GroupUpdateModel},
        repository::GroupRepository,
    },
    error::DomainError,
};

const QUERY_FIND_GROUP: &str = "
    select
        groupid,
        name,
        description,
        extid,
        slug,
        private,
        members,
        cityid,
        organizer,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link,
        active,
        count(1) over ()::OID as count
    from
        \"group\"";

const QUERY_FIND_GROUP_BY_ID: &str = "
    select
        groupid,
        name,
        description,
        extid,
        slug,
        private,
        members,
        cityid,
        organizer,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link,
        active,
        count(1) over ()::OID as count
    from
        \"group\"
    where 
        groupid = $1;";

const QUERY_INSERT_GROUP: &str = "
    insert into \"group\"(name,description,extid,slug,private,members,cityid,organizer,highres_link,photo_link,thumb_link,active)
    values
        ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
    returning
        groupid,
        name,
        description,
        extid,
        slug,
        private,
        members,
        cityid,
        organizer,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link, 
        active;";

const QUERY_UPDATE_GROUP_BY_ID: &str = "
    update
        \"group\" 
    set
        name=$2,
        description=$3,
        slug=$4,
        private=$5,
        members=$6,
        cityid=$7,
        organizer=$8,
        highres_link=$9,
        photo_link=$10,
        thumb_link=$11,
        active=$12,
        updated_at=now()
    where
        groupid = $1
    returning
        groupid,
        name,
        description,
        extid,
        slug,
        private,
        members,
        cityid,
        organizer,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link, 
        active;";

const QUERY_DELETE_GROUP_BY_ID: &str = "
    delete from
        \"group\" 
        where
            groupid = $1;";

pub struct PgGroupRepository {
    pool: Arc<Pool>,
}
impl PgGroupRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GroupRepository for PgGroupRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<GroupModel>, u32)>, DomainError> {
        let client = self.pool.get().await?;

        let mut queries: Vec<String> = vec![];
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(name) = name {
            queries.push(format!(
                "\"group\".name like '%' || ${} || '%'",
                params.len() + 1
            ));
            params.push(name);
        }

        let mut query = String::from(QUERY_FIND_GROUP);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let groups: Vec<GroupModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((groups, count)));
        }

        return Ok(None);
    }

    async fn find_by_groupid(&self, id: &i32) -> Result<Option<GroupModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_GROUP_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        group_create_model: &GroupCreateModel,
    ) -> Result<GroupModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_GROUP).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &group_create_model.name,
                    &group_create_model.description,
                    &group_create_model.extid,
                    &group_create_model.slug,
                    &group_create_model.private,
                    &group_create_model.members,
                    &group_create_model.cityid,
                    &group_create_model.organizer,
                    &group_create_model.highres_link,
                    &group_create_model.photo_link,
                    &group_create_model.thumb_link,
                    &group_create_model.active,
                ],
            )
            .await?;
        Ok(result.into())
    }

    async fn update_by_groupid(
        &self,
        groupid: &i32,
        group_update_model: &GroupUpdateModel,
    ) -> Result<GroupModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_UPDATE_GROUP_BY_ID).await?;
        let result = &client
        
            .query_one(
                &stmt,
                &[
                    groupid,
                    &group_update_model.name,
                    &group_update_model.description,
                    &group_update_model.slug,
                    &group_update_model.private,
                    &group_update_model.members,
                    &group_update_model.cityid,
                    &group_update_model.organizer,
                    &group_update_model.highres_link,
                    &group_update_model.photo_link,
                    &group_update_model.thumb_link,
                    &group_update_model.active,
                   
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn delete_by_groupid(&self, id: &i32) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_GROUP_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

impl From<&Row> for GroupModel {
    fn from(row: &Row) -> Self {
        Self {
            groupid:    row.get("groupid"),
            name:   row.get("name"),
            description:  row.get("description"),
            extid:  row.get("extid"),
            slug:   row.get("slug"),
            active: row.get("active"),
            private:    row.get("private"),
            members:    row.get("members"),
            cityid: row.get("cityid"),
            organizer:  row.get("organizer"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            highres_link:   row.get("highres_link"),
            photo_link: row.get("photo_link"),
            thumb_link: row.get("thumb_link"),
        }
    }
}
