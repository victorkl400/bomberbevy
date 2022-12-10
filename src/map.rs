use bevy_rapier3d::{
    geometry::Collider,
    prelude::{ActiveEvents, *},
    rapier::prelude::{ColliderBuilder, ColliderFlags, ColliderShape},
};
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
pub struct CustomProps {
    pub scale: Vec3,
    pub rotation: Quat,
    pub sum_translation: Vec3,
}
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectProps {
    pub name: String,
    pub add_floor: bool,
    pub path: String,
    pub is_floor: bool,
    pub interactive: bool,
    pub custom: Option<CustomProps>,
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
        if object_props.custom.is_some() {
            return spawn_custom(commands, object_props, asset_server, translation);
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
        //If interactive object, add collision events and make the collider smaller
        object_spawn
            .insert(Sensor)
            .insert(ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Collider::cuboid(0.3, 0.3, 0.3));
    } else {
        //If a normal object, default collider should be okay
        object_spawn.insert(Collider::cuboid(0.4, 0.4, 0.4));
    }
    object_spawn
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
        .insert(Name::new(format!("Floor#{}", object_props.name.clone())))
        .id()
}
fn spawn_custom(
    commands: &mut Commands,
    object_props: &ObjectProps,
    asset_server: &AssetServer,
    translation: Vec3,
) -> Entity {
    let custom_attributes = object_props.custom.clone().unwrap();
    let mut object_spawn = commands.spawn(SceneBundle {
        scene: asset_server.load(object_props.path.to_owned()),
        transform: Transform {
            translation: translation + custom_attributes.sum_translation,
            scale: custom_attributes.scale,
            ..default()
        },
        ..default()
    });

    //Spawn the custom object
    object_spawn
        .insert(Collider::cuboid(0.5, 0.5, 0.3))
        .insert(RigidBody::Fixed)
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
                custom: None,
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
                custom: None,
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
                custom: None,
                name: String::from("Block"),
            },
        ), //Block
        (
            67,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                interactive: true,
                path: "objects/sandwich.glb#Scene0".to_owned(),
                custom: None,
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
                custom: None,
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
                custom: None,
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
                custom: None,
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
                custom: None,
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
                custom: None,
                name: String::from("FloorStraight"),
            },
        ), //FloorStraight
        (
            64,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                interactive: false,
                path: "objects/workbench.glb#Scene0".to_owned(),
                custom: Some(CustomProps {
                    scale: Vec3::new(0.5, 0.5, 0.5),
                    rotation: Quat::from_rotation_y(0.0),
                    sum_translation: Vec3::new(0.0, 0.2, 0.0),
                }),
                name: String::from("Workbench"),
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
