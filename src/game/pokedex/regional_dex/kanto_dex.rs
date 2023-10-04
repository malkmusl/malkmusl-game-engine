extern crate lazy_static;
use lazy_static::lazy_static;

use crate::game::pokedex::Pokedex;
use crate::game::pokemon::data::kanto::*;

lazy_static!{
    pub static ref KANTO_DEX: Pokedex = Pokedex::new();
}

pub fn create_kanto_dex() {
    KANTO_DEX.insert_pokemon(0001, BULBASAUR.get());
    KANTO_DEX.insert_pokemon(0002, IVYSAUR.get());
    KANTO_DEX.insert_pokemon(0003, VENUSAUR.get());
}