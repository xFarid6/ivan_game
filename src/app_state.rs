// use bevy::reflect::Reflect;
use bevy::{prelude::*, reflect::Enum};

use crate::Maps;



// ====== STRUCTS ======


#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash, States, Reflect)] // Should be deriving Reflect as well
pub enum AppState {
    Scene1,
    Scene2,
    Scene3,
    Scene4,
    Scene5,
    PauseMenu,
}

impl AppState {
    fn get_next_state(&self) -> Self {
        match self {
            AppState::Scene1 => AppState::Scene2,
            AppState::Scene2 => AppState::Scene3,
            AppState::Scene3 => AppState::Scene4,
            AppState::Scene4 => AppState::Scene5,
            AppState::Scene5 => AppState::Scene1,
            AppState::PauseMenu => AppState::Scene1,
        }
    }
}

#[derive(Debug, Resource)]
pub struct SceneStack(Vec<AppState>);

impl SceneStack {
    pub fn new(initial_state: AppState) -> Self {
        SceneStack(vec![initial_state]) // Start with the initial scene
    }

    pub fn push(&mut self, state: AppState) {
        self.0.push(state);  // Add a new scene to the top of the stack
    }

    pub fn pop(&mut self) -> Option<AppState> {
        self.0.pop()  // Remove and return the top scene from the stack
    }

    pub fn current(&self) -> Option<&AppState> {
        self.0.last()  // Get the scene currently at the top of the stack
    }
}

#[derive(Debug, Component)]
pub struct Scene1Entity;

#[derive(Debug, Component)]
pub struct Scene2Entity;

#[derive(Debug, Component)]
pub struct Scene3Entity;

#[derive(Debug, Component)]
pub struct Scene4Entity;

#[derive(Debug, Component)]
pub struct PauseMenuEntity;


// ====== METHODS ======


pub fn handle_scene_switch(
    mut scene_stack: ResMut<SceneStack>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Push a new scene onto the stack (e.g., PauseMenu) when 'P' is pressed
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        if let Some(current_scene) = scene_stack.current() {
            if *current_scene != AppState::PauseMenu {
                scene_stack.push(AppState::PauseMenu);
                next_state.set(AppState::PauseMenu);  // Switch to PauseMenu
                println!("Switched to PauseMenu");
            } else {
                scene_stack.pop();
                next_state.set(scene_stack.current().expect("Always something before the PauseMenu").clone());
                println!("Exiting the PauseMenu");
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyN) {
        let current_state = scene_stack.current().expect("App should always be in some state");
        let go_to_state = AppState::get_next_state(current_state);
        println!("Going to: {:#?}", go_to_state);
        scene_stack.push(go_to_state);
        next_state.set(go_to_state);
    }

    // Pop the current scene off the stack and return to the previous scene when 'R' is pressed
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if scene_stack.pop().is_some() {
            if let Some(previous_scene) = scene_stack.current() {
                // state.set(Box::new(previous_scene.clone())).unwrap() // Where state is: mut state: ResMut<State<AppState>>
                println!("Returned to {:?}", previous_scene);
                next_state.set(*previous_scene);  // Switch back to the previous scene
            } else {
                scene_stack.push(AppState::Scene1);
                next_state.set(AppState::Scene1);
            }
        } 
    }
}


/*
Clean up logic:
- Marker based -> common
- World clearing -> complete reset
- Overlaying content -> potential issues
*/

pub fn cleanup_scene1(
    mut commands: Commands, 
    query: Query<Entity, With<Scene1Entity>>
) {
    println!("Removing {:?} entities...", query.iter().len());
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    println!("Cleaned scene 1!");
}

pub fn cleanup_scene2(
    mut commands: Commands, 
    query: Query<Entity, With<Scene2Entity>>) {
    println!("Removing {:?} entities...", query.iter().len());
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    println!("Cleaned scene 2!");
}

pub fn make_invis_map_scene2(
    mut commands: Commands,
    mut maps: ResMut<Maps>,
    mut query: Query<&mut Visibility>,
) {
    let scene2_map_name = "Tiny_Swords".to_string(); 
    let scene2_map_layers = maps.get_map_layers(scene2_map_name);
    let layer_entities = scene2_map_layers.get_layers_ids();
    
    for entity in layer_entities.iter() {
        if let Ok(mut visibility) = query.get_mut(*entity) {
            *visibility = Visibility::Hidden; // Modify the visibility component
        }
    }
}

pub fn make_visible_map_scene2(
    mut commands: Commands,
    mut maps: ResMut<Maps>,
    mut query: Query<&mut Visibility>,
) {
    let scene2_map_name = "Tiny_Swords".to_string(); 
    let scene2_map_layers = maps.get_map_layers(scene2_map_name);
    let layer_entities = scene2_map_layers.get_layers_ids();
    
    for entity in layer_entities.iter() {
        if let Ok(mut visibility) = query.get_mut(*entity) {
            *visibility = Visibility::Visible; // Modify the visibility component
        }
    }
}

pub fn cleanup_scene3(
    mut commands: Commands, 
    query: Query<Entity, With<Scene3Entity>>
) {
    println!("Removing {:?} entities...", query.iter().len());
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    println!("Cleaned scene 3!");
}

pub fn cleanup_scene4(
    mut commands: Commands, 
    query: Query<Entity, With<Scene4Entity>>
) {
    println!("Removing {:?} entities...", query.iter().len());
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    println!("Cleaned scene 4!");
}

pub fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn clear_world_system(mut commands: Commands, mut entities: Query<Entity, (Without<Camera>, Without<Window>)>,) {
    // This removes all entities
    for entity_id in entities.iter_mut() {
        commands.entity(entity_id).despawn();
    }
}

// These are for testing purposes
pub fn some_weird_fn() {
    println!("I'm the weird function!");
}

pub fn my_placeholder_fn() {}

pub fn falsy_run_condition() -> bool {
    false
}

pub fn truthy_run_condition() -> bool {
    true
}

