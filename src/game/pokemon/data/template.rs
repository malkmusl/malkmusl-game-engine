extern crate lazy_static;
use lazy_static::lazy_static;

use crate::game::pokedex::Pokedex;
use crate::game::pokemon::data::kanto::*;

lazy_static! {
    pub static ref MISSINGNO: Pokemon = Pokemon::new(
        0,                          //*  Generation
        0000,                       //*  National Dex Number 
        "", "",                     //*  Types
        "", "",                     //*  Abilitys
        "", "", "",                 //*  Evolution Chain 
        "",                         //*  Name English
        "",                         //*  Name Japanese
        "",                         //*  Name German
        "",                         //*  Name France
        "",                         //*  Name Italy
        "",                         //*  Name Espanole
        "",                         //*  Name Korean
        "",                         //*  Name Chinese Simplyfied 
        "",                         //*  Name Chinese Traditional
        "",                         //*  Desc English
        "",                         //*  Desc Japanese
        "",                         //*  Desc German
        "",                         //*  Desc France
        "",                         //*  Desc Italy
        "",                         //*  Desc Espanole
        "",                         //*  Desc Korean
        "",                         //*  Desc Chinese Simplyfied
        "",                         //*  Desc Chinese Traditional
        "",                         //*  Species English
        "",                         //*  Species Japanese
        "",                         //*  Species German
        "",                         //*  Species France
        "",                         //*  Species Italy
        "",                         //*  Species Espanole
        "",                         //*  Species Korean
        "",                         //*  Species Chinese Simplyfied
        "",                         //*  Species Chinese Traditional
        0, 
        0, 
        0, 
        0, 
        0, 
        0, 
        0, 
        0, 
        0, 
        0, 
        0, 
        0, 
        "", 
        "", 
        0.0, 
        0.0, 
        0.0, 
        0, 
        0, 
        0, 
        "MEDIUM_SLOW", 
        vec!(
            LevelUpAttack::new("growl", 1),
            LevelUpAttack::new("tackle", 1),
            LevelUpAttack::new("vine_whip", 3),
            LevelUpAttack::new("growth", 6),
            LevelUpAttack::new("leech_seed", 9),
            LevelUpAttack::new("razor_leaf", 12),
            LevelUpAttack::new("poison_powder", 15),
            LevelUpAttack::new("sleep_powder", 15),
            LevelUpAttack::new("seed_bomb", 18),
            LevelUpAttack::new("take_down", 21),
            LevelUpAttack::new("sweet_scent", 24),
            LevelUpAttack::new("synthesis", 27),
            LevelUpAttack::new("worry_seed", 30),
            LevelUpAttack::new("double_edge", 33),
            LevelUpAttack::new("solar_beam", 36),
        ),
        vec!(
            HMTMAttack::new("toxic"),
            HMTMAttack::new("bullet_seed"),
            HMTMAttack::new("work_up"),
            HMTMAttack::new("sunny_day"),
            HMTMAttack::new("light_screen"),
            HMTMAttack::new("protect"),
            HMTMAttack::new("giga_drain"),
            HMTMAttack::new("safeguard"),
            HMTMAttack::new("solar_beam"),
            HMTMAttack::new("double_team"),
            HMTMAttack::new("sludge_bomb"),
            HMTMAttack::new("facade"),
            HMTMAttack::new("rest"),
            HMTMAttack::new("attract"),
            HMTMAttack::new("energy_ball"),
            HMTMAttack::new("false_swipe"),
            HMTMAttack::new("endure"),
            HMTMAttack::new("flash"),
            HMTMAttack::new("swords_dance"),
            HMTMAttack::new("sleep_talk"),
            HMTMAttack::new("grass_knot"),
            HMTMAttack::new("swagger"),
            HMTMAttack::new("substitute"),
            HMTMAttack::new("cut"),
            HMTMAttack::new("strength"),
            HMTMAttack::new("rock_smash"),
        ), 
        vec!(
            EggAttack::new("amnesia"),
            EggAttack::new("charm"),
            EggAttack::new("curse"),
            EggAttack::new("grassy_terrain"),
            EggAttack::new("ingrain"),
            EggAttack::new("leaf_storm"),
            EggAttack::new("magical_leaf"),
            EggAttack::new("nature_power"),
            EggAttack::new("petal_dance"),
            EggAttack::new("power_whip"),
            EggAttack::new("skull_bash"),
            EggAttack::new("sludge"),
    
        ), 
        vec!(), 
        "front_sprite", 
        "back_sprite", 
        "front_shiny_sprite", 
        "back_shiny_sprite"
    );
}