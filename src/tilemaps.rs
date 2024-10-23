use serde::Deserialize;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde_json::from_str;
use std::fs;

#[derive(Deserialize, Debug)]
struct TilemapData {
    tile_size: u32,
    map_width: u32,
    map_height: u32,
    layers: Vec<Layer>,
}

#[derive(Deserialize, Debug)]
struct Layer {
    name: String,
    tiles: Vec<Tile>,
}

#[derive(Deserialize, Debug)]
struct Tile {
    id: String,  // Use String if IDs are not guaranteed to be numbers
    x: u32,
    y: u32,
}

#[derive(Deserialize, Debug)]
struct Tileset {
    firstgid: u32,
    image: String,  // Path to the spritesheet image
    tilewidth: u32,
    tileheight: u32,
    columns: u32,
}

fn tilemaps_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load the JSON tilemap file
    let tilemap_json = fs::read_to_string("assets/tilemap.json")
        .expect("Could not load tilemap file");
    let tilemap_data: TilemapData = from_str(&tilemap_json)
        .expect("Could not parse tilemap JSON");

    // Load the spritesheet (make sure to provide the correct path)
    let texture_handle = asset_server.load("path/to/your/spritesheet.png");

    // Create a texture atlas from the spritesheet
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(tilemap_data.tile_size as f32, tilemap_data.tile_size as f32),
        // Specify the number of columns and rows if you know them; adjust as necessary
        10, // Number of columns
        10, // Number of rows (adjust based on your spritesheet)
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Set up the tilemap
    let map_size = TilemapSize {
        x: tilemap_data.map_width,
        y: tilemap_data.map_height,
    };
    let tile_size = TilemapTileSize {
        x: tilemap_data.tile_size as f32,
        y: tilemap_data.tile_size as f32,
    };
    let grid_size = Vec2::new(tile_size.x, tile_size.y);

    let mut tile_storage = TileStorage::empty(map_size);

    // Fill the tilemap with data from the JSON
    for layer in tilemap_data.layers.iter() {
        for tile in layer.tiles.iter() {
            let tile_id: usize = tile.id.parse().unwrap(); // Assuming IDs are numbers

            // Create the position based on x and y
            let tile_pos = TilePos { x: tile.x, y: tile.y };

            // Create the tile bundle
            let tile_bundle = TileBundle {
                position: tile_pos,
                texture_index: TileTextureIndex(tile_id), // Using the ID as the index
                ..Default::default()
            };
            tile_storage.set(&tile_pos, tile_bundle);
        }
    }

    // Spawn the tilemap into the Bevy world
    commands.spawn(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: texture_atlas_handle,
        tile_size,
        ..Default::default()
    });
}
