use chrono::{DateTime, Utc};


pub trait Processable {
    fn get_checksum(&self) -> String;
}

pub trait Guidable {
    fn get_extid(&self) -> String;
    fn set_extid(&mut self, extid: String);
}

#[cfg(test)]
use crate::api::utils::{
    random_string,
    random_number,
};

#[derive(Debug, Clone)]
pub struct ArticleCreateModel {
    pub extid: String,
    pub name: String,
    pub description: Option<String>,
    pub time_m: i32,
    pub source: String,
    pub link: String,
    pub author: String,
    pub publish_at: DateTime<Utc>,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub last_update: String,
}
impl ArticleCreateModel {
    pub fn new(
        name: String,
        description: Option<String>,
        extid: String,
        time_m: i32,
        source: String,
        link: String,
        author: String,
        publish_at: DateTime<Utc>,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
        last_update: String,
    ) -> Self {
        Self {
            name,
            description,
            extid,
            time_m,
            source,
            author,
            link,
            publish_at,
            highres_link,
            photo_link,
            thumb_link,
            last_update,
        }
    }
    
    pub fn to_update(&self) -> ArticleUpdateModel {
        ArticleUpdateModel::new(
            self.extid.clone(),
            self.name.clone(),
            self.description.clone(),
            self.time_m.clone(),
            self.link.clone(),
            self.author.clone(),
            self.publish_at.clone(),
            self.highres_link.clone(),
            self.photo_link.clone(),
            self.thumb_link.clone(),
        )
    }
}

impl Processable for ArticleCreateModel {
    fn get_checksum(&self) -> String {
        self.last_update.clone()
    }
}

impl Guidable for ArticleCreateModel {
    fn get_extid(&self) -> String {
        self.extid.clone()
    }

    fn set_extid(&mut self,extid: String){
        self.extid = extid;
    }
}

#[cfg(test)]
impl ArticleCreateModel {
    pub fn mock_default() -> Self {
        Self {
            extid: random_string(10),
            name: "article".to_string(),
            link: random_string(10),
            description: Some("The famous article".to_string()),
            time_m: 5,
            source: "source".to_string(),
            author: "author".to_string(),
            publish_at: DateTime::default(),
            highres_link: Some("The img".to_string()),
            photo_link: Some("The img".to_string()),
            thumb_link: Some("The img".to_string()),
            last_update: "test".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArticleUpdateModel {
    pub extid: String,
    pub name: String,
    pub description: Option<String>,
    pub time_m: i32,
    pub link: String,
    pub author: String,
    pub publish_at: DateTime<Utc>,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}
impl ArticleUpdateModel {
    pub fn new(
        extid: String,
        name: String,
        description: Option<String>,
        time_m: i32,
        link: String,
        author: String,
        publish_at: DateTime<Utc>,
        highres_link: Option<String>,
        photo_link: Option<String>,
        thumb_link: Option<String>,
    ) -> Self {
        Self {
            extid,
            name,
            description,
            time_m,
            link,
            author,
            publish_at,
            highres_link,
            photo_link,
            thumb_link,
        }
    }
}
#[cfg(test)]
impl ArticleUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            name: "article".to_string(),
            description: Some("The famous article".to_string()),
            time_m: 5,
            extid: random_string(10),
            link: random_string(10),
            author: "author".to_string(),
            publish_at: DateTime::default(),
            highres_link: Some("The img".to_string()),
            photo_link: Some("The img".to_string()),
            thumb_link: Some("The img".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArticleModel {
    pub articleid: i32,
    pub extid: String,
    pub name: String,
    pub description: Option<String>,
    pub time_m: i32,
    pub link: String,
    pub source: String,
    pub author: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub publish_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
impl ArticleModel {
    pub fn mock_default() -> Self {
        Self {
            articleid: random_number(),
            extid: random_string(10),
            name: "article".to_string(),
            description: Some("The famous article".to_string()),
            link: random_string(10),
            time_m: 5,
            source: "source".to_string(),
            author: "author".to_string(),
            highres_link: Some("highres_link".to_string()),
            photo_link: Some("photo_link".to_string()),
            thumb_link: Some("thumb_link".to_string()),
            publish_at: DateTime::default(),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
