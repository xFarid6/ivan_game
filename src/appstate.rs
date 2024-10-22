// use bevy::reflect::Reflect;
use bevy::prelude::*;



// ====== STRUCTS ======


#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Reflect)] // Should be deriving Reflect as well
pub enum AppState {
    Scene1,
    Scene2,
    PauseMenu,
}



#[derive(Debug, Resource)]
pub struct SceneStack(Vec<AppState>);

#[derive(Debug, Component)]
pub struct Scene1Entity;

#[derive(Debug, Component)]
pub struct PauseMenuEntity;


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
            }
        }
    }

    // Pop the current scene off the stack and return to the previous scene when 'R' is pressed
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        if scene_stack.pop().is_some() {
            if let Some(previous_scene) = scene_stack.current() {
                // state.set(Box::new(previous_scene.clone())).unwrap() // Where state is: mut state: ResMut<State<AppState>>
                next_state.set(previous_scene.clone());  // Switch back to the previous scene
                println!("Returned to {:?}", previous_scene);
            }
        }
    }
}


pub fn cleanup_scene1(mut commands: Commands, query: Query<Entity, With<Scene1Entity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn some_weird_fn() {
    println!("I'm the weird function!");
}

pub fn falsy_run_condition() -> bool {
    false
}

pub fn truthy_run_condition() -> bool {
    true
}

