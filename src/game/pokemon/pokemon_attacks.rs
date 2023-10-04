#![allow(dead_code)]
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Attacks {
    level_up_attacks: Vec<LevelUpAttack>,
    hm_tm_attacks: Vec<HMTMAttack>,
    egg_attacks: Vec<EggAttack>,
    tutor_attacks: Vec<TutorAttack>, // Replace with actual tutor attack type
}

#[derive(Clone, Debug, Deserialize)]
pub struct LevelUpAttack {
    attack: String,
    level: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HMTMAttack {
    attack: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EggAttack {
    attack: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TutorAttack {
    attack: String,
}

impl Attacks {
    pub fn new(
        level_up_attacks: Vec<LevelUpAttack>, 
        hm_tm_attacks: Vec<HMTMAttack>,
        egg_attacks: Vec<EggAttack>,
        tutor_attacks: Vec<TutorAttack>
    ) -> Attacks {
        Attacks { 
            level_up_attacks: level_up_attacks, 
            hm_tm_attacks: hm_tm_attacks, 
            egg_attacks: egg_attacks, 
            tutor_attacks: tutor_attacks 
        }
    }
}

impl LevelUpAttack {
    pub fn new(level_attack_name: &str, learn_level: u32) -> LevelUpAttack {
        LevelUpAttack {
            attack: level_attack_name.to_string(),
            level: learn_level 
        }
    }
}

impl HMTMAttack {
    pub fn new(hmtm_attack_name: &str) -> HMTMAttack{
        HMTMAttack { 
            attack: hmtm_attack_name.to_string(), 
        }
    }
}

impl EggAttack {
    pub fn new(egg_attack_name: &str) -> EggAttack {
        EggAttack { 
            attack: egg_attack_name.to_string() 
        }
    }
}

impl TutorAttack {
    pub fn new(tutor_attack_name: &str) -> TutorAttack {
        TutorAttack { 
            attack: tutor_attack_name.to_string() 
        }
    }
}