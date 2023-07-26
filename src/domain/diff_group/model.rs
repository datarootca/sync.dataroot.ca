#[cfg(test)]
use crate::api::utils::{
    random_string,
};
use crate::domain::article::model::{Guidable, Processable};


#[derive(Debug, Clone)]
pub struct DiffGroupModel {
    pub key: String,
    pub value: String,
}
impl DiffGroupModel {
    pub fn new(
        key: String,
        value: String,
    ) -> Self {
        Self {
            key,
            value,
        }
    }
}

impl Guidable for DiffGroupModel {
    fn get_extid(&self) -> String {
        self.key.clone()
    }

    fn set_extid(&mut self,key: String) {
        self.key = key;
    }
}
impl Processable for DiffGroupModel {
    fn get_checksum(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
impl DiffGroupModel {
    pub fn mock_default() -> Self {
        Self {
            key: random_string(10),
            value: "group".to_string(),
        }
    }
}
