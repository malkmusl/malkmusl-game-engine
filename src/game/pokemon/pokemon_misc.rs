#![allow(dead_code)]
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct EggGroups {
    group_1: String,
    group_2: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Gender {
    male: f32,
    female: f32,
    no_gender: f32,
}

impl EggGroups {
    pub fn new(
        egg_group_1: String, 
        egg_group_2: String
    ) -> EggGroups {
        EggGroups { 
            group_1: egg_group_1, 
            group_2: egg_group_2 
        }
    }
}

impl Gender {
    pub fn new(
        male: f32, 
        female: f32, 
        no_gender: f32
    ) -> Gender{
        Gender { 
            male: male, 
            female: female, 
            no_gender: no_gender 
        }
    }
}

