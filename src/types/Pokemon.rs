use serde::{Deserialize, Serialize};
use serde_json::from_str;

use std::io::{BufRead, BufReader, Write};
use std::fs::File;

use crate::types::BaseEntity::*;

const CURRENT_POKEMON_PATH: &str = "data/current_pokemon.txt";
const POKEMON_ENCOUNTER_CHANCES_PATH: &str = "data/config/pokemon_encounter_chance.config";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Pokemon {
    pub name: String,
    pub shiny: bool,
    pub legendary: bool
}

impl Pokemon {
    pub fn new_emtpy() -> Pokemon {
        Pokemon {
            name: String::new(),
            shiny: false,
            legendary: false,
        }
    }

    pub fn new_with_attrs(name: String, shiny: bool, legendary: bool) -> Pokemon {
        Pokemon {
            name,
            shiny,
            legendary,
        }
    }

    pub fn write_to_current(pokemon: &Pokemon) {
        let json_str = serde_json::to_string(pokemon).unwrap();
    
        match File::create(CURRENT_POKEMON_PATH) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(json_str.as_bytes()) {
                    eprintln!("Error writing to file {}: {}", CURRENT_POKEMON_PATH, err);
                } else {
                    eprintln!("Successfully wrote {} to {}", json_str, CURRENT_POKEMON_PATH);
                }
            }
            Err(_) => {
                eprintln!("Error creating or opening file {}", CURRENT_POKEMON_PATH);
            }
        }
    }

    pub fn read_from_current() -> Pokemon {
        match File::open(CURRENT_POKEMON_PATH) {
            Ok(file) => {
                let reader = BufReader::new(file);
                if let Some(Ok(line)) = reader.lines().next() {
                    if let Ok(pokemon) = from_str::<Pokemon>(&line) {
                        return pokemon;
                    }
                }
            }
            Err(_) => {
                eprintln!("Error opening file {}", CURRENT_POKEMON_PATH);
            }
        }
    
        //if error, return a pokemon with no name
        Pokemon {
            name: String::new(),
            shiny: false,
            legendary: false,
        }
    }

    pub fn read_encounter_rates() -> BaseEntities {
        BaseEntities::deserialize(POKEMON_ENCOUNTER_CHANCES_PATH)
    }
}

