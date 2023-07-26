use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Pool;

use tokio_postgres::{types::ToSql, Row};

use crate::{domain::{
    event::{
        model::{EventCreateModel, EventModel, EventUpdateModel},
        repository::EventRepository,
    },
    error::DomainError,
}, api::lib::BatchOperations};

const QUERY_FIND_EVENT: &str = "
    select
        e.eventid,
        e.name,
        e.description,
        e.extid,
        e.location,
        e.groupid,
        e.in_person,
        e.time,
        e.duration,
        e.link,
        e.waitlist_count,
        e.is_online,
        e.yes_rsvp_count,
        e.fee,
        e.created_at,
        e.updated_at,
        e.highres_link,
        e.photo_link,
        e.thumb_link,
        e.rsvp_limit,
        count(1) over ()::OID as count
    from
        event e";

const QUERY_FIND_EVENT_BY_ID: &str = "
    select
        e.eventid,
        e.name,
        e.description,
        e.extid,
        e.location,
        e.groupid,
        e.in_person,
        e.time,
        e.duration,
        e.link,
        e.waitlist_count,
        e.is_online,
        e.yes_rsvp_count,
        e.fee,
        e.created_at,
        e.updated_at,
        e.highres_link,
        e.rsvp_limit,
        e.photo_link,
        e.thumb_link,
        count(1) over ()::OID as count
    from
        event e
    where 
        eventid = $1;";

const QUERY_FIND_EVENT_BY_EXTID: &str = "
        select
            e.eventid,
            e.name,
            e.description,
            e.extid,
            e.location,
            e.groupid,
            e.in_person,
            e.time,
            e.duration,
            e.link,
            e.waitlist_count,
            e.is_online,
            e.yes_rsvp_count,
            e.fee,
            e.created_at,
            e.updated_at,
            e.highres_link,
            e.rsvp_limit,
            e.photo_link,
            e.thumb_link,
            count(1) over ()::OID as count
        from
            event e
        where 
            extid = $1;";

const QUERY_INSERT_EVENT: &str = "
    insert into event(name,description,extid,location,groupid,in_person,time,duration,link,waitlist_count,is_online,yes_rsvp_count,fee,highres_link,photo_link,thumb_link,rsvp_limit)
    values
        ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17)
    returning
        eventid,
        name,
        description,
        extid,
        location,
        groupid,
        in_person,
        time,
        duration,
        link,
        waitlist_count,
        is_online,
        yes_rsvp_count,
        rsvp_limit,
        fee,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link;";

const QUERY_UPDATE_EVENT_BY_EXTID: &str = "
    update
        event 
    set
        name=$2,
        description=$3,
        location=$4,
        groupid=$5,
        in_person=$6,
        time=$7,
        duration=$8,
        link=$9,
        waitlist_count=$10,
        is_online=$11,
        yes_rsvp_count=$12,
        fee=$13,
        highres_link=$14,
        photo_link=$15,
        thumb_link=$16,
        rsvp_limit=$17,
        updated_at=now()
    where
        extid = $1
    returning
        eventid,
        name,
        description,
        extid,
        location,
        groupid,
        in_person,
        time,
        duration,
        link,
        waitlist_count,
        is_online,
        yes_rsvp_count,
        rsvp_limit,
        fee,
        created_at,
        updated_at,
        highres_link,
        photo_link,
        thumb_link;";

const QUERY_DELETE_EVENT_BY_ID: &str = "
            delete from
                event 
            where
                eventid = $1;";

pub struct PgEventRepository {
    pool: Arc<Pool>,
}
impl PgEventRepository {
    pub fn new(pool: Arc<Pool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventRepository for PgEventRepository {
    async fn find(
        &self,
        name: &Option<String>,
        page: &u32,
        page_size: &u32,
    ) -> Result<Option<(Vec<EventModel>, u32)>, DomainError> {
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

        let mut query = String::from(QUERY_FIND_EVENT);
        if !queries.is_empty() {
            query = format!("{} where {}", query, queries.join(" and "));
        }

        let offset = page_size * (page - 1);
        query = format!("{query} limit {page_size} offset {offset}");

        let stmt = client.prepare(&query).await?;
        let result = client.query(&stmt, &params[..]).await?;

        if !result.is_empty() {
            let count: u32 = result.first().unwrap().get("count");

            let events: Vec<EventModel> = result.iter().map(|row| row.into()).collect();

            return Ok(Some((events, count)));
        }

        return Ok(None);
    }

    async fn find_by_eventid(&self, id: &i32) -> Result<Option<EventModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_EVENT_BY_ID).await?;

        if let Some(result) = client.query_opt(&stmt, &[id]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn find_by_extid(&self, extid: String) -> Result<Option<EventModel>, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_FIND_EVENT_BY_EXTID).await?;

        if let Some(result) = client.query_opt(&stmt, &[&extid]).await? {
            return Ok(Some((&result).into()));
        }

        return Ok(None);
    }

    async fn insert(
        &self,
        event_create_model: &EventCreateModel,
    ) -> Result<EventModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_INSERT_EVENT).await?;
        let result = &client
            .query_one(
                &stmt,
                &[
                    &event_create_model.name,
                    &event_create_model.description,
                    &event_create_model.extid,
                    &event_create_model.location,
                    &event_create_model.groupid,
                    &event_create_model.in_person,
                    &event_create_model.time,
                    &event_create_model.duration,
                    &event_create_model.link,
                    &event_create_model.waitlist_count,
                    &event_create_model.is_online,
                    &event_create_model.yes_rsvp_count,
                    &event_create_model.fee,
                    &event_create_model.highres_link,
                    &event_create_model.photo_link,
                    &event_create_model.thumb_link,
                    &event_create_model.rsvp_limit,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn update_by_extid(
        &self,
        event_update_model: &EventUpdateModel,
    ) -> Result<EventModel, DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_UPDATE_EVENT_BY_EXTID).await?;
        let result = &client
        
            .query_one(
                &stmt,
                &[
                    &event_update_model.extid,
                    &event_update_model.name,
                    &event_update_model.description,
                    &event_update_model.location,
                    &event_update_model.groupid,
                    &event_update_model.in_person,
                    &event_update_model.time,
                    &event_update_model.duration,
                    &event_update_model.link,
                    &event_update_model.waitlist_count,
                    &event_update_model.is_online,
                    &event_update_model.yes_rsvp_count,
                    &event_update_model.fee,
                    &event_update_model.highres_link,
                    &event_update_model.photo_link,
                    &event_update_model.thumb_link,
                    &event_update_model.rsvp_limit,
                ],
            )
            .await?;

        Ok(result.into())
    }

    async fn delete_by_eventid(&self, id: &i32) -> Result<(), DomainError> {
        let client = self.pool.get().await?;
        let stmt = client.prepare(QUERY_DELETE_EVENT_BY_ID).await?;
        client.execute(&stmt, &[id]).await?;
        Ok(())
    }
}

#[async_trait]
impl BatchOperations<EventCreateModel,EventUpdateModel,EventModel> for PgEventRepository {
    async fn insert_many(&self, items: Vec<EventCreateModel>) -> Result<Vec<EventModel>, DomainError> {
        let mut inserted_items = Vec::new();

        for item in items {
            let inserted_article = self.insert(&item).await?;
            inserted_items.push(inserted_article);
        }
    
        Ok(inserted_items)
    }

    async fn update_many(&self, items: Vec<EventUpdateModel>) -> Result<Vec<EventModel>, DomainError> {
        let mut updated_items = Vec::new();

        for item in items {
            let updated_model = self.update_by_extid(&item).await?;

            updated_items.push(updated_model);
        }
    
        Ok(updated_items)
    }
}

impl From<&Row> for EventModel {
    fn from(row: &Row) -> Self {
        Self {
            eventid: row.get("eventid"),
            name: row.get("name"),
            description: row.get("description"),
            extid: row.get("extid"),
            location: row.get("location"),
            groupid: row.get("groupid"),
            in_person: row.get("in_person"),
            time: row.get("time"),
            duration: row.get("duration"),
            link: row.get("link"),
            waitlist_count: row.get("waitlist_count"),
            is_online: row.get("is_online"),
            yes_rsvp_count: row.get("yes_rsvp_count"),
            fee: row.get("fee"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            highres_link: row.get("highres_link"),
            photo_link: row.get("photo_link"),
            thumb_link: row.get("thumb_link"),
            rsvp_limit: row.get("rsvp_limit"),
            
        }
    }
}
