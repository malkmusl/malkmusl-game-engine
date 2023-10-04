#![allow(dead_code)]
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Ident {
    dex_id: DexId,
    type_: Type,
    evolution: Evolution,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DexId {
    generation: u32,
    national: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Type {
    main: String,
    secondary: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Evolution {
    method: String,
    to: String,
    condition: String,
}

impl Ident {
    pub fn new(
        generation: u32, 
        national_dex: u32, 
        main_type: String, 
        secondary_type: String, 
        method: String, 
        evolution: String, 
        condition: String
    ) -> Ident {
        Ident { 
            dex_id: DexId::new(generation, national_dex), 
            type_: Type ::new(main_type, secondary_type), 
            evolution: Evolution::new(method, evolution, condition)
        }
    }
}

impl DexId {
    pub fn new(
        generation: u32, 
        national: u32
    ) -> DexId {
        DexId { 
            generation: generation, 
            national: national 
        }
    }
}

impl Type {
    pub fn new(
        main_type: String, 
        secondary_type: String
    ) -> Type{
        Type { 
            main: main_type, 
            secondary: secondary_type
        }
    }
}

impl Evolution {
    pub fn new(
        method: String, 
        evolution: String, 
        condition: String
    ) -> Evolution {
        Evolution { 
            method: method,
            to: evolution, 
            condition: condition 
        }
    }
}