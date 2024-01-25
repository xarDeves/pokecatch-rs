use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use crate::types::BaseEntity::*;

const POKEBALL_CONTENTS_PATH: &str = "data/pokeball_available.txt";
const POKEBALL_CHANCES_PATH: &str = "data/config/pokeball_chances.config";

#[derive(Debug, Serialize, Deserialize)]
pub struct Ball;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokeballs(pub BaseEntities);

impl Deref for Pokeballs {
    type Target = BaseEntities;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pokeballs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Pokeballs {
    pub fn new() -> Self {
        Pokeballs(BaseEntities::deserialize(POKEBALL_CONTENTS_PATH))
    }

    pub fn get(&self) -> &BaseEntities {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Catchrates(pub BaseEntities);

impl Deref for Catchrates {
    type Target = BaseEntities;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Catchrates {
    pub fn new() -> Self {
        Catchrates(BaseEntities::deserialize(POKEBALL_CHANCES_PATH))
    }

    pub fn get(&self) -> &BaseEntities {
        &self.0
    }
}

impl Ball {
    pub fn pokeballs() -> Pokeballs {
        Pokeballs::new()
    }

    pub fn catch_rates() -> Catchrates {
        Catchrates::new()
    }
}
