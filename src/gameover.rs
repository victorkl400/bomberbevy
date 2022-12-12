use bevy::prelude::*;

use crate::{
    constants::{HEIGHT, WIDTH},
    GameState,
};

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct CloseButton;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_update(GameState::GameOver));
    }
}

/// We create a TextBundle with a Text that has a single section, and we set the alignment of the Text
/// to be centered. We then set the style of the TextBundle itself to be positioned at the bottom right
/// of the screen
///
/// Arguments:
///
/// * `commands`: Commands - This is the main way to spawn entities into the world.
/// * `asset_server`: Res<AssetServer>
fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texto = commands
        .spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Game \n Over !",
                TextStyle {
                    font: asset_server.load("fonts/Kenney-Future.ttf"),
                    font_size: 100.0,
                    color: Color::BLACK,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        ))
        .insert(GameOverUI)
        .id();

    //Spawn Menu Background image
    commands
        .spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(WIDTH), Val::Px(HEIGHT)),
                ..default()
            },
            image: asset_server.load("images/menu_bomberbevy.png").into(),
            ..default()
        })
        .add_child(texto)
        .insert(GameOverUI);
}
