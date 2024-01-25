use clap::Parser;

use pokecatch::utils::*;
use pokecatch::cli::*;
use pokecatch::sprites::*;

use pokecatch::types::BaseEntity::*;
use pokecatch::types::Pc::*;
use pokecatch::types::Bag::*;
use pokecatch::types::Pokemon::*;
use pokecatch::types::Ball::*;
use pokecatch::types::Item::*;

const POKEBALL_CONTENTS_PATH: &str = "data/pokeball_available.txt";
const BAG_CONTENTS_PATH: &str = "data/bag_contents.txt";

fn throw(ball: &str) {
    let current_pokemon = Pokemon::read_from_current();
    
    if current_pokemon.name == "" {
        println!("no pokemon encountered, restart your terminal or re-run pokecatch !");
        return;
    }
    
    if Pc::is_present(&current_pokemon){
        println!("you already have that pokemon !");
        return;
    }

    println!("Throwing {} ball...", ball);

    let mut pokeballs_contents = Ball::pokeballs();
    let pokeballs_catch_rates_contents = Ball::catch_rates();

    let caught;
    let catch_rate = pokeballs_catch_rates_contents.get_quantity(&ball);
    let count = pokeballs_contents.get_quantity(&ball);

    if count > 0 {
        pokeballs_contents.modify_quantity(&ball, -1);
        caught = calc_success_from_percentage(catch_rate);
    } else {
        println!("Not enough {}s", ball);
        return;
    }

    BaseEntities::serialize(POKEBALL_CONTENTS_PATH, &pokeballs_contents);

    if caught {
        Pc::add(&current_pokemon);
        //erase the current pokemon encountered from file since we caught it
        Pokemon::write_to_current(& Pokemon::new_emtpy());
        println!("you caught {} !", &current_pokemon.name);
    }
    else{
        println!("oh no, you could not catch {}.", current_pokemon.name);
    }
}

/* fn sanitize_filename(filename: String) -> String {
    // I hate Mr. Mime and Farfetch'd.
    filename.replace([' ', '_'], "-")
        .replace(['.', '\'', ':'], "")
        .to_lowercase()
} */

//TODO: should also fetch shinies and legendaries to contruct 'Pokemon' correctly
//that is, all permutations of shinies, legendaries, and normals
fn pc_show(args: &PcShowArgs) {
    if args.all {
        println!("Showing all Pokemon");
        let all_pokemon_names = get_all_pokemon_names();        
        for name in all_pokemon_names {
            display_pokemon_name(&Pokemon::new_with_attrs(name, false,false));
        }
    } else if args.caught {
        println!("Showing caught Pokemon");
        //display caught vertically
        let mut pokemons = Pc::to_vector();
        let (_width, _height, sprites) = get_sprites(&mut pokemons);

        eprintln!(
            "{}\n",
            pokemons
                .iter()
                .enumerate()
                .map(|(i, x)| (x.name.to_string()) + if i != pokemons.len() - 1 { ", " } else { "" })
                .collect::<String>()
        );

        for (pokemon, sprite) in pokemons.iter().zip(sprites.iter()) {
            display_pokemon_name(pokemon);
            println!("{}", showie::to_ascii(sprite));
        }
        
        //display caught horizontally
        /* let combined = combine_sprites_horizontally(width, height, &sprites);
        eprintln!(
            "{}\n",
            pokemons
                .iter()
                .enumerate()
                .map(|(i, x)| sanitize_filename(x.name.to_string()) + if i != pokemons.len() - 1 { ", " } else { "" })
                .collect::<String>()
        );
        println!("{}", showie::to_ascii(&combined)); */
    } else if args.uncaught {
        println!("Showing uncaught Pokemon");
        let pokemons_in_pc = Pc::to_vector();
        let all_pokemon_names = get_all_pokemon_names();
        let not_in_pc: Vec<String> = all_pokemon_names
            .iter()
            .filter(|name| !pokemons_in_pc.iter().any(|pc_pokemon| pc_pokemon.name == **name))
            .cloned()
            .collect();

        for name in not_in_pc {
            display_pokemon_name(&Pokemon::new_with_attrs(name, false,false));
        }
    }else {
        println!("Invalid command, use 'pokecatch pc show -all', 'pokecatch pc show -caught', or 'pokecatch pc show -uncaught'");
    }
}

fn bag_show() {
    let bag_contents = Bag::contents();

    for item in &bag_contents.0 .0 {
        println!("{} - {}", item.name, item.quantity);
    }
}

fn bag_use(args: &BagUseArgs) {
    let mut bag_contents = Bag::contents();
    
    if args.items.contains(&String::from("legendary-bait")) && args.items.contains(&String::from("shiny-bait")) {
        use_legendary_and_shiny_bait(&mut bag_contents);
    } else {
        use_individual_baits(&mut bag_contents, &args.items);
    }

    BaseEntities::serialize(BAG_CONTENTS_PATH, &bag_contents);
}

fn use_legendary_and_shiny_bait(bag_contents: &mut Bag) {
    let shiny_bait_amount = bag_contents.get_quantity("shiny-bait");
    let legendary_bait_amount = bag_contents.get_quantity("legendary-bait");

    if shiny_bait_amount <= 0 || legendary_bait_amount <= 0 {
        println!("You are out of legendary and shiny bait!");
        return;
    }

    println!("Using legendary-bait and shiny-bait together...");

    bag_contents.modify_quantity("legendary-bait", -1);
    bag_contents.modify_quantity("shiny-bait", -1);

    let legendary_success = calc_success_from_percentage(Item::powerup_chances().get_quantity("legendary-bait"));
    let shiny_success = calc_success_from_percentage(Item::powerup_chances().get_quantity("shiny-bait"));

    if legendary_success || shiny_success {
        let shiny = shiny_success;
        let legendary = legendary_success;

        summon_pokemon(shiny, legendary);   
    } else {
        println!("Not even a nibble...");
    }
}

fn use_individual_baits(bag_contents: &mut Bag, items: &Vec<String>) {
    for item in items.iter() {
        match item.as_str() {
            "legendary-bait" => use_individual_bait(bag_contents, item, "legendary"),
            "shiny-bait" => use_individual_bait(bag_contents, item, "shiny"),
            _ => println!("Unknown item: {}", item),
        }
    }
}

fn use_individual_bait(bag_contents: &mut Bag, item: &str, bait_type: &str) {
    let bait_amount = bag_contents.get_quantity(item);

    if bait_amount <= 0 {
        println!("You are out of {} bait!", bait_type);
        return;
    }

    println!("Using item: {}", item);

    bag_contents.modify_quantity(item, -1);
    BaseEntities::serialize(BAG_CONTENTS_PATH, &bag_contents);

    let success = calc_success_from_percentage(Item::powerup_chances().get_quantity(item));

    if success {
        let shiny = bait_type == "shiny";
        let legendary = bait_type == "legendary";

        summon_pokemon(shiny, legendary);
    } else {
        println!("Not even a nibble...");
    }
}

fn summon_pokemon(shiny: bool, legendary: bool) {
    let mut pokemon = Pokemon::new_with_attrs(String::new(), shiny, legendary);
    summon_random_pokemon(&mut pokemon);
}

//we could name our data in the encounter chance files the same as in the item files and use iteration
//but since we don't have a lot of cases i prefer this even though it is quite cluttered
fn roll_and_fetch_random_item() {
    let encounter_chances = Item::encounter_chances();
    let mut pokeballs = Ball::pokeballs();
    let mut bag = Bag::contents();

    let mut encountered = false;

    if calc_success_from_percentage(encounter_chances.0.get_quantity("general_item_encounter")) {
        if calc_success_from_percentage(encounter_chances.0.get_quantity("pokeball_encounter")) {
            pokeballs.0.modify_quantity("pokeball", 1);
            println!("Congrats, you found a Poke-Ball!");
            encountered = true;
        }
        if calc_success_from_percentage(encounter_chances.0.get_quantity("greatball_encounter")) {
            pokeballs.0.modify_quantity("greatball", 1);
            println!("Congrats, you found a Great-Ball!");
            encountered = true;
        }
        if calc_success_from_percentage(encounter_chances.0.get_quantity("ultraball_encounter")) {
            pokeballs.0.modify_quantity("ultraball", 1);
            println!("Congrats, you found an Ultra-Ball!");
            encountered = true;
        }
        if calc_success_from_percentage(encounter_chances.0.get_quantity("masterball_encounter")) {
            pokeballs.0.modify_quantity("masterball", 1);
            println!("Congrats, you found a Master-Ball!");
            encountered = true;
        }
        if calc_success_from_percentage(encounter_chances.0.get_quantity("shiny-bait_encounter")) {
            bag.0.modify_quantity("shiny-bait", 1);
            println!("Congrats, you found a Shiny Bait!");
            encountered = true;
        }
        if calc_success_from_percentage(encounter_chances.0.get_quantity("legendary-bait_encounter")) {
            bag.0.modify_quantity("legendary-bait", 1);
            println!("Congrats, you found a Legendary Bait!");
            encountered = true;
        }
    } 
    
    if !encountered{
        println!("No items encountered...");
    }

    BaseEntities::serialize(BAG_CONTENTS_PATH, &bag);
    BaseEntities::serialize(POKEBALL_CONTENTS_PATH, &pokeballs);
}

fn execute_subcommand(args: &Args) -> bool {
    if let Some(subcommand) = &args.subcommand {
        match subcommand {
            SubCommand::Throw { ball } => throw(ball),
            SubCommand::Pc { pc_subcommand } => match pc_subcommand {
                Some(PcSubCommand::Show(pc_show_args)) => pc_show(pc_show_args),
                None => return false,
            },            
            SubCommand::Bag { bag_subcommand } => match bag_subcommand {
                Some(BagSubCommand::Show) => bag_show(),
                Some(BagSubCommand::Use(items)) => bag_use(items),
                None => return false,
            },
        }
        return true;
    }
    false
}

fn display_pokemon_name(pokemon : &Pokemon){
    print!("{}", format_pokemon_name(&pokemon.name));

    if Pc::is_present(&pokemon) {
        //green
        print!("\x1b[32m ✓\x1b[0m");
    } else {
        //red
        print!("\x1b[31m ✗\x1b[0m");
    }
    
    if pokemon.name.contains("female") {
        //pink
        print!("\x1b[38;5;205m ♀\x1b[0m");
    } else {
        //blue
        print!("\x1b[34m ♂\x1b[0m");
    }
    
    if pokemon.shiny{
        print!(" ✨");
    }

    if pokemon.legendary {
        print!(" 💫");
    }

    println!("");
}

//TODO: split to functions
//TODO: incorporate 'legendary' to type ?
fn summon_random_pokemon(pokemon: &mut Pokemon) {
    let bytes = get_random_sprite_from_file(pokemon);
    let img = image::load_from_memory(&bytes).unwrap();
    let trimmed = showie::trim(&img);
    
    Pokemon::write_to_current(&pokemon);
    display_pokemon_name(&pokemon);

    println!("{}", showie::to_ascii(&trimmed));
}

fn main() {
    let args = Args::parse();

    //a subcommand was not found, thus display a random pokemon
    if !execute_subcommand(&args){
        roll_and_fetch_random_item();

        let mut pokemon = Pokemon::new_with_attrs(
            String::new(),
            calc_success_from_rate(Pokemon::read_encounter_rates().get_quantity("shiny")),
            calc_success_from_rate(Pokemon::read_encounter_rates().get_quantity("legendary"))
        );
        
        summon_random_pokemon(&mut pokemon);
    }
}