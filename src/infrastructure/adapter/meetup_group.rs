use std::{collections::HashMap};
use std::sync::{Arc};
use tokio::sync::{Mutex};

use async_trait::async_trait;
use std::time::Duration;
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

    pub async fn get(&mut self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        self.check_rate_limit(url).await?;
        let resp = self.client.get(url).send().await?;
        self.update_rate_limit(url, &resp);
        Ok(resp)
    }

    async fn check_rate_limit(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
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
            .get("X-RateLimit-Limit")
            .and_then(|val| val.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let remaining = resp
            .headers()
            .get("X-RateLimit-Remaining")
            .and_then(|val| val.to_str().ok())
            .and_then(|s| s.parse::<u32>().ok());

        let reset = resp
            .headers()
            .get("X-RateLimit-Reset")
            .and_then(|val| val.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

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
    photo: MeetupGroupPhoto,
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
    key_photo: MeetupGroupPhoto,
    group_photo: MeetupGroupPhoto,
    organizer: MeetupOrganizer,
}

fn remove_html_tags(input: &str) -> String {
    let re = Regex::new(r"<[^>]+>").unwrap();
    re.replace_all(input, "").to_string()
}


pub struct MeetupGroupAdapter {
    client: Arc<Mutex<RateLimitedClient>>,
}

impl MeetupGroupAdapter {
    pub fn new(client: Arc<Mutex<RateLimitedClient>>) -> Self {
        Self {
            client,
        }
    }
}
#[async_trait]
impl GroupAdapter for MeetupGroupAdapter {
    async fn fetch(&self, names: Vec<String>) -> Result<Vec<GroupCreateModel>, Box<dyn std::error::Error>> {
        let mut groups: Vec<GroupCreateModel> = Vec::new();

        for name in names {
            let url = format!("https://api.meetup.com/{}", name);
            let response = self.client.lock().await.get(&url).await?;


            if response.status().is_success() {
                let resp: MeetupAPIGroupResponse = response.json().await?;
                let description = remove_html_tags(&resp.description);
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
                        ImageLinks{
                            photo_link: Some(resp.key_photo.photo_link),
                            thumb_link: Some(resp.key_photo.thumb_link),
                            highres_link: Some(resp.key_photo.highres_link),
                        }
                    ),
                );
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to fetch from Meetup API: {:?}", response.text().await?),
                )));
            }
        }

        Ok(groups)
    }
}
