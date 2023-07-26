use async_trait::async_trait;
use chrono::{Local, Timelike};
use deadpool_postgres::Pool;
use tokio::{time::{interval_at, Duration, Instant}, sync::Mutex};
use std::sync::Arc;
use std::error::Error;

use crate::{domain::{error::DomainError, article::model::{Processable, Guidable}}, infrastructure::{repository::{
    postgres::{group::PgGroupRepository, article::PgArticleRepository, city::PgCityRepository, event::PgEventRepository,postgres::{run_migrations as prod_migrations}}, sync::{postgres::{run_migrations as sync_migrations},diff_article::PgDiffArticleRepository, registered_author::PgRegisteredAuthorRepository, diff_group::PgDiffGroupRepository, registered_group::PgRegisteredGroupRepository, diff_event::PgDiffEventRepository}}, adapter::{meetup_group::{RateLimitedClient, MeetupGroupAdapter}, medium_article::{ MediumArticleAdapter}, meetup_event::MeetupEventAdapter}}};

// The services
use super::services::{article_sync, group_sync, event_sync};

pub struct Scheduler {
    pg_pool: Arc<Pool>,
    sync_pool: Arc<Pool>,
}

#[async_trait]
pub trait BatchOperations<T,D,G> {
    async fn insert_many(&self, items: Vec<T>) -> Result<Vec<G>, DomainError>;
    async fn update_many(&self, items: Vec<D>) -> Result<Vec<G>, DomainError>;
}

#[async_trait]
pub trait DiffOperations<T: Processable + Guidable> {
    async fn find_by_extids(&self, extids: Vec<String>) -> Result<Vec<T>, DomainError>;
}


impl Scheduler {
    pub fn new(pg_pool: Arc<Pool>, sync_pool: Arc<Pool>) -> Scheduler {
        Scheduler { pg_pool, sync_pool }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        prod_migrations().await?;
        sync_migrations().await?;
        // Define the times of day to run each task
        let article_hour = 1; // Run at 1 AM
        let group_hour = 2; // Run at 2 AM
        let event_hour = 3; // Run at 3 AM

        // Calculate the duration until the next occurrence of each time
        let article_duration_until = self.duration_until_hour(article_hour);
        let group_duration_until = self.duration_until_hour(group_hour);
        let event_duration_until = self.duration_until_hour(event_hour);

        // Calculate the Instant for each start time
        let start_instant = Instant::now();
        let article_start_time = start_instant + article_duration_until;
        let group_start_time = start_instant + group_duration_until;
        let event_start_time = start_instant + event_duration_until;

        // Set up the intervals to run every 24 hours
        let article_interval = interval_at(article_start_time, Duration::from_secs(24 * 3600));
        let group_interval = interval_at(group_start_time, Duration::from_secs(24 * 3600));
        let event_interval = interval_at(event_start_time, Duration::from_secs(24 * 3600));

        let article_repository = Arc::new(PgArticleRepository::new(self.pg_pool.clone()));
        let diff_article_repository = Arc::new(PgDiffArticleRepository::new(self.sync_pool.clone()));
        let registered_author_repository = Arc::new(PgRegisteredAuthorRepository::new(self.sync_pool.clone()));

        // The tasks
        let mut article_sync_task = article_sync::ArticleSync::new(
            MediumArticleAdapter::new(),
            article_repository.clone(), 
            diff_article_repository.clone(), 
            registered_author_repository.clone(),
            article_interval
        );

        let rate_limited_client = Arc::new(Mutex::new(RateLimitedClient::new()));
        let city_repository = Arc::new(PgCityRepository::new(self.pg_pool.clone()));


        let group_repository = Arc::new(PgGroupRepository::new(self.pg_pool.clone()));
        let diff_group_repository = Arc::new(PgDiffGroupRepository::new(self.sync_pool.clone()));
        let registered_group_repository = Arc::new(PgRegisteredGroupRepository::new(self.sync_pool.clone()));

        let rate_limited_client_clone = rate_limited_client.clone();

        let mut group_sync_task = group_sync::GroupSync::new(
            MeetupGroupAdapter::new(rate_limited_client_clone,city_repository.clone()),
            group_repository.clone(), 
            diff_group_repository.clone(), 
            registered_group_repository.clone(),
            group_interval
        );

        let event_repository = Arc::new(PgEventRepository::new(self.pg_pool.clone()));
        let diff_event_repository = Arc::new(PgDiffEventRepository::new(self.sync_pool.clone()));

        let rate_limited_client_clone = rate_limited_client.clone();

        let mut event_sync_task = event_sync::EventSync::new(
            MeetupEventAdapter::new(rate_limited_client_clone),
            event_repository.clone(), 
            diff_event_repository.clone(), 
            group_repository.clone(), 
            event_interval,
        );

       

        tokio::try_join!(
            article_sync_task.start(),
            group_sync_task.start(),
            event_sync_task.start()
        )?;

        Ok(())
    }

    fn duration_until_hour(&self, hour: u32) -> Duration {
        let now = Local::now();
        let next = now
            .with_hour(hour).unwrap()
            .with_minute(0).unwrap()
            .with_second(0).unwrap()
            .with_nanosecond(0).unwrap();
    
        let next = if next > now {
            next
        } else {
            next + chrono::Duration::days(1)
        };
    
        let duration_until_next = next.signed_duration_since(now);
        let duration_secs = duration_until_next.num_seconds() as u64;
        Duration::from_secs(duration_secs)
        
    }
}