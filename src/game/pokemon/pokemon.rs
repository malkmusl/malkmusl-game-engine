#![allow(dead_code)]
use serde::Deserialize;

use super::{pokemon_lang::Lang, pokemon_stats::Stats, pokemon_attacks::{Attacks, LevelUpAttack, HMTMAttack, EggAttack, TutorAttack}, pokemon_sprites::Sprites, pokemon_data::Ident};

#[derive(Clone, Debug, Deserialize)]
pub struct Pokemon {
    ident: Ident,
    lang: Lang,
    stats: Stats,
    attacks: Attacks,
    sprites: Sprites,
}

impl Pokemon {
    pub fn new(
        generation: u32, 
        national_dex: u32, 
        main_type: &str, 
        secondary_type: &str,
        main_ability: &str,
        hidde_ability: &str,
        height: &str,
        weight: &str, 
        method: &str, 
        evolution: &str, 
        condition: &str,
        name_eng: &str, 
        name_jpn: &str, 
        name_ger: &str, 
        name_fr: &str, 
        name_it: &str, 
        name_esp: &str, 
        name_kor: &str, 
        name_zh_hans: &str, 
        name_zh_hant: &str,
        desc_eng: &str, 
        desc_jpn: &str, 
        desc_ger: &str,
        desc_fr: &str, 
        desc_it: &str, 
        desc_esp: &str,
        desc_kor: &str, 
        desc_zh_hans: &str, 
        desc_zh_hant: &str,
        species_eng: &str, 
        species_jpn: &str, 
        species_ger: &str, 
        species_fr: &str, 
        species_it: &str, 
        species_esp: &str, 
        species_kor: &str, 
        species_zh_hans: &str, 
        species_zh_hant: &str,
        base_hp: u32, 
        base_attack: u32, 
        base_defense: u32, 
        base_sp_attack: u32, 
        base_sp_defense: u32, 
        base_speed: u32,
        ev_hp: u32, 
        ev_attack: u32, 
        ev_defense: u32, 
        ev_sp_attack: u32, 
        ev_sp_defense: u32, 
        ev_speed: u32,
        egg_group_1: &str, 
        egg_group_2: &str,
        egg_cycle: u32,
        male: f32, 
        female: f32, 
        no_gender: f32,
        catch_rate: u32,
        base_friendship: u32,
        base_exp: u32,
        growth_rate: &str,
        level_up_attacks: Vec<LevelUpAttack>, 
        hm_tm_attacks: Vec<HMTMAttack>,
        egg_attacks: Vec<EggAttack>,
        evolution_attacks: Vec<TutorAttack>,
        tutor_attacks: Vec<TutorAttack>,
        front_sprite: &str,
        back_sprite: &str,
        front_shiny_sprite: &str,
        back_shiny_sprite: &str,
    ) -> Pokemon{
        Pokemon { 
            ident: Ident::new(
                generation, 
                national_dex, 
                main_type.to_string(), 
                secondary_type.to_string(), 
                method.to_string(), 
                evolution.to_string(), 
                condition.to_string()
            ), 
            lang: Lang::new(
                name_eng.to_string(), name_jpn.to_string(), name_ger.to_string(), name_fr.to_string(), name_it.to_string(),
                name_esp.to_string(), name_kor.to_string(), name_zh_hans.to_string(), name_zh_hant.to_string(), 
                desc_eng.to_string(), desc_jpn.to_string(), desc_ger.to_string(), desc_fr.to_string(), desc_it.to_string(),
                desc_esp.to_string(), desc_kor.to_string(), desc_zh_hans.to_string(), desc_zh_hant.to_string(), 
                species_eng.to_string().to_string(), species_jpn.to_string(), species_ger.to_string(), species_fr.to_string(), species_it.to_string(), 
                species_esp.to_string(), species_kor.to_string(), species_zh_hans.to_string(), species_zh_hant.to_string()
            ), 
            stats: Stats::new(
                base_hp, base_attack, base_defense, base_sp_attack, base_sp_defense, base_speed, 
                ev_hp, ev_attack, ev_defense, ev_sp_attack, ev_sp_defense, ev_speed, 
                egg_group_1.to_string(), egg_group_2.to_string(), male, female, no_gender, catch_rate, base_friendship, base_exp, growth_rate.to_string()
            ),
            attacks: Attacks::new(level_up_attacks, hm_tm_attacks, egg_attacks, tutor_attacks), 
            sprites: Sprites::new(front_sprite.to_string(), back_sprite.to_string(), front_shiny_sprite.to_string(), back_shiny_sprite.to_string())
        }
    }

    pub fn get(&self) -> Pokemon{
        Pokemon { ident: self.ident.clone(), lang: self.lang.clone(), stats: self.stats.clone(), attacks: self.attacks.clone(), sprites: self.sprites.clone() }
    }
}