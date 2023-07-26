#[cfg(test)]
use crate::api::utils::random_number;
#[cfg(test)]
use crate::api::utils::random_string;
use crate::domain::article::model::Guidable;
use crate::domain::article::model::Processable;
use serde::Serialize;
use sha2::{Sha256, Digest};

use chrono::{DateTime, Utc};
#[derive(Debug, Clone, Serialize)]
pub struct ImageLinks {
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroupCreateModel {
    pub name: String,
    pub description: String,
    pub slug: String,
    pub extid: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: Option<i32>,
    pub organizer: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub cityextid: String,
}
impl GroupCreateModel {
    pub fn new(
        extid: String,
        name: String,
        description: String,
        slug: String,
        active: bool,
        private: bool,
        members: i32,
        organizer: String,
        image: ImageLinks,
        cityextid: String,
    ) -> Self {
        Self {
            extid,
            name,
            description,
            slug,
            active,
            private,
            members,
            cityid: None,
            organizer,
            highres_link: image.highres_link,
            photo_link: image.photo_link,
            thumb_link: image.thumb_link,
            cityextid,
        }
    }

    pub fn set_cityid(&mut self,cityid: i32) -> &mut Self {
        self.cityid = Some(cityid);
        self
    }

    pub fn to_update(&self) -> GroupUpdateModel {
        GroupUpdateModel::new(
            self.extid.clone(),
            self.name.clone(),
            self.description.clone(),
            self.slug.clone(),
            self.active.clone(),
            self.private.clone(),
            self.members.clone(),
            self.cityid.clone(),
            self.organizer.clone(),
            ImageLinks { 
                highres_link: self.highres_link.clone(),
                photo_link: self.photo_link.clone(),
                thumb_link: self.thumb_link.clone(),
             }
        )
    }

}

impl Processable for GroupCreateModel {
    fn get_checksum(&self) -> String {
        let model_string = serde_json::to_string(&self)
        .expect("Failed to serialize GroupCreateModel to JSON");

        let mut hasher = Sha256::new();
        hasher.update(model_string);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl Guidable for GroupCreateModel {
    fn get_extid(&self) -> String {
        self.extid.clone()
    }

    fn set_extid(&mut self,extid: String){
        self.extid = extid;
    }
}

#[cfg(test)]
impl GroupCreateModel {
    pub fn mock_default() -> Self {
        Self {
            name: random_string(10),
            description: "The Big Group".to_string(),
            extid: random_string(10),
            slug: random_string(10),
            organizer: "organizer".to_string(),
            active: false,
            private: true,
            members: 100,
            cityid: Some(random_number()),
            cityextid: random_string(10),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupUpdateModel {
    pub extid: String,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: Option<i32>,
    pub organizer: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
}



impl GroupUpdateModel {
    pub fn new(
        extid: String,
        name: String,
        description: String,
        slug: String,
        active: bool,
        private: bool,
        members: i32,
        cityid: Option<i32>,
        organizer: String,
        image: ImageLinks,
    ) -> Self {
        Self {
            extid,
            name,
            description,
            slug,
            active,
            private,
            members,
            cityid,
            organizer,
            highres_link: image.highres_link,
            photo_link: image.photo_link,
            thumb_link: image.thumb_link,
        }
    }
}
#[cfg(test)]
impl GroupUpdateModel {
    pub fn mock_default() -> Self {
        Self {
            extid: random_string(10),
            name: random_string(10),
            description: "The Big Group".to_string(),
            slug: random_string(10),
            organizer: "organizer".to_string(),
            active: true,
            private: true,
            members: 100,
            cityid: Some(random_number()),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroupModel {
    pub groupid: i32,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub extid: String,
    pub active: bool,
    pub private: bool,
    pub members: i32,
    pub cityid: i32,
    pub organizer: String,
    pub highres_link: Option<String>,
    pub photo_link: Option<String>,
    pub thumb_link: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
impl GroupModel {
    pub fn mock_default() -> Self {
        Self {
            groupid: random_number(),
            name: random_string(10),
            description: "The Big Group".to_string(),
            extid: random_string(10),
            slug: random_string(10),
            organizer: "organizer"  .to_string(),
            active: true,
            private: true,
            members: 100,
            cityid: random_number(),
            highres_link: Some("".to_string()),
            photo_link: Some("".to_string()),
            thumb_link: Some("".to_string()),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
