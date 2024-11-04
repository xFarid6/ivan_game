use bevy::ecs::query;
use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use bevy::transform::commands;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Debug, Component)]
pub struct Scene3Bundle;


pub fn button_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        (NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        visibility: Visibility::Hidden,
        ..default()
        }, Scene3Bundle))
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: Default::default(),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                        },),
                    );
                });
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn toggle_visibility_system(
    mut query: Query<&mut Visibility, With<Node>>,
) {
    for mut visibility in query.iter_mut() {
        *visibility = match *visibility {
            Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Visible,
            Visibility::Inherited => Visibility::Inherited,
        };
    }
}
