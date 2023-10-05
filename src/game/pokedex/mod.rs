use std::hash::Hash;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use super::pokemon::pokemon::Pokemon;

pub mod regional_dex;

pub struct Pokedex {
    pokemon: Mutex<HashMap<DexNumber, Pokemon>>
}

impl Pokedex {
    pub fn new() -> Pokedex {
        Pokedex { 
            pokemon: Mutex::new(HashMap::new())
        }
    }

    pub fn insert_pokemon(&self, dex_number: f32, pokemon: Pokemon) {
        // Lock the mutex and handle any potential errors
        if let Ok(mut map) = self.pokemon.lock() {
            // Insert the new Pokemon into the Pokedex
            map.insert(DexNumber::f32_to_dex_number(dex_number), pokemon);
        } else {
            // Handle the case where locking failed (e.g., log an error or panic)
            eprintln!("Failed to lock Pokedex");
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct DexNumber {
    dex_number: u8,
    dex_variant: u8
}
impl DexNumber {
    pub fn f32_to_dex_number(float_number: f32) -> DexNumber{
        let mut dex_number: u8 = 0;
        let mut dex_variant: u8 = 0;

        // Convert f32 to a string
        let float_str = float_number.to_string();

        // Split the string at the dot
        let parts: Vec<&str> = float_str.split('.').collect();

        // Convert each part to u8
        if let Some(integer_part) = parts.get(0) {
            if let Ok(integer_value) = integer_part.parse::<u8>() {
                dex_number = integer_value;
            } else {
                println!("Failed to parse integer part as u8");
            }
        }

        if let Some(decimal_part) = parts.get(1) {
            if let Ok(decimal_value) = decimal_part.parse::<u8>() {
                dex_variant = decimal_value;
            } else {
                println!("Failed to parse decimal part as u8");
            }
        }

        DexNumber { 
            dex_number: dex_number, 
            dex_variant: dex_variant 
        }
    }
}
