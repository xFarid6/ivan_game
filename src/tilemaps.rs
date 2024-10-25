use serde::Deserialize;
use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_tilemap::prelude::*;
use serde_json::from_str;
use std::{collections::HashSet, fs};
use image::image_dimensions;


// ====== STRUCTS ======

#[derive(Deserialize, Debug)]
pub struct TilemapData {
    tile_size: u32,
    map_width: u32,
    map_height: u32,
    layers: Vec<Layer>,
}

#[derive(Deserialize, Debug)]
pub struct Layer {
    name: String,
    tiles: Vec<Tile>,
    collider: bool,
}

#[derive(Deserialize, Debug)]
pub struct Tile {
    id: String,  // Use String if IDs are not guaranteed to be numbers
    x: u32,
    y: u32,
}

#[derive(Debug, Component)]
struct NamedLayer(String);

// These next two structs actually hold a lot of duplicated data. Could be better. 
#[derive(Debug, Resource)]
pub struct Maps {
    pub map_names: Vec<String>,
    pub maps: HashMap<String, MapLayersData>
}

#[derive(Debug)]
pub struct MapLayersData {
    layer_numbers: HashMap<String, u32>,
    layer_data: HashMap<String, (Entity, TileStorage)>,
}


impl Maps {
    pub fn new() -> Self {
        Maps {
            map_names: Vec::new(),
            maps: HashMap::new()
        }
    }

    fn add_map_name(&mut self, name: String) {
        self.map_names.push(name);
    }

    fn add_new_map(&mut self, map_name: String, map_layers_data: MapLayersData) {
        self.add_map_name(map_name.clone());

        match self.maps.insert(map_name, map_layers_data) {
            Some(old_value) => {
                println!("Map was already registered! It has now been UPDATED.")
            },
            None => {
                println!("New map data has been inserted correctly");
            },
        }
    }

    pub fn get_map_layers(&self, map_name: String) -> &MapLayersData {
        self.maps.get(&map_name).expect("Map name didn't match any KEYS")
    }
}


impl MapLayersData {
    pub fn new() -> Self {
        Self {
            layer_numbers: HashMap::new(),
            layer_data: HashMap::new()
        }
    }

    pub fn add_layer(&mut self, layer_name: String, layer_index: u32) {
        match self.layer_numbers.insert(layer_name.clone(), layer_index) {
            Some(old_layer) => {
                println!("Layer was already in the map! It has been UPDATED.");
                println!("Old layer was: {:?}", old_layer);
                println!("New layer is: {:?}", (layer_name, layer_index));
            },
            None => {
                println!("Correclty inserted layer: {:?} with index {:?}", layer_name, layer_index);
            },
        }
    }

    pub fn add_data(&mut self, layer_name: String, tilemap_components: (Entity, TileStorage)) {
        match self.layer_data.insert(layer_name, tilemap_components) {
            Some(old_data) => {
                println!("A layer with the same name was already in the map, and now it has been UPDATED");
            },
            None => {
                println!("New map data for this layer inserted correctly");
            },
        }
    }

    pub fn get_layers_ids(&self) -> Vec<Entity> {
        let mut res = Vec::new();
        for (id, _) in self.layer_data.values() {
            res.push(*id);
        }
        res
    }
}


// ====== METHODS ======

#[deprecated(since="24/10's commit", note="this never actually worked")]
fn tilemaps_setup_no(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the JSON tilemap file -> TilemapData struct
    let tilemap_json = fs::read_to_string("assets/maps/Tiny_Swords/map.json")
        .expect("Could not load tilemap file");
    let tilemap_data: TilemapData = from_str(&tilemap_json)
        .expect("Could not parse tilemap JSON");

    // Load the spritesheet (make sure to provide the correct path)
    let texture_handle: Handle<Image> = asset_server.load("maps/Tiny_Swords/spritesheet.png");

    // Create a texture atlas layout for the spritesheet
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2 { x: tilemap_data.tile_size, y: tilemap_data.tile_size }, 
        8, 
        25, 
        Some(UVec2::new(0, 0)), 
        None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas_layout.clone());

    // Set up the tilemap
    let map_size = TilemapSize {
        x: tilemap_data.map_width,// * tilemap_data.layers.len() as u32,
        y: tilemap_data.map_height,// * tilemap_data.layers.len() as u32,
    };
    let tile_size = TilemapTileSize {
        x: tilemap_data.tile_size as f32,
        y: tilemap_data.tile_size as f32,
    };
    let grid_size = TilemapGridSize::new(tile_size.x * 8., tile_size.y * 25.); // TODO: check for this multiplication to be necessary
    // Vec2::new(tile_size.x * tilemap_data.map_width as f32, tile_size.y * tilemap_data.map_height as f32);

    let mut tile_storage = TileStorage::empty(map_size);
    // assert_eq!(tile_storage.size, map_size, "Tile storage size does not match the map size.");

    // Use a HashSet to track occupied positions for each layer
    // let mut occupied_positions_per_layer = vec![HashSet::new(); tilemap_data.layers.len()];
    let mut occupied_positions = HashSet::new();
    let mut tile_storage = TileStorage::empty(map_size);

    // Fill the tilemap with data from the JSON
    for (layer_index, layer) in tilemap_data.layers.iter().enumerate() {
        let z_layer = layer_index as u32; // Use a small float offset for each layer

        for tile in layer.tiles.iter() {
            let tile_id: usize = tile.id.parse().expect("Failed to parse the tile ID into a number"); // Assuming IDs are numbers
            let texture_index = TileTextureIndex(tile_id.try_into().expect("tile_id usize cannot be into-ed to u32")); // Using the ID as the index

            if tile_id >= texture_atlas_layout.len() {
                panic!("Tile ID {} exceeds the texture atlas size.", tile_id);
            }
            
            let tile_pos = TilePos { x: tile.x, y: tile.y };
            // Check for overlap (within the same layer)
            if !occupied_positions.insert((tile_pos, z_layer)) {
                panic!("Duplicate tile position detected at {:?}", tile_pos);
            }
            if tile_pos.x >= map_size.x || tile_pos.y >= map_size.y {
                panic!("Tile position out of bounds: {:?}", tile_pos);
            }

            // Spawn the tile entity
            let tile_entity_id = commands.spawn(
                TileBundle {
                    position: tile_pos,
                    texture_index: texture_index,
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity_id);
        }

        commands.spawn(TilemapBundle {
            // Size of the tiles on the grid in pixels. This can be used to overlay tiles on top of each other. 
            // Ex. A 16x16 pixel tile can be overlapped by 8 pixels by using a grid size of 16x8.
            grid_size: grid_size, 
            size: map_size,
            storage: tile_storage.clone(), // is this big enough? does it take into account the layers?
            texture: TilemapTexture::Single(texture_handle.clone()),
            tile_size: tile_size,
            ..Default::default()
        });
    }

    // Spawn the tilemap into the Bevy world
    // commands.spawn(TilemapBundle {
    //     // Size of the tiles on the grid in pixels. This can be used to overlay tiles on top of each other. 
    //     // Ex. A 16x16 pixel tile can be overlapped by 8 pixels by using a grid size of 16x8.
    //     grid_size: grid_size, 
    //     size: map_size,
    //     storage: tile_storage, // is this big enough? does it take into account the layers?
    //     texture: TilemapTexture::Single(texture_handle),
    //     tile_size: tile_size,
    //     ..Default::default()
    // });
}

#[deprecated(since="24/10's commit", note="this never worked eighter. They're here for reference.")]
fn tilemaps_setup_no_2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the JSON tilemap file -> TilemapData struct
    let tilemap_json = fs::read_to_string("assets/maps/Tiny_Swords/map.json")
        .expect("Could not load tilemap file");
    let tilemap_data: TilemapData = from_str(&tilemap_json)
        .expect("Could not parse tilemap JSON");

    // Load the spritesheet (make sure to provide the correct path)
    let texture_handle: Handle<Image> = asset_server.load("maps/Tiny_Swords/spritesheet.png");

    // Create a texture atlas layout for the spritesheet
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2 { x: tilemap_data.tile_size, y: tilemap_data.tile_size }, 
        8, 
        24, 
        Some(UVec2::new(0, 0)), 
        None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas_layout.clone());

    // Set up the tilemap
    let map_size = TilemapSize {
        x: tilemap_data.map_width,
        y: tilemap_data.map_height,
    };
    let tile_size = TilemapTileSize {
        x: tilemap_data.tile_size as f32,
        y: tilemap_data.tile_size as f32,
    };
    let grid_size = TilemapGridSize::new(tile_size.x, tile_size.y);

    // Use a HashSet to track occupied positions for each layer
    let mut occupied_positions_per_layer = vec![HashSet::new(); tilemap_data.layers.len()];

    let mut tile_storage = TileStorage::empty(map_size);
    // Fill the tilemap with data from the JSON
    for (layer_index, layer) in tilemap_data.layers.iter().enumerate() {
        if layer_index > 0 {
            continue;
        }
        for tile in layer.tiles.iter() {
            let tile_id: usize = tile.id.parse().expect("Failed to parse the tile ID into a number");
            let texture_index = TileTextureIndex(tile_id.try_into().expect("tile_id usize cannot be into-ed to u32"));

            if tile_id >= texture_atlas_layout.len() {
                panic!("Tile ID {} exceeds the texture atlas size.", tile_id);
            }

            let tile_pos = TilePos { x: tile.x, y: tile.y };
            
            // Check for overlap only within the same layer
            if !occupied_positions_per_layer[layer_index].insert(tile_pos) {
                panic!("Duplicate tile position detected at {:?} in layer {}", tile_pos, layer_index);
            }

            if tile_pos.x >= map_size.x || tile_pos.y >= map_size.y {
                panic!("Tile position out of bounds: {:?}", tile_pos);
            }

            // Spawn the tile entity
            let tile_entity_id = commands.spawn(
                TileBundle {
                    position: tile_pos,
                    texture_index: texture_index,
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity_id);
        }

    }
    // After processing each layer, update the tile storage for that layer
    commands.spawn(TilemapBundle {
        grid_size: grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size: tile_size,
        ..Default::default()
    });
}

pub fn tilemaps_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut all_maps: ResMut<Maps>
) {
    let map_name = "Tiny_Swords".to_string();

    let spritesheet_path = "maps/".to_string() + &map_name + "/spritesheet.png";
    let tilemap_json = fs::read_to_string("assets/maps/".to_string() + &map_name + "/map.json")
        .expect("Could not load tilemap file");
    let tilemap_data: TilemapData = from_str(&tilemap_json)
        .expect("Could not parse tilemap JSON");    
    let texture_handle: Handle<Image> = asset_server
        .load(spritesheet_path.clone());
    let (img_x, img_y) = image_dimensions("assets/".to_owned() + &spritesheet_path)
        .expect("Image dimensions were not readable");


    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2 { x: tilemap_data.tile_size, y: tilemap_data.tile_size }, 
        img_x / tilemap_data.tile_size, // only works when there's no padding and no offset (i think)
        img_y / tilemap_data.tile_size, 
        None, 
        None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas_layout.clone());

    let map_size = TilemapSize {
        x: tilemap_data.map_width,
        y: tilemap_data.map_height,
    };
    let tile_size = TilemapTileSize {
        x: tilemap_data.tile_size as f32,
        y: tilemap_data.tile_size as f32,
    };
    let grid_size = tile_size.into(); // TilemapGridSize::new(tile_size.x, tile_size.y);
    let map_type = TilemapType::Square;

    
    let mut occupied_positions_per_layer = vec![HashSet::new(); tilemap_data.layers.len()];
    let mut map_layers_data = MapLayersData::new();

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for (layer_index, layer) in tilemap_data.layers.iter().rev().enumerate() {
        // if layer_index > 3 { continue; }
                
        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(map_size);
        let layer_name = layer.name.clone();

        for tile in layer.tiles.iter() {
            let tile_id: u32 = tile.id.parse()
                .expect("Failed to parse the tile ID into a number");
            let texture_index = TileTextureIndex(tile_id);
            let tile_pos = TilePos { 
                x: tile.x, 
                y: map_size.y - 1 - tile.y // Invert the Y-axis 
            };

            if tile_id >= texture_atlas_layout.len() as u32 {
                panic!("Tile ID {} exceeds the texture atlas size.", tile_id);
            }
            if tile_pos.x >= map_size.x || tile_pos.y >= map_size.y {
                panic!("Tile position out of bounds: {:?}", tile_pos);
            }
            if !occupied_positions_per_layer[layer_index].insert(tile_pos) {
                panic!("Duplicate tile position detected at {:?} in layer {}", tile_pos, layer_index);
            }

            let tile_entity = commands.spawn(
                TileBundle {
                    position: tile_pos,
                    texture_index,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }

        // Set Z-index based on layer index to control draw order
        let z_index = layer_index as f32;
        let centered_transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);

        commands.entity(tilemap_entity).insert(
            (
                TilemapBundle 
                {
                    grid_size,
                    map_type,
                    size: map_size,
                    storage: tile_storage.clone(),
                    texture: TilemapTexture::Single(texture_handle.clone()),
                    tile_size,
                    transform: Transform { 
                        translation: Vec3 { 
                            x: centered_transform.translation.x, 
                            y: centered_transform.translation.x, 
                            z: z_index // All this just to change the z index and order the layers
                        }, 
                        ..centered_transform
                    },
                    ..Default::default()
                },
                NamedLayer(layer_name.clone())
            )
        );

        // Keep track of the "small" maps (single layer of a map)
        
        map_layers_data.add_layer(layer_name.clone(), layer_index as u32);
        map_layers_data.add_data(layer_name, (tilemap_entity, tile_storage));
    }

    // Keep track of the full maps
    all_maps.add_new_map(map_name, map_layers_data);

    println!("Doing it all");
}

pub fn camera_movement_scene2(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

