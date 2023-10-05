extern crate lazy_static;
use lazy_static::lazy_static;

use crate::game::pokedex::Pokedex;
use crate::game::pokemon::data::kanto::*;

lazy_static!{
    pub static ref KANTO_DEX: Pokedex = Pokedex::new();
}

pub fn create_kanto_dex() {

//?  KANTO_DEX.insert_pokemon(3.1, MEGA_VENUSAUR.get());

    KANTO_DEX.insert_pokemon(1.0, BULBASAUR.get());
    KANTO_DEX.insert_pokemon(2.0, IVYSAUR.get());
    KANTO_DEX.insert_pokemon(3.0, VENUSAUR.get());
    KANTO_DEX.insert_pokemon(3.1, MEGA_VENUSAUR.get());
    KANTO_DEX.insert_pokemon(4.0, CHARMANDER.get());
    KANTO_DEX.insert_pokemon(5.0, CHARMELEON.get());
    KANTO_DEX.insert_pokemon(6.0, CHARIZARD.get());
    KANTO_DEX.insert_pokemon(6.1, MEGA_CHARIZARD_X.get());
    KANTO_DEX.insert_pokemon(6.2, MEGA_CHARIZARD_Y.get());
}