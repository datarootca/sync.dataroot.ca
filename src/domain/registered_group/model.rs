use chrono::{DateTime, Utc};

#[cfg(test)]
use crate::api::utils::random_number;
#[cfg(test)]
use crate::api::utils::random_string;

#[derive(Debug, Clone)]
pub struct RegisteredGroupCreateModel {
    pub source: String,
    pub name: String,
}
impl RegisteredGroupCreateModel {
    pub fn new(
        name: String, 
        source: String,
    ) -> Self {
        Self {
            name,
            source,
        }
    }
}

#[cfg(test)]
impl RegisteredGroupCreateModel {
    pub fn mock_default() -> Self {
        Self {
            source: "medium".to_string(),
            name: random_string(10),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisteredGroupModel {
    pub registered_groupid: i32,
    pub source: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[cfg(test)]
impl RegisteredGroupModel {
    pub fn mock_default() -> Self {
        Self {
            registered_groupid: random_number(),
            source: "medium".to_string(),
            name: random_string(10),
            created_at: DateTime::default(),
            updated_at: Some(DateTime::default()),
        }
    }
}
