use std::sync::Arc;

use tokio::time::Interval;

use crate::{domain::{group::{adapter::GroupAdapter, repository::GroupRepository, model::{GroupCreateModel, GroupModel, GroupUpdateModel}}, diff_group::{repository::DiffGroupRepository, model::DiffGroupModel}, error::DomainError,article::model::{Processable, Guidable}, registered_group::{repository::RegisteredGroupRepository, model::RegisteredGroupModel}}, api::lib::{BatchOperations, DiffOperations}};



pub struct GroupSync<A: GroupAdapter> {
    adapter: A,
    repository: Arc<dyn GroupRepository>,
    diff_repository: Arc<dyn DiffGroupRepository>,
    registered_group_repository: Arc<dyn RegisteredGroupRepository>,
    interval: Interval,
}

impl<A: GroupAdapter> GroupSync<A> {
    pub fn new(
        adapter: A,
        repository: Arc<dyn GroupRepository>,
        diff_repository: Arc<dyn DiffGroupRepository>,
        registered_group_repository: Arc<dyn RegisteredGroupRepository>,
        interval: Interval
    ) -> GroupSync<A> {
        GroupSync {
            adapter,
            repository,
            diff_repository,
            registered_group_repository,
            interval,
        }
    }

    pub async fn start(&mut self) -> Result<(), DomainError> {
        loop {
            // Wait until the next interval
            self.interval.tick().await;

            // Perform the sync
            self.sync().await?;
        }
    }

    async fn sync(&self) -> Result<(), DomainError> {
        println!("Syncing groups...");
    
        let page_size = 100;
        let mut current_page = 1;
        let mut total = 0;
    
        loop {
            let registred_groups_data = self.get_registred_groups(current_page, page_size).await?;
            if let Some((registred_groups, total_registred_groups)) = registred_groups_data {
                total += registred_groups.len();
                self.process_all_groups(registred_groups).await?;
                if self.is_end_of_registred_groups(total as u32, total_registred_groups) {
                    break;
                } else {
                    current_page += 1;
                }
            } else {
                break;
            }
        }
    
        Ok(())
    }

    async fn process_groups(&self, group_names: Vec<String>) -> Result<(), DomainError> {
        let groups = self.adapter.fetch(group_names).await?;
        self.process_items(groups).await?;
        Ok(())
    }
    
    async fn get_registred_groups(&self, page: u32, size: u32) -> Result<Option<(Vec<RegisteredGroupModel>, u32)>, DomainError> {
        self.registered_group_repository.find(&None, &page, &size).await
    }
    
    async fn process_all_groups(&self, registred_groups: Vec<RegisteredGroupModel>) -> Result<(), DomainError> {
        let group_names: Vec<String> = registred_groups.into_iter().map(|item| item.name).collect();
        self.process_groups(group_names).await?;
        Ok(())
    }
    
    fn is_end_of_registred_groups(&self, total_processed: u32, total_registred_groups: u32) -> bool {
        total_processed >= total_registred_groups
    }

    async fn process_items(&self, items: Vec<GroupCreateModel>) -> Result<(), DomainError> 
    where
        GroupCreateModel: Processable + Guidable,
        dyn GroupRepository: BatchOperations<GroupCreateModel, GroupUpdateModel, GroupModel>, 
        dyn DiffGroupRepository: BatchOperations<DiffGroupModel, DiffGroupModel, DiffGroupModel> + DiffOperations<DiffGroupModel>
    {
        let extids: Vec<String> = items.iter().map(|item| item.extid.clone()).collect();
        let existing_items = &self.diff_repository.find_by_extids(extids).await?;

        let mut items_to_insert: Vec<GroupCreateModel> = Vec::new();
        let mut diff_items_to_insert: Vec<DiffGroupModel> = Vec::new();
        let mut items_to_update: Vec<GroupUpdateModel> = Vec::new();
        let mut diff_items_to_update: Vec<DiffGroupModel> = Vec::new();

        for item in items {
            match existing_items.iter().find(|existing_item| existing_item.get_extid() == item.get_extid()) {
                Some(existing_item) => {
                    self.process_existing_item(&item, existing_item, &mut items_to_update, &mut diff_items_to_update)?;
                }
                None => {
                    self.process_new_item(&item, &mut items_to_insert, &mut diff_items_to_insert)?;
                }
            }
        }

        self.insert_items(&items_to_insert, &diff_items_to_insert).await?;
        self.update_items(&items_to_update, &diff_items_to_update).await?;

        Ok(())
    }

    fn process_existing_item(&self, 
        item: &GroupCreateModel, 
        existing_item: &DiffGroupModel, 
        items_to_update: &mut Vec<GroupUpdateModel>, 
        diff_items_to_update: &mut Vec<DiffGroupModel>
    ) -> Result<(), DomainError> {
        if existing_item.get_checksum() != item.get_checksum() {
            items_to_update.push(item.to_update());
            diff_items_to_update.push(DiffGroupModel::new(item.get_extid(),item.get_checksum()));
        }
        Ok(())
    }

    fn process_new_item(&self, 
        item: &GroupCreateModel, 
        items_to_insert: &mut Vec<GroupCreateModel>, 
        diff_items_to_insert: &mut Vec<DiffGroupModel>
    ) -> Result<(), DomainError> {
        diff_items_to_insert.push(DiffGroupModel::new(item.get_extid(),item.get_checksum()));
        items_to_insert.push(item.clone());
        Ok(())
    }

    async fn insert_items(&self, items_to_insert: &Vec<GroupCreateModel>, diff_items_to_insert: &Vec<DiffGroupModel>) -> Result<(), DomainError> {
        if !items_to_insert.is_empty() {
            let _ = &self.repository.insert_many(items_to_insert.clone()).await?;
            let _ = &self.diff_repository.insert_many(diff_items_to_insert.clone()).await?;
        }
        Ok(())
    }

    async fn update_items(&self, items_to_update: &Vec<GroupUpdateModel>, diff_items_to_update: &Vec<DiffGroupModel>) -> Result<(), DomainError> {
        if !items_to_update.is_empty() {
            let _ = &self.repository.update_many(items_to_update.clone()).await?;
            let _ = &self.diff_repository.update_many(diff_items_to_update.clone()).await?;
        }
        Ok(())
    }
}
