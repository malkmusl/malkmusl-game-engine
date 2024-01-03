use crate::engine::assets_loader::loader::ASSET_FOLDER;
use crate::engine::core::{app::App, metadata::DEBUG};
use crate::engine::core::world::map_gen::{self, read_base_map_file};



pub fn main() {
    //dir_exist();
    //list_files();
    //map_test();

    let app = App::new("Test Game", "0.0.1-alpha", 800, 600, DEBUG);
    app.run();

}

fn map_test(){
    let path = format!("{}/basemap.ron", ASSET_FOLDER);
    
    map_gen::generate_base_map_file(&path, 15, 15);

    match read_base_map_file(&path) {
        Ok(tiles) => {
            println!("Successfully read the base map file.");
            for tile in tiles {
                println!("{:?}", tile);
            }
        }
        Err(err) => eprintln!("Error reading base map file: {}", err),
    }
}
