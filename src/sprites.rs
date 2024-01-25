use std::process::exit;
use rand::seq::SliceRandom;
use rand::seq::IteratorRandom;
use std::path::Path;
use image::{DynamicImage, GenericImage};

use crate::types::Pokemon::Pokemon;
use crate::Data;

fn get_random_pokemon_name_from_embedded_sprite_path() -> String {
    // Use Iterator::choose to randomly select an entry
    if let Some(entry) = Data::iter().choose(&mut rand::thread_rng()) {
        // Use Path to extract the file name without path information
        if let Some(file_name) = Path::new(entry.as_ref()).file_name() {
            return file_name.to_string_lossy().to_string();
        }
    }

    // Handle the case where no entry or file name is found
    String::new()
}

fn sanitize_filename(filename: String) -> String {
    // I hate Mr. Mime and Farfetch'd.
    filename.replace([' ', '_'], "-")
        .replace(['.', '\'', ':'], "")
        .to_lowercase()
}

pub fn get_random_sprite_from_file(
    pokemon: &mut Pokemon,
) -> Vec<u8> {
    let mut filename;
    let path: String;
    
    if pokemon.legendary {
        let pokemon_list_str = include_str!("../data/pokemon_legendary.txt");
        let pokemon_list: Vec<&str> = pokemon_list_str.lines().collect();
        filename = pokemon_list.choose(&mut rand::thread_rng()).unwrap().to_string();
    
        filename = sanitize_filename(filename.clone());
    
        path = format!("{}/{}.png", if pokemon.shiny { "shiny" } else { "regular" }, filename.trim());
    } else {
        filename = get_random_pokemon_name_from_embedded_sprite_path();
    
        if let Some(first_element) = filename.clone().split('.').next() {
            filename = sanitize_filename(first_element.to_string());
        }
    
        path = format!("{}/{}.png", if pokemon.shiny { "shiny" } else { "regular" }, filename.trim());
    }
    
    pokemon.name = filename;

    Data::get(path.as_str())
        .unwrap_or_else(|| {
            eprintln!("pokemon not found at {}", path);
            exit(1);
        })
        .data
        .into_owned()
}

/// Fetches a sprite and returns a vector of bytes.
/// This will also format the names properly.
pub fn get_sprite(
    pokemon: &mut String,
    shiny: bool
) -> Vec<u8> {
    let path: String;

    path = format!("{}/{}.png", if shiny { "shiny" } else { "regular" }, sanitize_filename(pokemon.clone()).trim());

    Data::get(path.as_str())
        .unwrap_or_else(|| {
            eprintln!("pokemon not found at {}", path);
            exit(1);
        })
        .data
        .into_owned()
}

pub fn combine_sprites_horizontally(
    combined_width: u32,
    combined_height: u32,
    sprites: &[DynamicImage],
) -> DynamicImage {
    let mut combined = DynamicImage::new_rgba8(combined_width - 1, combined_height);
    let mut shift = 0;

    for sprite in sprites {
        combined
            .copy_from(sprite, shift, combined_height - sprite.height())
            .unwrap();
        shift += sprite.width() + 1;
    }

    combined
}

pub fn get_sprites(pokemons: &mut [Pokemon],) -> (u32, u32, Vec<DynamicImage>) {
    let mut sprites = Vec::new();
    let mut combined_width: u32 = 0;
    let mut combined_height: u32 = 0;

    for pokemon in pokemons.iter_mut() {
        let bytes = get_sprite(&mut pokemon.name, pokemon.shiny);

        let img = image::load_from_memory(&bytes).unwrap();
        let trimmed = showie::trim(&img);

        combined_width += trimmed.width() + 1;

        if trimmed.height() > combined_height {
            combined_height = trimmed.height();
        }

        sprites.push(trimmed);
    }

    (combined_width, combined_height, sprites)
}
