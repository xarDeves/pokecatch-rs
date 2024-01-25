//REFACTOR: i don't like the name of this type

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseEntity {
    pub name: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseEntities(pub Vec<BaseEntity>);

impl BaseEntities {
    pub fn modify_quantity(&mut self, item_name: &str, amount: i32) {
        if let Some(item) = self.0.iter_mut().find(|i| i.name == item_name) {
            if amount > 0 {
                item.quantity = item.quantity.saturating_add(amount as u32);
            } else {
                item.quantity = item.quantity.saturating_sub(amount.abs() as u32);
            }
        } else if amount > 0 {
            self.0.push(BaseEntity {
                name: item_name.to_string(),
                quantity: amount as u32,
            });
        }
    }
    
    pub fn deserialize(filename: &str) -> BaseEntities {
        let contents = fs::read_to_string(filename).expect("Unable to read file");
        let mut items = Vec::new();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let quantity: u32 = parts[1].parse().expect("Invalid quantity");
                items.push(BaseEntity { name, quantity });
            } else {
                panic!("Invalid format in file");
            }
        }

        BaseEntities(items)
    }

    pub fn serialize(filename: &str, items: &BaseEntities) {
        let mut content = String::new();

        for item in &items.0 {
            content.push_str(&format!("{} {}\n", item.name, item.quantity));
        }

        fs::write(filename, content).expect("Unable to write file");
    }

    pub fn get_quantity(&self, item_name: &str) -> u32 {
        self.0.iter().find(|i| i.name == item_name).map_or(0, |item| item.quantity)
    }

    pub fn print_items(item: &BaseEntities) {
        for item in &item.0 {
            eprintln!("Name: {}, Quantity: {}", item.name, item.quantity);
        }
        eprintln!("--------");
    }
}