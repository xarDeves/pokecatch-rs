use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use crate::types::BaseEntity::*;

const BAG_CONTENTS_PATH: &str = "data/bag_contents.txt";

#[derive(Debug, Serialize, Deserialize)]
pub struct Bag(pub BaseEntities);

impl Bag {
    pub fn contents() -> Self {
        Bag(BaseEntities::deserialize(BAG_CONTENTS_PATH))
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