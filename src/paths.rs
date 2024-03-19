use std::env;
use std::path::PathBuf;

const DATA_DIR: &str = "data";
const CONFIG_DIR: &str = "config";

// Files ending in .txt only require the DATA_DIR prefix
// Files ending in .config require both DATA_DIR and CONFIG_DIR prefix (in that order)
const BAG_CONTENTS_PATH: &str = "bag_contents.txt";
const CURRENT_POKEMON_FILE: &str = "current_pokemon.txt";
const POKEBALL_CONTENTS_PATH: &str = "pokeball_available.txt";
const PC_PATH: &str = "pc.txt";
const POKEMON_ENCOUNTER_CHANCES_FILE: &str = "pokemon_encounter_chance.config";
const ITEM_POWERUP_CHANCES_PATH: &str = "item_powerup_chances.config";
const ITEM_ENCOUNTER_CHANCES_PATH: &str = "item_encounter_chances.config";
const POKEBALL_CHANCES_PATH: &str = "pokeball_chances.config";


fn exe_dir() -> PathBuf {
    env::current_exe()
        .expect("Failed to get current executable's path")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf()
}

pub fn get_pc_path() -> String {
    exe_dir()
        .join(DATA_DIR)
        .join(PC_PATH)
        .to_string_lossy()
        .into_owned()
}

pub fn get_current_pokemon_path() -> String {
    exe_dir()
        .join(DATA_DIR)
        .join(CURRENT_POKEMON_FILE)
        .to_string_lossy()
        .into_owned()
}

pub fn get_bag_contents_path() -> String {
    exe_dir()
        .join(DATA_DIR)
        .join(BAG_CONTENTS_PATH)
        .to_string_lossy()
        .into_owned()
}

pub fn get_pokeball_contents_path() -> String {
    exe_dir()
        .join(DATA_DIR)
        .join(POKEBALL_CONTENTS_PATH)
        .to_string_lossy()
        .into_owned()
}

pub fn get_pokemon_encounter_chances_path() -> String {
    exe_dir()
        .join(DATA_DIR)
        .join(CONFIG_DIR)
        .join(POKEMON_ENCOUNTER_CHANCES_FILE)
        .to_string_lossy()
        .into_owned()
}

pub fn get_item_powerup_chances_path() -> String {
    exe_dir()
        .join(DATA_DIR)
        .join(CONFIG_DIR)
        .join(ITEM_POWERUP_CHANCES_PATH)
        .to_string_lossy()
        .into_owned()
}

pub fn get_item_encounter_chances_path() -> String {
    exe_dir()
        .join(DATA_DIR)
        .join(CONFIG_DIR)
        .join(ITEM_ENCOUNTER_CHANCES_PATH)
        .to_string_lossy()
        .into_owned()
}

pub fn get_pokeball_chances_path() -> String {
    exe_dir()
        .join(DATA_DIR)
        .join(CONFIG_DIR)
        .join(POKEBALL_CHANCES_PATH)
        .to_string_lossy()
        .into_owned()
}