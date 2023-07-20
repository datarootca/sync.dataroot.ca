use chrono::{DateTime, Utc};

#[cfg(test)]
use crate::api::utils::random_number;
#[cfg(test)]
use crate::api::utils::random_string;

#[derive(Debug, Clone)]
pub struct CityCreateModel {
    pub name: String,
    pub slug: String,
    pub stateid: i32,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub extid: String,
}
impl CityCreateModel {
    pub fn new(
        name: String, 
        slug: String,
        stateid: i32,
        extid: String,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link:Option<String>,
    ) -> Self {
        Self {
            name,
            slug,
            stateid,
            extid,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}

#[cfg(test)]
impl CityCreateModel {
    pub fn mock_default() -> Self {
        Self {
            stateid: random_number(),
            name: random_string(10),
            slug: random_string(10),
            extid: random_string(10),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CityUpdateModel {
    pub name: String,
    pub slug: String,
    pub stateid: i32,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl CityUpdateModel {
    pub fn new(
        name: String, 
        slug: String,
        stateid: i32,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link:Option<String>,
    ) -> Self {
        Self {
            name,
            slug,
            stateid,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}
#[cfg(test)]
impl CityUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "Ohio 1".to_string(),
            slug: "ohio 1".to_string(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            stateid: random_number(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CityModel {
    pub cityid: i32,
    pub name: String,
    pub slug: String,
    pub stateid: i32,
    pub extid: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(test)]
impl CityModel {
    pub fn mock_default() -> Self {
        Self {
            cityid: random_number(),
            name: random_string(10),
            slug: random_string(10),
            extid: random_string(10),
            stateid: random_number(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
