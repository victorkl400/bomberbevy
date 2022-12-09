use bevy_rapier3d::{geometry::Collider, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

pub struct MapPlugin;

#[derive(Component, Inspectable)]
pub struct ObjectCollider {}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectProps {
    pub name: String,
    pub add_floor: bool,
    pub path: String,
    pub is_floor: bool,
    pub interactive: bool,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_basic_map);
    }
}
pub fn spawn_map_object(
    commands: &mut Commands,
    asset_server: &AssetServer,
    _object_types: &HashMap<i32, ObjectProps>,
    char_key: i32,
    translation: Vec3,
) -> Entity {
    let default_scale = Vec3::new(0.57, 1., 0.57);

    let object_props = _object_types.get(&char_key).unwrap();

    //If floor is needed , spawn floor and the object
    if object_props.add_floor {
        //Spawn default floor
        spawn_floor(
            commands,
            object_props,
            asset_server,
            default_scale,
            translation,
            true,
        );
        //Spawn Object
        return spawn_object(
            commands,
            object_props,
            asset_server,
            default_scale,
            translation,
        );
    }
    //If is a floor, custom collider
    if object_props.is_floor {
        //Spawn custom Floor
        return spawn_floor(
            commands,
            object_props,
            asset_server,
            default_scale,
            translation,
            false,
        );
    }
    //Spawn Object
    return spawn_object(
        commands,
        object_props,
        asset_server,
        default_scale,
        translation,
    );
}
fn spawn_object(
    commands: &mut Commands,
    object_props: &ObjectProps,
    asset_server: &AssetServer,
    default_scale: Vec3,
    translation: Vec3,
) -> Entity {
    //Make interactive objects bigger
    let scale = if object_props.interactive {
        default_scale + Vec3::new(0.5, 0.5, 0.5)
    } else {
        default_scale
    };
    //Make interactive objects floating
    let translation = if object_props.interactive {
        Vec3::new(translation.x, translation.y + 0.5, translation.z)
    } else {
        Vec3::new(translation.x, translation.y + 0.1, translation.z)
    };

    let mut object_spawn = commands.spawn(SceneBundle {
        scene: asset_server.load(object_props.path.to_owned()),
        transform: Transform {
            translation: translation,
            scale: scale,
            ..default()
        },
        ..default()
    });
    if object_props.interactive {
        object_spawn.insert(Sensor);
    }
    object_spawn
        .insert(Collider::cuboid(0.5, 0.5, 0.3))
        .insert(RigidBody::Fixed)
        .insert(Name::new(object_props.name.clone()))
        .id()
}
fn spawn_floor(
    commands: &mut Commands,
    object_props: &ObjectProps,
    asset_server: &AssetServer,
    default_scale: Vec3,
    translation: Vec3,
    default_floor: bool,
) -> Entity {
    let path = if !default_floor {
        object_props.path.clone().to_string()
    } else {
        "objects/tile.glb#Scene0".to_string()
    };
    commands
        .spawn(SceneBundle {
            scene: asset_server.load(path),
            transform: Transform {
                translation: translation,
                scale: default_scale,
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1., 0.2, 1.))
        .insert(Name::new(object_props.name.clone()))
        .id()
}
fn create_basic_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file = File::open("assets/maps/level3.txt").expect("No map found");
    //Hashmap that maps each character index and relates to the rendering
    let object_types = HashMap::from([
        (
            32,
            ObjectProps {
                add_floor: false,
                is_floor: true,
                interactive: false,
                path: "objects/tile.glb#Scene0".to_owned(),
                name: String::from("Floor"),
            },
        ), //Floor
        (
            87,
            ObjectProps {
                add_floor: false,
                is_floor: false,
                interactive: false,
                path: "objects/towerSquare_bottomC.glb#Scene0".to_owned(),
                name: String::from("Wall"),
            },
        ), //Wall
        (
            66,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                interactive: false,
                path: "objects/towerSquare_middleA.glb#Scene0".to_owned(),
                name: String::from("Block"),
            },
        ), //Block
        (
            67,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                interactive: true,
                path: "objects/detail_crystal.glb#Scene0".to_owned(),
                name: String::from("Coin"),
            },
        ), //Coin
        (
            82,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                interactive: false,
                path: "objects/towerSquare_sampleE.glb#Scene0".to_owned(),
                name: String::from("RoundedWall"),
            },
        ), //Rounded Wall
        (
            33,
            ObjectProps {
                add_floor: false,
                is_floor: true,
                interactive: false,
                path: "objects/tile_dirt.glb#Scene0".to_owned(),
                name: String::from("FloorDirt"),
            },
        ), //Floor Dirt
        (
            35,
            ObjectProps {
                add_floor: false,
                is_floor: false,
                interactive: false,
                path: "objects/towerSquare_sampleA.glb#Scene0".to_owned(),
                name: String::from("TowerA"),
            },
        ), //TowerA
        (
            36,
            ObjectProps {
                add_floor: false,
                is_floor: false,
                interactive: false,
                path: "objects/towerSquare_bottomC.glb#Scene0".to_owned(),
                name: String::from("TowerC"),
            },
        ), //TowerC
        (
            37,
            ObjectProps {
                add_floor: false,
                is_floor: true,
                interactive: false,
                path: "objects/tile_straight.glb#Scene0".to_owned(),
                name: String::from("FloorStraight"),
            },
        ), //FloorStraight
    ]);
    for (z, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                spawn_map_object(
                    &mut commands,
                    &asset_server,
                    &object_types,
                    char as i32,
                    Vec3::new(x as f32 / 2. - 6., 0.0, z as f32 / 2. - 4.),
                );
            }
        }
    }
}
