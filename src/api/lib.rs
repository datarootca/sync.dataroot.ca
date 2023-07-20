use deadpool_postgres::Pool;
use std::{error::Error, sync::Arc};

use crate::{
    api::{
        config,
    },
    domain::{state::repository::StateRepository, city::repository::CityRepository, article::{repository::ArticleRepository, adapter::ArticleAdapter, model::ArticleCreateModel}, group::repository::GroupRepository, event::repository::EventRepository},
    infrastructure::{repository::postgres::{state::PgStateRepository, postgres, city::PgCityRepository, article::PgArticleRepository, event::PgEventRepository, group::PgGroupRepository}, adapter::medium_article::MediumArticleAdapter},
};

pub struct AppState {
    pub state_repository: Arc<dyn StateRepository>,
    pub city_repository: Arc<dyn CityRepository>,
    pub article_repository: Arc<dyn ArticleRepository>,
    pub group_repository: Arc<dyn GroupRepository>,
    pub event_repository: Arc<dyn EventRepository>,
}

pub struct ArticleService<A: ArticleAdapter> {
    adapter: A,
}

impl<A: ArticleAdapter> ArticleService<A> {
    pub fn new(adapter: A) -> Self {
        Self { adapter }
    }

    pub async fn process_articles(&self, author: String) -> Result<Vec<ArticleCreateModel>, Box<dyn std::error::Error>> {
        self.adapter.fetch(author).await
    }
}

pub async fn run(pg_pool: Arc<Pool>) -> Result<(), Box<dyn Error>> {
    postgres::run_migrations().await?;


    let repositories = AppState {
        state_repository: Arc::new(PgStateRepository::new(pg_pool.clone())),
        city_repository: Arc::new(PgCityRepository::new(pg_pool.clone())),
        article_repository: Arc::new(PgArticleRepository::new(pg_pool.clone())),
        group_repository: Arc::new(PgGroupRepository::new(pg_pool.clone())),
        event_repository: Arc::new(PgEventRepository::new(pg_pool.clone())),
    };

    let adapter = MediumArticleAdapter::new();
    let service = ArticleService::new(adapter);

    match service.process_articles("tkudlicka".to_string()).await {
        Ok(articles) => {
            // Do something with articles
            for article in articles {
                println!("{:?}", article);
            }
        }
        Err(e) => eprintln!("Error fetching articles: {}", e),
    }


    Ok(())
}
