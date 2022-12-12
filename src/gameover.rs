use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

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
            .add_system_set(
                SystemSet::on_update(GameState::GameOver).with_system(any_button_pressed),
            );
    }
}
/// When any button is pressed, despawn the menu UI and set the game state to `Gameplay`
///
/// Arguments:
///
/// * `commands`: Commands is a struct that allows you to create, delete, and modify entities.
/// * `key_evr`: EventReader<KeyboardInput>
/// * `menu_root`: Query<Entity, With<MenuUI>>
/// * `game_state`: The game state resource.
fn any_button_pressed(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    game_over_root: Query<Entity, With<GameOverUI>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                let root_entity = game_over_root.single();
                commands.entity(root_entity).despawn_recursive();

                game_state.set(GameState::Menu).unwrap();
            }
            ButtonState::Released => {}
        }
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
    //Spawn Menu Background image
    commands
        .spawn(ImageBundle {
            style: Style {
                size: Size::new(Val::Px(WIDTH), Val::Px(HEIGHT)),
                ..default()
            },
            image: asset_server.load("images/gameover_bomberbevy.png").into(),
            ..default()
        })
        .insert(GameOverUI);
}
