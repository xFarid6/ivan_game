use bevy::prelude::*;

// =========== STRUCTS ===========

#[derive(Resource, Debug)]
pub struct LastWindowPos {
    pub x: i32,
    pub y: i32,
}

impl Default for LastWindowPos {
    fn default() -> Self {
        LastWindowPos { x: 0, y: 0 }
    }
}

impl LastWindowPos {
    pub fn get(&self) -> IVec2 {
        IVec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn is_default(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn set(&mut self, pos: IVec2) {
        self.x = pos.x;
        self.y = pos.y;
    }
}

// =========== METHODS ===========

// Close the focused window whenever the escape key (<kbd>Esc</kbd>) is pressed
//
// This is useful for examples or prototyping.
pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

pub fn getwindowsize(window: Query<&Window>) -> (f32, f32) {
    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    // dbg!(width, height, x, y);
    (width, height)
}

pub fn window_to_world(position: Vec2, window: &Window, camera: &Transform) -> Vec3 {
    // Center in screen space
    let norm = Vec3::new(
        position.x - window.width() / 2.,
        position.y - window.height() / 2.,
        0.,
    );

    // Apply camera transform
    *camera * norm

    // Alternatively:
    //camera.mul_vec3(norm)
}

/// If available, gets the x and y of the bevy window on the screen
pub fn get_window_coordinates(window: Query<&Window>) -> (i32, i32) {
    let window = window
        .get_single()
        .expect("The number of queries for &Window was not 1");
    // Get current window coordinates
    let mut current_w_x: i32 = 0;
    let mut current_w_y: i32 = 0;
    match window.position {
        WindowPosition::Automatic => {} // println!("Window position: Automatic (system defined)"),
        WindowPosition::Centered(_monitor) => {} // println!("Window position: {:?}", monitor),
        WindowPosition::At(pos) => {
            current_w_x = pos.x;
            current_w_y = pos.y;
        } // println!("Window position: At x: {}, y: {}", pos.x, pos.y),
    }

    (current_w_x, current_w_y)
}

/// Any Entity with a Transform (except for the Camera) is moved in the opposite way 
/// of any window movement (click and drag)
/// Note: dragging the window on the screen prevents it from updating for the duration
/// of the dragging motion
pub fn translate_everything_on_window_move(
    mut win_move: EventReader<WindowMoved>,
    mut last_win_pos: ResMut<LastWindowPos>,
    mut transforms: Query<&mut Transform, Without<Camera>>,
) {
    for ev in win_move.read() {
        // Set the value for the first time
        if last_win_pos.is_default() {
            last_win_pos.set(ev.position);
            return;
        }

        // Get difference between last window position and current one
        let move_on_x = ev.position.x - last_win_pos.x;
        let move_on_y = ev.position.y - last_win_pos.y;

        // Move everything by that difference
        for mut item in &mut transforms {
            item.translation.x -= move_on_x as f32;
            item.translation.y += move_on_y as f32;
        }

        // Update the LastWinPos
        last_win_pos.set(ev.position);
    }
}

// Move the camera when moving the window
// WindowMoved - An event that is sent when a window is repositioned in physical pixels.
// WindowResized - A window event that is sent whenever a window’s logical size has changed.

// fn window_resized_event(mut events: EventReader<WindowResized>) {
//    for event in events.iter() {
//        // resize window
//    }
// }
// Works but breaks the ball_to_mouse logic
pub fn camera_move_on_window_move(
    window: Query<&Window>,
    mut last_win_pos: ResMut<LastWindowPos>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    /*
    Manual Dragging: Bevy, by default, does not detect when a window is dragged manually
    by the user outside of Bevy's API. If you need to track such changes,
    you'd need to use platform-specific solutions
    (e.g., using raw window handles and interfacing with system APIs).
    */

    // Can we raelly use this super useful method on window.position and that's it?!
    // .is_changed()

    let window_s = window.single();
    // println!("{:?}", window.position);

    // Basically if the window hasn't ever been moved this function will **return early**
    match window_s.position {
        WindowPosition::Automatic => return,
        WindowPosition::Centered(_monitor_selection) => return,
        WindowPosition::At(ivec2) => {
            if ivec2 == last_win_pos.get() {
                return;
            }
        }
    }

    // Get current window coordinates
    let (current_w_x, current_w_y) = get_window_coordinates(window);

    // Calculate the shift
    let last_w_x = last_win_pos.x;
    let last_w_y = last_win_pos.y;

    let move_on_x = current_w_x - last_w_x;
    let move_on_y = current_w_y - last_w_y;

    // Reposition the camera
    let mut camera_transform = camera.single_mut();

    camera_transform.translation = Vec3 {
        x: camera_transform.translation.x + move_on_x as f32,
        y: camera_transform.translation.y - move_on_y as f32,
        ..camera_transform.translation
    };

    // Update the LastWinPos Resource
    last_win_pos.x = current_w_x;
    last_win_pos.y = current_w_y;
}