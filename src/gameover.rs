use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::{app::AppExit, prelude::*};

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
                SystemSet::on_update(GameState::GameOver)
                    .with_system(any_button_pressed)
                    .with_system(close_button_clicked),
            );
    }
}

fn any_button_pressed(
    mut commands: Commands,
    mut key_evr: EventReader<KeyboardInput>,
    menu_root: Query<Entity, With<GameOverUI>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                let root_entity = menu_root.single();
                commands.entity(root_entity).despawn_recursive();

                game_state.set(GameState::Gameplay).unwrap();
            }
            ButtonState::Released => {}
        }
    }
}

fn close_button_clicked(
    interactions: Query<&Interaction, (With<CloseButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            exit.send(AppExit);
        }
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Spawn a close button
    let close_button = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Px(400.0)),
                align_self: AlignSelf::FlexStart,
                justify_content: JustifyContent::FlexStart,
                margin: UiRect::all(Val::Percent(2.0)),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        })
        .insert(CloseButton)
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
        .add_child(close_button)
        .insert(GameOverUI);
}
