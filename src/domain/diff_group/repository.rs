use async_trait::async_trait;


use crate::{api::lib::{BatchOperations, DiffOperations}};

use super::model::{DiffGroupModel};

#[async_trait]
pub trait DiffGroupRepository: BatchOperations<DiffGroupModel,DiffGroupModel,DiffGroupModel> + DiffOperations<DiffGroupModel> + Send + Sync {
}