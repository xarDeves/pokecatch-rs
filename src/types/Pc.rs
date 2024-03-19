use std::io::{self, BufRead, BufReader, Write};
use serde_json::from_str;
use std::fs::{File, OpenOptions};

use crate::types::Pokemon::*;
use crate::paths::*;

pub struct Pc;


impl Pc {
    pub fn is_present(target_pokemon: &Pokemon) -> bool {
        let file = match File::open(get_pc_path()) {
            Ok(file) => file,
            Err(_) => return false,
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(json_str) = line {
                if let Ok(pokemon) = from_str::<Pokemon>(&json_str) {
                    if pokemon == *target_pokemon {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn add(pokemon: &Pokemon) {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(get_pc_path())
            .unwrap();

        let mut file_writer = io::BufWriter::new(file);
        let json_str = serde_json::to_string(&pokemon).unwrap();
        writeln!(&mut file_writer, "{}", json_str).unwrap();
    }

    pub fn to_vector() -> Vec<Pokemon> {
        let file = File::open(get_pc_path()).unwrap();
        let reader = BufReader::new(file);
        let mut pokemon_vec = Vec::new();

        for line in reader.lines() {
            if let Ok(json_str) = line {
                if let Ok(pokemon) = from_str::<Pokemon>(&json_str) {
                    pokemon_vec.push(pokemon);
                }
            }
        }

        pokemon_vec
    }
}
