use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::types::BaseEntity::*;

const ITEM_POWERUP_CHANCES_PATH: &str = "data/config/item_powerup_chances.config";
const ITEM_ENCOUNTER_CHANCES_PATH: &str = "data/config/item_encounter_chances.config";

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
        Item(BaseEntities::deserialize(ITEM_POWERUP_CHANCES_PATH))
    }

    pub fn encounter_chances() -> Self {
        Item(BaseEntities::deserialize(ITEM_ENCOUNTER_CHANCES_PATH))
    }
}
