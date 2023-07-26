use async_trait::async_trait;


use crate::{api::lib::{BatchOperations, DiffOperations}};

use super::model::{DiffArticleModel};

#[async_trait]
pub trait DiffArticleRepository: BatchOperations<DiffArticleModel,DiffArticleModel,DiffArticleModel> + DiffOperations<DiffArticleModel> + Send + Sync {
}