use async_trait::async_trait;


use crate::{api::lib::{BatchOperations, DiffOperations}};

use super::model::{DiffEventModel};

#[async_trait]
pub trait DiffEventRepository: BatchOperations<DiffEventModel,DiffEventModel,DiffEventModel> + DiffOperations<DiffEventModel> + Send + Sync {
}