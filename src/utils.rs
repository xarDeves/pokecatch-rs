use rand::Rng;
use inflector::Inflector;
use std::path::Path;
use crate::Data;

pub fn calc_success_from_percentage(success_percentage: u32) -> bool {
    let mut rng = rand::thread_rng();
    let random_num: u32 = rng.gen_range(0..=100);

    if random_num <= success_percentage {
        return true;
    } else {
        return false;
    }
}

pub fn calc_success_from_rate(rate: u32) -> bool {
    let mut rng = rand::thread_rng();
    let random_num: u32 = rng.gen_range(1..=rate);

    return random_num == 1;
}

pub fn generate_random_pokemon_id(pokemon_list: &Box<[&str]>) -> String {
    let mut rng = rand::thread_rng();
    let random_number: usize = rng.gen_range(1..=pokemon_list.len() + 1);

    random_number.to_string()
}

/// Returns a random pokemon from a pokemon name list.
pub fn random_name_pokemon_from_list(list: &[&str]) -> String {
    let mut rand = rand::thread_rng();

    String::from(list[rand.gen_range(0..list.len())])
}

pub fn format_pokemon_name(name: &String) -> String {
    name.to_title_case().replace('-', " ")
}

pub fn get_all_pokemon_names() -> Vec<String> {
    let all_pokemon_names: Vec<String> = Data::iter()
        .filter_map(|entry| {
            Path::new(entry.as_ref())
                .file_name()
                .map(|file_name| {
                    let filename = file_name.to_string_lossy();
                    let pokemon_name = filename
                        .clone()
                        .split('.')
                        .next()
                        .map(|first_element| first_element.to_string());

                    // If there's a name, return it; otherwise, return an empty string
                    pokemon_name.unwrap_or_else(|| String::new())
                })
        })
        .collect();

    all_pokemon_names
}