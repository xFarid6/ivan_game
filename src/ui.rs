use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

use crate::Gravity;

// ====== STRUCTS ======
// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct GravityText;

// ====== METHODS ======

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}

pub fn gravity_text_update_system(
    mut query: Query<&mut Text, With<GravityText>>,
    gravity: Res<Gravity>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!("Gravity values: {:.2}, {:.2}", gravity.x, gravity.y);
    }
}