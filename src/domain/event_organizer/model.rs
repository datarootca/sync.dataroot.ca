use chrono::{DateTime, Utc};

#[cfg(test)]
use crate::api::utils::random_number;
#[cfg(test)]
use crate::api::utils::random_string;
use crate::domain::article::model::Guidable;
use crate::domain::article::model::Processable;

#[derive(Debug, Clone)]
pub struct EventCreateModel {
    pub name: String,
    pub description: String,
    pub location: String,
    pub extid: String,
    pub groupid: i32,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: Option<i32>,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub last_update: String,
}
impl EventCreateModel {
    pub fn new(
        name: String, 
        description: String,
        location: String,
        groupid: i32,
        extid: String,
        link: String,
        in_person: bool,
        is_online: bool,
        duration: i32,
        waitlist_count: i32,
        yes_rsvp_count: i32,
        fee: bool,
        rsvp_limit: Option<i32>,
        time: DateTime<Utc>,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
        last_update: String,
    ) -> Self {
        Self {
            name,
            description,
            location, 
            groupid,
            extid, 
            in_person, 
            is_online, 
            time, 
            duration, 
            link, 
            waitlist_count, 
            fee, 
            yes_rsvp_count, 
            rsvp_limit, 
            highres_link, 
            photo_link,
            thumb_link,
            last_update,
        }
    }

    pub fn to_update(&self) -> EventUpdateModel {
        EventUpdateModel::new( 
            self.extid.clone(),
            self.name.clone(),
            self.description.clone(),
            self.location.clone(),
            self.groupid.clone(),
            self.link.clone(),
            self.in_person.clone(),
            self.is_online.clone(),
            self.duration.clone(),
            self.waitlist_count.clone(),
            self.yes_rsvp_count.clone(),
            self.fee.clone(),
            self.rsvp_limit.clone(),
            self.time.clone(),
            self.highres_link.clone(),
            self.photo_link.clone(),
            self.thumb_link.clone(),
        )
    }   
}

impl Processable for EventCreateModel {
    fn get_checksum(&self) -> String {
        self.last_update.clone()
    }
}

impl Guidable for EventCreateModel {
    fn get_extid(&self) -> String {
        self.extid.clone()
    }

    fn set_extid(&mut self,extid: String){
        self.extid = extid;
    }
}

#[cfg(test)]
impl EventCreateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "event".to_string(),
            description: "The Big Event".to_string(),
            location: "boulvar".to_string(),
            last_update: "boulvar".to_string(),
            groupid: random_number(),
            extid: random_string(10),
            in_person: true,
            is_online: true,
            time: DateTime::default(),
            duration: 5,
            link: random_string(10),
            waitlist_count: 5,
            fee: false,
            yes_rsvp_count: 5,
            rsvp_limit: Some(5),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug,Clone)]
pub struct EventUpdateModel {
    pub extid: String,
    pub name: String,
    pub description: String,
    pub location: String,
    pub groupid: i32,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: Option<i32>,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl EventUpdateModel {
    pub fn new(
        extid: String,
        name: String,
        description: String,
        location: String,
        groupid: i32,
        link: String,
        in_person: bool,
        is_online: bool,
        duration: i32,
        waitlist_count: i32,
        yes_rsvp_count: i32,
        fee: bool,
        rsvp_limit: Option<i32>,
        time: DateTime<Utc>,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            extid,
            name,
            description,
            location, 
            groupid,
            in_person, 
            is_online, 
            time, 
            duration, 
            link, 
            waitlist_count, 
            fee, 
            yes_rsvp_count, 
            rsvp_limit, 
            highres_link, 
            photo_link,
            thumb_link,
        }
    }
}
#[cfg(test)]
impl EventUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            extid: random_string(10),
            name: "Event".to_string(),
            description: "The Big Event".to_string(),
            location: "boulvar".to_string(),
            groupid: random_number(),
            in_person: true,
            is_online: true,
            time: DateTime::default(),
            duration: 5,
            link: random_string(10),
            waitlist_count: 5,
            fee: false,
            yes_rsvp_count: 5,
            rsvp_limit: Some(5),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventModel {
    pub eventid: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    pub extid: String,
    pub groupid: i32,
    pub in_person: bool,
    pub is_online: bool,
    pub time: DateTime<Utc>,
    pub duration: i32,
    pub link: String,
    pub waitlist_count: i32,
    pub fee: bool,
    pub yes_rsvp_count: i32,
    pub rsvp_limit: Option<i32>,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(test)]
impl EventModel {
    pub fn mock_default() -> Self {
        Self {
            eventid: random_number(),
            groupid: random_number(),
            name: "Event".to_string(),
            description: "The Big Event".to_string(),
            location: "boulvar".to_string(),
            extid: random_string(10),
            in_person: true,
            is_online: true,
            time: DateTime::default(),
            duration: 5,
            link: random_string(10),
            waitlist_count: 5,
            fee: false,
            yes_rsvp_count: 5,
            rsvp_limit: Some(5),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
