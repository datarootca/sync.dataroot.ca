use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;

use crate::domain::event::{adapter::EventAdapter, model::{EventCreateModel}};
use chrono::{NaiveDateTime};
use serde::{ Serialize,Deserialize};
use chrono::{ DateTime, Utc};
use chrono::format::ParseError;

use super::meetup_group::{RateLimitedClient};

#[derive(Serialize,Deserialize)]
struct Venue {
    id: i32,
    name: String,
    lat: f64,
    lon: f64,
    repinned: bool,
    address_1: String,
    city: String,
    country: String,
    localized_country_name: String,
    zip: String,
    state: String,
}

#[derive(Serialize,Deserialize)]
struct MeetupEvent {
    id: String,
    duration: i32,
    name: String,
    date_in_series_pattern: bool,
    status: String,
    time: u64,
    local_date: String,
    local_time: String,
    updated: i64,
    utc_offset: i32,
    waitlist_count: i32,
    yes_rsvp_count: i32,
    is_online_event: bool,
    link: String,
    eventType: String,
    description: String,
    visibility: String,
    member_pay_fee: bool,
    venue: Option<Venue>,
    rsvp_limit: Option<i32>,
}

#[derive(Serialize,Deserialize)]
struct MeetupGroupPhoto {
    id: i32,
    highres_link: String,
    photo_link: String,
    thumb_link: String,
}

fn convert_ms(unix_time: u64) -> Result<DateTime<Utc>, ParseError> {
    let seconds: u64 = unix_time / 1000; // Convert to seconds.
    let nanoseconds = (unix_time % 1000) * 1_000_000; // Convert remainder to nanoseconds.

    let naive_datetime = NaiveDateTime::from_timestamp(seconds as i64, nanoseconds as u32);
    Ok(DateTime::from_utc(naive_datetime, Utc))
}

pub struct MeetupEventAdapter {
    client: Arc<Mutex<RateLimitedClient>>,
}

impl MeetupEventAdapter {
    pub fn new(client: Arc<Mutex<RateLimitedClient>>) -> Self {
        Self {
            client,
        }
    }
}
#[async_trait]
impl EventAdapter for MeetupEventAdapter {
    async fn fetch(&self, group_names: Vec<String>) -> Result<Vec<EventCreateModel>, Box<dyn std::error::Error>> {
        let mut events: Vec<EventCreateModel> = Vec::new();

        for name in group_names {
            let url = format!("https://api.meetup.com/{}/events", &name);
            let response = self.client.lock().await.get(&url).await?;

            if response.status().is_success() {
                let resp: Vec<MeetupEvent> = response.json().await?;

                for meetup_event in resp {
                    let url = format!("https://api.meetup.com/{}/events/{}/photos", &name,&meetup_event.id);
                    let photo_response = self.client.lock().await.get(&url).await?;

                    let photo_album: Vec<MeetupGroupPhoto> = if photo_response.status().is_success() {
                        photo_response.json().await?
                    } else {
                        vec![] // or provide a default
                    };

                    let (highres_link, thumb_link, photo_link) = if let Some(first_photo) = photo_album.first() {
                        (first_photo.highres_link.clone(), first_photo.thumb_link.clone(), first_photo.photo_link.clone())
                    } else {
                        (String::from(""), String::from(""), String::from("")) // or provide a default
                    };

                    events.push(
                        EventCreateModel::new(
                            meetup_event.name,
                            meetup_event.description,
                            meetup_event.venue.unwrap().name, 
                            1,
                            format!("m{}", meetup_event.id), 
                            meetup_event.link,
                            meetup_event.eventType == "PHYSICAL",
                            meetup_event.is_online_event,
                            meetup_event.duration, 
                            meetup_event.waitlist_count, 
                            meetup_event.yes_rsvp_count, 
                            meetup_event.member_pay_fee, 
                            meetup_event.rsvp_limit, 
                            convert_ms(meetup_event.time).expect("Failed parse date"), 
                            Some(highres_link), 
                            Some(photo_link),
                            Some(thumb_link),
                        ),
                    );
                }

            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to fetch from Meetup API: {:?}", response.text().await?),
                )));
            }
        }

        Ok(events)
    }
}
