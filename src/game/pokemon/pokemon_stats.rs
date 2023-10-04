#![allow(dead_code)]
use serde::Deserialize;

use super::pokemon_misc::{EggGroups, Gender};

#[derive(Clone, Debug, Deserialize)]
pub struct Stats {
    base: BaseStats,
    ev: EVStats,
    misc: MiscStats,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BaseStats {
    hp: u32,
    attack: u32,
    defense: u32,
    sp_attack: u32,
    sp_defense: u32,
    speed: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EVStats {
    hp: u32,
    attack: u32,
    defense: u32,
    sp_attack: u32,
    sp_defense: u32,
    speed: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MiscStats {
    egg_groups: EggGroups,
    gender: Gender,
    catch_rate: u32,
    base_friendship: u32,
    base_exp: u32,
    growth_rate: String, // You may want to define an enum for growth rates
}

impl Stats {
    pub fn new(
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
        egg_group_1: String, 
        egg_group_2: String,
        male: f32, 
        female: f32, 
        no_gender: f32,
        catch_rate: u32,
        base_friendship: u32,
        base_exp: u32,
        growth_rate: String
    ) -> Stats{
        Stats { 
            base: BaseStats::new(base_hp, base_attack, base_defense, base_sp_attack, base_sp_defense, base_speed), 
            ev: EVStats::new(ev_hp, ev_attack, ev_defense, ev_sp_attack, ev_sp_defense, ev_speed), 
            misc: MiscStats::new(egg_group_1, egg_group_2, male, female, no_gender, catch_rate, base_friendship, base_exp, growth_rate) 
        }
    }
}

impl BaseStats {
    pub fn new(
        base_hp: u32, 
        base_attack: u32, 
        base_defense: u32, 
        base_sp_attack: u32, 
        base_sp_defense: u32, 
        base_speed: u32
    ) -> BaseStats{
        BaseStats { 
            hp: base_hp, 
            attack: base_attack, 
            defense: base_defense, 
            sp_attack: base_sp_attack, 
            sp_defense: base_sp_defense, 
            speed: base_speed 
        }
    }
}

impl EVStats {
    pub fn new(
        ev_hp: u32, 
        ev_attack: u32, 
        ev_defense: u32, 
        ev_sp_attack: u32, 
        ev_sp_defense: u32, 
        ev_speed: u32
    ) -> EVStats{
        EVStats { 
            hp: ev_hp, 
            attack: ev_attack, 
            defense: ev_defense, 
            sp_attack: ev_sp_attack, 
            sp_defense: ev_sp_defense, 
            speed: ev_speed 
        }
    }
}

impl MiscStats {
    pub fn new(
        egg_group_1: String, 
        egg_group_2: String,
        male: f32, 
        female: f32, 
        no_gender: f32,
        catch_rate: u32,
        base_friendship: u32,
        base_exp: u32,
        growth_rate: String
    ) -> MiscStats {
        MiscStats { 
            egg_groups: EggGroups::new(egg_group_1, egg_group_2), 
            gender: Gender::new(male, female, no_gender),
            catch_rate: catch_rate, 
            base_friendship: base_friendship, 
            base_exp: base_exp, 
            growth_rate: growth_rate 
        }
    }
}