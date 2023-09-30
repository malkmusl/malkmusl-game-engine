extern crate lazy_static;

use super::texture_loader::TextureAtlas;
use lazy_static::lazy_static;

// Lazy-static instances of TextureAtlas, organized alphabetically for better readability.
lazy_static! {
    // TextureAtlas for the interior of the Bike Shop.
    pub static ref BIKE_SHOP_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "bike_shop_interior", [32, 32]);

    // TextureAtlas for the Boat tileset.
    pub static ref BOAT_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "boat", [32, 32]);

    // TextureAtlas for the Caves environment.
    pub static ref CAVES_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "caves", [32, 32]);

    // TextureAtlas for the interior of the Department Store.
    pub static ref DEPARTMENT_STORE_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "department_store_interior", [32, 32]);

    // TextureAtlas for the Dungeon Cave environment.
    pub static ref DUNGEON_CAVE_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "dungeon_cave", [32, 32]);

    // TextureAtlas for the Dungeon Forest environment.
    pub static ref DUNGEON_FOREST_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "dungeon_forest", [32, 32]);

    // TextureAtlas for the interior of the Factory.
    pub static ref FACTORY_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "factory_interior", [32, 32]);

    // TextureAtlas for the interior of the Game Corner.
    pub static ref GAME_CORNER_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "game_corner_interior", [32, 32]);

    // TextureAtlas for the interior of the Graveyard Tower.
    pub static ref GRAVEYARD_TOWER_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "graveyard_tower_interior", [32, 32]);

    // TextureAtlas for the interior of Gyms.
    pub static ref GYMS_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "gyms_interior", [32, 32]);

    // TextureAtlas for the interior of the Harbour.
    pub static ref HARBOUR_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "harbour_interior", [32, 32]);

    // TextureAtlas for general interior tiles.
    pub static ref INTERIOR_GENERAL_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "interior_general", [32, 32]);

    // TextureAtlas for the interior of Mansions.
    pub static ref MANSION_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "mansion_interior", [32, 32]);

    // TextureAtlas for the interior of Marts.
    pub static ref MART_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "mart_interior", [32, 32]);

    // TextureAtlas for Multiplayer Rooms.
    pub static ref MULTIPLAYER_ROOMS_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "multiplayer_rooms", [32, 32]);

    // TextureAtlas for the interior of the Museum.
    pub static ref MUSEUM_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "museum_interior", [32, 32]);

    // TextureAtlas for the interior of Poke Centres.
    pub static ref POKE_CENTRE_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "poke_centre_interior", [32, 32]);

    // TextureAtlas for the interior of Ruins.
    pub static ref RUINS_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "ruins_interior", [32, 32]);

    // TextureAtlas for the interior of Trainer Towers.
    pub static ref TRAINER_TOWER_INTERIOR_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "trainer_tower_interior", [32, 32]);

    // TextureAtlas for the Underground Path.
    pub static ref UNDERGROUND_PATH_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "underground_path", [32, 32]);

    // TextureAtlas for the Underwater environment.
    pub static ref UNDERWATER_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "underwater", [32, 32]);

    // TextureAtlas for the Outside environment.
    pub static ref OUTSIDE_ATLAS: TextureAtlas = TextureAtlas::new("textures/tilesets/", "outside", [32, 32]);
}

