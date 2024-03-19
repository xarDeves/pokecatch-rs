use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::types::BaseEntity::*;
use crate::paths::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct Item(pub BaseEntities);

impl Deref for Item {
    type Target = BaseEntities;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Item {
    pub fn powerup_chances() -> Self {
        Item(BaseEntities::deserialize(&get_item_powerup_chances_path()))
    }

    pub fn encounter_chances() -> Self {
        Item(BaseEntities::deserialize(&get_item_encounter_chances_path()))
    }
}
