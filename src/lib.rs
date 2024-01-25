use rust_embed::RustEmbed;

pub mod cli;
pub mod sprites;
pub mod utils;

#[allow(non_snake_case)]
pub mod types {
    pub mod BaseEntity;
    pub mod Pokemon;
    pub mod Bag;
    pub mod Ball;
    pub mod Item;
    pub mod Pc;
}

#[derive(RustEmbed)]
#[folder = "data/pokesprite/pokemon-gen8"]
#[include = "*"]
pub struct Data;