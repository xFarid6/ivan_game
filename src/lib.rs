// COMPILER DIRECTIVES
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

// Modules
mod app_utils;
mod constants;
mod ui;
mod player;
mod particles;
mod cards;
mod app_state;
mod tilemaps;
mod buttons;
mod pendulum;
// mod scene4;
mod scene1;

pub mod server;
pub mod client;
mod networking;

// Bevy
use bevy::{
    color::palettes::css::PURPLE, core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    }, 
    diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, 
    render::mesh::Mesh, sprite::MaterialMesh2dBundle, winit::WinitSettings
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_tilemap::prelude::*;

// Std rust libraries
use rand::Rng;
use std::{collections::HashMap, fs, time::Duration};
use std::env;

// Other libraries

// Personal imports
use app_utils::*;
use constants::*;
use particles::*;
use player::*;
use ui::*;
use cards::*;
use app_state::*;
use tilemaps::*;
use buttons::*;
use pendulum::*;
// use scene4::*;
use scene1::*;

use client::*;
use networking::*;



// SYSTEM SETS

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyWeirdSet; // Sets are just a reference name, not an actual grouping

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct Scene1Set;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct Scene2Set;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct Scene3Set;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct Scene4Set;

// GAME ENTRY POINT
pub fn run_game() {
    // env::set_var("RUST_BACKTRACE", "1");
    App::new()
        // PLUGINS
        .add_plugins(
            (
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Something other than \"Bevy App\"".to_string(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                    .set(ImagePlugin::default_nearest()), 
                FrameTimeDiagnosticsPlugin,
                TilemapPlugin
            ))
        .add_plugins(WorldInspectorPlugin::new())

        // RESOURCES - must be initialized after the Default Plugins (else weird crashes happen)
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(
                1000,
            )),
        })
        // .insert_resource(WinitSettings::desktop_app())
        .insert_resource(CardHandles { cards_map: HashMap::new() } )
        .insert_resource(SceneStack::new(AppState::Scene4))  // TODO: Start with Scene 1
        .insert_resource(Maps::new())
        .insert_state(AppState::Scene4) // TODO: Match above state


        // SYSTEM CONFIGURATIONS    
        .configure_sets
        (
            Startup, 
            (
                MyWeirdSet.run_if(in_state(AppState::Scene1)),
                Scene1Set.run_if(in_state(AppState::Scene1))
            )
        )
        .configure_sets
        (
            Update, 
            (
                MyWeirdSet.run_if(in_state(AppState::Scene1)), // Configured to be able to run but not called
                Scene1Set.run_if(in_state(AppState::Scene1)),
                Scene3Set.run_if(in_state(AppState::Scene3)),
                Scene4Set.run_if(in_state(AppState::Scene4)),
            )
        )
        .configure_sets
        (
            FixedUpdate, 
            (
                MyWeirdSet.run_if(in_state(AppState::Scene1)), // Configured to be able to run but not called
                Scene1Set.run_if(in_state(AppState::Scene1)),
                Scene2Set.run_if(in_state(AppState::Scene2)),
                Scene4Set.run_if(in_state(AppState::Scene4)),
            )
        )

        
        // ADD SYSTEMS
        // special schedules (generated on State existance)
        .add_systems(OnEnter(AppState::Scene1), (
            world_setup_scene1, spawn_emitter_scene1, ui_setup_scene1
        ))
        .add_systems(OnExit(AppState::Scene1), cleanup_scene1)

        .add_systems(OnEnter(AppState::Scene2), (
            make_visible_map_scene2, // tilemaps_setup,
        ))
        .add_systems(OnExit(AppState::Scene2), (
            make_invis_map_scene2, cleanup_scene2
        ))
        .add_systems(OnEnter(AppState::Scene3), (
            load_slimes, toggle_visibility_system,
        ))
        .add_systems(OnExit(AppState::Scene3), (
            cleanup_scene3, toggle_visibility_system
        ))
        .add_systems(OnEnter(AppState::Scene4), (
            setup_pendulum, setup_double_pendulum,
        ))
        .add_systems(OnExit(AppState::Scene4), cleanup_scene4)

        .add_systems(OnEnter(AppState::PauseMenu), (
            my_placeholder_fn,
        ))
        .add_systems(OnExit(AppState::PauseMenu), cleanup_pause_menu)

        // normal schedules
        .add_systems
        (
            Startup, 
            (
                assets_setup, world_setup, button_setup,
                (tilemaps_setup, make_invis_map_scene2).chain(),
                (some_weird_fn, some_weird_fn).in_set(MyWeirdSet),
            )
        )
        .add_systems
        (
            Update, 
            (
                handle_scene_switch, // one time event, oneshot system
                (fps_text_update_system, 
                gravity_text_update_system,
                update_bloom_settings,).in_set(Scene1Set),
                (button_system, execute_animations, spawn_slimes_system, update_slime_position).in_set(Scene3Set),
                
            ),
        )
        .add_systems
        (
            FixedUpdate,
            (
                close_on_esc,
                (
                    (
                        window_walls,
                        reposition_ball_on_mouse_click,
                        apply_gravity,
                        apply_velocity,
                    ).chain(),
                    (add_ball_random_pos, remove_temp_balls),
                    change_gravity,
                    (draw_a_line_example, draw_xy_axis, draw_cursor),
                    (translate_everything_on_window_move, move_camera_on_mouse_wheel),
                    (
                        emitter_system_scene1, particle_movement_system,
                        particle_lifetime_system, particle_fade_system, 
                        particle_size_scaling_system, particle_gravity_system,
                        move_emitter_sin_wave, draw_path
                    ).chain(),
                    spawn_random_card,
                ).in_set(Scene1Set),
                (camera_movement_scene2).in_set(Scene2Set),
                (
                    update_pendulum_system, draw_pendulum_system, draw_pendulum_trace,
                    update_pendulum_system_rk4, draw_double_pendulum_system
                ).in_set(Scene4Set),
            ),
        )

        .run();
}

fn world_setup(mut commands: Commands,) {
    // Camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    // Gravity
    commands.insert_resource(Gravity { x: 0., y: -1. });

    // Init LastWindowPos Resource
    // commands.insert_resource(LastWindowPos{ x: 0, y: 0 });
    commands.init_resource::<LastWindowPos>();
}



fn assets_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    card_handles: ResMut<CardHandles>
) {
    let icon_handle: Handle<Image> = asset_server.load("icon.png");
    let stars_handle: Handle<Image> = asset_server.load("particles/star.png");

    load_cards_pngs(asset_server, card_handles);

    // Store the material handle as a resource
    commands.insert_resource(ParticleMaterialHandle(stars_handle));
}


