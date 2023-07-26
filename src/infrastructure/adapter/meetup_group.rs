use std::collections::HashSet;
use std::{collections::HashMap};
use std::sync::{Arc};
use tokio::sync::{Mutex};

use async_trait::async_trait;
use std::time::Duration;
use crate::domain::city::repository::CityRepository;
use crate::domain::error::DomainError;
use crate::domain::group::{adapter::GroupAdapter, model::{GroupCreateModel, ImageLinks}};
use serde::{Serialize, Deserialize};
use regex::Regex;
use tokio::time::sleep;
use reqwest::{Client, Response};

pub struct RateLimitedClient {
    client: Client,
    rate_limits: HashMap<String, RateLimit>,
}

struct RateLimit {
    limit: u32,
    remaining: u32,
    reset: u64,
}

impl RateLimitedClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            rate_limits: HashMap::new(),
        }
    }

    pub async fn get(&mut self, url: &str) -> Result<Response, DomainError> {
        self.check_rate_limit(url).await?;
        let resp = self.client.get(url).send().await?;
        self.update_rate_limit(url, &resp);
        Ok(resp)
    }

    async fn check_rate_limit(&self, url: &str) -> Result<(), DomainError> {
        if let Some(rate_limit) = self.rate_limits.get(url) {
            if rate_limit.remaining == 0 {
                let sleep_duration = Duration::from_secs(rate_limit.reset);
                sleep(sleep_duration).await;
            }
        }
        Ok(())
    }

    fn update_rate_limit(&mut self, url: &str, resp: &Response) {
        let limit = resp
            .headers()
            .get("x-ratelimit-limit")
            .and_then(|val| val.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let remaining = resp
            .headers()
            .get("x-ratelimit-remaining")
            .and_then(|val| val.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let reset = resp
            .headers()
            .get("x-ratelimit-reset")
            .and_then(|val| val.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        println!("{:?} {:?} {:?}",&limit,&remaining,&reset);
        if let (Some(limit), Some(remaining), Some(reset)) = (limit, remaining, reset) {
            self.rate_limits.insert(
                url.to_string(),
                RateLimit {
                    limit,
                    remaining,
                    reset,
                },
            );
        }
    }
}
#[derive(Serialize,Deserialize)]
struct MeetupOrganizer {
    id: i32,
    name: String,
    bio: String,
    photo: Option<MeetupGroupPhoto>,
}

#[derive(Serialize,Deserialize)]
struct MeetupGroupPhoto {
    id: i32,
    highres_link: String,
    photo_link: String,
    thumb_link: String,
}

#[derive(Serialize,Deserialize)]
struct MeetupAPIGroupResponse {
    id: i32,
    name: String,
    join_mode: String,
    lat: f32,
    lon: f32,
    status: String,
    link: String,
    description: String,
    urlname: String,
    state: String,
    visibility: String,
    city: String,
    members: i32,
    member_pay_fee: bool,
    wepay_fee_deprecated: bool,
    lang: String,
    timezone: String,
    who: String,
    key_photo: Option<MeetupGroupPhoto>,
    group_photo: Option<MeetupGroupPhoto>,
    organizer: MeetupOrganizer,
}

fn remove_html_tags(input: &str) -> String {
    let re = Regex::new(r"<[^>]+>").unwrap();
    re.replace_all(input, "").to_string()
}


pub struct MeetupGroupAdapter {
    client: Arc<Mutex<RateLimitedClient>>,
    city_repository: Arc<dyn CityRepository>,
}

impl MeetupGroupAdapter {
    pub fn new(
        client: Arc<Mutex<RateLimitedClient>>,
        city_repository: Arc<dyn CityRepository>,
    ) -> Self {
        Self {
            client,
            city_repository
        }
    }
}
#[async_trait]
impl GroupAdapter for MeetupGroupAdapter {
    async fn fetch(&self, names: Vec<String>) -> Result<Vec<GroupCreateModel>, DomainError> {
        let mut groups: Vec<GroupCreateModel> = Vec::new();

        for name in names {
            let url = format!("https://api.meetup.com/{}", name);
            println!("{}",&url);
            let response = self.client.lock().await.get(&url).await?;

            if response.status().is_success() {
                let resp: MeetupAPIGroupResponse = response.json().await?;
                let description = remove_html_tags(&resp.description);

                let photo_links = resp.key_photo.map(|photo| ImageLinks{
                    photo_link: Some(photo.photo_link),
                    thumb_link: Some(photo.thumb_link),
                    highres_link: Some(photo.highres_link),
                }).unwrap_or_else(|| ImageLinks{
                    photo_link: None,
                    thumb_link: None,
                    highres_link: None,
                });

                groups.push(
                    GroupCreateModel::new(
                        resp.id.to_string(),
                        resp.name.clone(),
                        description,
                        resp.urlname,
                        resp.status == "active",
                        resp.join_mode != "open",
                        resp.members,
                        resp.organizer.name,
                        photo_links,
                        resp.city,
                    ),
                );
            } else {
                let error_message = response.text().await?;
                return Err(DomainError::InternalServerError(format!("Failed to fetch from Meetup API: {}", error_message)));
            }
        }
        let city_extids: HashSet<_> = groups.iter().map(|group| group.cityextid.clone()).collect();
        let city_extids_vec: Vec<_> = city_extids.into_iter().collect();

        let city_models = self.city_repository.find_by_extids(city_extids_vec).await?;

        if let Some(city_models) = city_models {
            let cityid_by_extid: HashMap<String, i32> = city_models
                .into_iter()
                .map(|model| (model.name, model.cityid))
                .collect();
        
            for group in &mut groups {
                if let Some(cityid) = cityid_by_extid.get(&group.cityextid) {
                    group.set_cityid(*cityid);
                }
            }
        }

        Ok(groups)
    }
}
