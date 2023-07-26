#[cfg(test)]
use crate::api::utils::{
    random_string,
};
use crate::domain::article::model::{Guidable, Processable};


#[derive(Debug, Clone)]
pub struct DiffArticleModel {
    pub key: String,
    pub value: String,
}
impl DiffArticleModel {
    pub fn  new(
        key: String,
        value: String,
    ) -> Self {
        Self {
            key,
            value,
        }
    }
}

impl Guidable for DiffArticleModel {
    fn get_extid(&self) -> String {
        self.key.clone()
    }

    fn set_extid(&mut self,key: String) {
        self.key = key;
    }
}
impl Processable for DiffArticleModel {
    fn get_checksum(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
impl DiffArticleModel {
    pub fn mock_default() -> Self {
        Self {
            key: random_string(10),
            value: "article".to_string(),
        }
    }
}