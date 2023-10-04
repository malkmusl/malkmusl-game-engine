#![allow(dead_code)]
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Lang {
    name: Name,
    description: Description,
    species: Species,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Name {
    eng: String,
    jpn: String,
    ger: String,
    fr: String,
    it: String,
    esp: String,
    kor: String,
    zh_hans: String,
    zh_hant: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Description {
    eng: String,
    jpn: String,
    ger: String,
    fr: String,
    it: String,
    esp: String,
    kor: String,
    zh_hans: String,
    zh_hant: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Species {
    eng: String,
    jpn: String,
    ger: String,
    fr: String,
    it: String,
    esp: String,
    kor: String,
    zh_hans: String,
    zh_hant: String,
}

impl Lang {
    pub fn new(
        name_eng: String, 
        name_jpn: String, 
        name_ger: String, 
        name_fr: String, 
        name_it: String, 
        name_esp: String, 
        name_kor: String, 
        name_zh_hans: String, 
        name_zh_hant: String,
        desc_eng: String, 
        desc_jpn: String, 
        desc_ger: String,
        desc_fr: String, 
        desc_it: String, 
        desc_esp: String,
        desc_kor: String, 
        desc_zh_hans: String, 
        desc_zh_hant: String,
        species_eng: String, 
        species_jpn: String, 
        species_ger: String, 
        species_fr: String, 
        species_it: String, 
        species_esp: String, 
        species_kor: String, 
        species_zh_hans: String, 
        species_zh_hant: String
    ) -> Lang {
        Lang { 
            name: Name::new(name_eng, name_jpn, name_ger, name_fr, name_it, name_esp, name_kor, name_zh_hans, name_zh_hant), 
            description: Description::new(desc_eng, desc_jpn, desc_ger, desc_fr, desc_it, desc_esp, desc_kor, desc_zh_hans, desc_zh_hant), 
            species: Species::new(species_eng, species_jpn, species_ger, species_fr, species_it, species_esp, species_kor, species_zh_hans, species_zh_hant) 
        }
    }
}

impl Name {
    pub fn new(
        name_eng: String, 
        name_jpn: String, 
        name_ger: String, 
        name_fr: String, 
        name_it: String, 
        name_esp: String, 
        name_kor: String, 
        name_zh_hans: String, 
        name_zh_hant: String
    ) -> Name {
        Name {
            eng: name_eng, 
            jpn: name_jpn, 
            ger: name_ger, 
            fr: name_fr, 
            it: name_it, 
            esp: name_esp, 
            kor: name_kor, 
            zh_hans: name_zh_hans, 
            zh_hant: name_zh_hant 
        }
    }
}

impl Description {
    pub fn new(
        desc_eng: String, 
        desc_jpn: String, 
        desc_ger: String,
        desc_fr: String, 
        desc_it: String, 
        desc_esp: String,
        desc_kor: String, 
        desc_zh_hans: String, 
        desc_zh_hant: String
    ) -> Description {
        Description { 
            eng: desc_eng, 
            jpn: desc_jpn, 
            ger: desc_ger, 
            fr: desc_fr, 
            it: desc_it, 
            esp: desc_esp, 
            kor: desc_kor, 
            zh_hans: desc_zh_hans, 
            zh_hant: desc_zh_hant 
        }
    }
}

impl Species {
    pub fn new(
        species_eng: String, 
        species_jpn: String, 
        species_ger: String, 
        species_fr: String, 
        species_it: String, 
        species_esp: String, 
        species_kor: String, 
        species_zh_hans: String, 
        species_zh_hant: String
    ) -> Species {
        Species { 
            eng: species_eng, 
            jpn: species_jpn, 
            ger: species_ger, 
            fr: species_fr, 
            it: species_it, 
            esp: species_esp, 
            kor: species_kor, 
            zh_hans: species_zh_hans, 
            zh_hant: species_zh_hant 
        }
    }
}
