use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use crate::types::BaseEntity::*;
use crate::paths::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct Bag(pub BaseEntities);

impl Bag {
    pub fn contents() -> Self {
        Bag(BaseEntities::deserialize(&get_bag_contents_path()))
    }
}

impl Deref for Bag {
    type Target = BaseEntities;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}