use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use super::pokemon::pokemon::Pokemon;

pub mod regional_dex;

pub struct Pokedex {
    pokemon: Mutex<HashMap<u32, Pokemon>>
}

impl Pokedex {
    pub fn new() -> Pokedex {
        Pokedex { 
            pokemon: Mutex::new(HashMap::new())
        }
    }

    pub fn insert_pokemon(&self, dex_number: u32, pokemon: Pokemon) {
        let mut map: MutexGuard<HashMap<u32, Pokemon>> = self.pokemon.lock().expect("Failed to lock Pokedex");
        // Insert the new Pokemon into the Pokedex
        map.insert(dex_number, pokemon);
    }
}