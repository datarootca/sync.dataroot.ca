use chrono::{DateTime, Utc};

#[cfg(test)]
use crate::api::utils::random_number;
#[cfg(test)]
use crate::api::utils::random_string;

#[derive(Debug, Clone)]
pub struct StateCreateModel {
    pub name: String,
    pub symbol: String,
    pub extid: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl StateCreateModel {
    pub fn new(
        extid: String,
        name: String, 
        symbol: String,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            extid,
            name,
            symbol,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}

#[cfg(test)]
impl StateCreateModel {
    pub fn mock_default() -> Self {
        Self {
            name: random_string(10),
            symbol: random_string(2),
            extid: random_string(10),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateUpdateModel {
    pub name: String,
    pub symbol: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl StateUpdateModel {
    pub fn new(
        name: String, 
        symbol: String,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            name,
            symbol,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}
#[cfg(test)]
impl StateUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "Slovakia ".to_string(),
            symbol: "cs".to_string(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StateModel {
    pub stateid: i32,
    pub name: String,
    pub symbol: String,
    pub extid: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(test)]
impl StateModel {
    pub fn mock_default() -> Self {
        Self {
            stateid: random_number(),
            name: random_string(10),
            symbol: random_string(2),
            extid: random_string(10),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
