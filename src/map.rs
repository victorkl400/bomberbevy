use bevy_rapier3d::{geometry::Collider, prelude::*};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

pub struct MapPlugin;

#[derive(Component, Inspectable)]
pub struct ObjectCollider {}
pub struct ObjectProps {
    pub name: String,
    pub add_floor: bool,
    pub path: String,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_basic_map);
    }
}
pub fn spawn_map_object(
    commands: &mut Commands,
    asset_server: &AssetServer,
    char_key: i32,
    translation: Vec3,
) -> Entity {
    //Hashmap that maps each character index and relates to the rendering
    let _object_types = HashMap::from([
        (
            32,
            ObjectProps {
                add_floor: false,
                path: "objects/tile.glb#Scene0".to_owned(),
                name: String::from("Floor"),
            },
        ), //Floor
        (
            87,
            ObjectProps {
                add_floor: false,
                path: "objects/towerSquare_bottomC.glb#Scene0".to_owned(),
                name: String::from("Wall"),
            },
        ), //Wall
        (
            66,
            ObjectProps {
                add_floor: true,
                path: "objects/towerSquare_middleA.glb#Scene0".to_owned(),
                name: String::from("Block"),
            },
        ), //Block
        (
            67,
            ObjectProps {
                add_floor: true,
                path: "objects/detail_crystal.glb#Scene0".to_owned(),
                name: String::from("Coin"),
            },
        ), //Coin
        (
            82,
            ObjectProps {
                add_floor: true,
                path: "objects/towerSquare_sampleE.glb#Scene0".to_owned(),
                name: String::from("RoundedWall"),
            },
        ), //Rounded Wall
        (
            33,
            ObjectProps {
                add_floor: false,
                path: "objects/tile_dirt.glb#Scene0".to_owned(),
                name: String::from("FloorDirt"),
            },
        ), //Floor Dirt
        (
            35,
            ObjectProps {
                add_floor: false,
                path: "objects/towerSquare_sampleA.glb#Scene0".to_owned(),
                name: String::from("FloorDirt"),
            },
        ), //Floor Dirt
        (
            36,
            ObjectProps {
                add_floor: false,
                path: "objects/towerSquare_bottomC.glb#Scene0".to_owned(),
                name: String::from("FloorDirt"),
            },
        ), //Floor Dirt
        (
            37,
            ObjectProps {
                add_floor: false,
                path: "objects/tile_straight.glb#Scene0".to_owned(),
                name: String::from("FloorDirt"),
            },
        ), //Floor Dirt
    ]);

    let default_scale = Vec3::new(0.57, 1., 0.57);

    let object_props = _object_types.get(&char_key).unwrap();
    //If floor is needed , spawn floor and the object
    if object_props.add_floor {
        //Spawn floor
        commands
            .spawn(SceneBundle {
                scene: asset_server.load("objects/tile.glb#Scene0"),
                transform: Transform {
                    translation: translation,
                    scale: default_scale,
                    ..Default::default()
                },
                ..default()
            })
            .insert(Name::new("Floor"));
        //Spawn Object
        commands
            .spawn(SceneBundle {
                scene: asset_server.load(object_props.path.to_owned()),
                transform: Transform {
                    translation: Vec3::new(translation.x, translation.y + 0.1, translation.z),
                    scale: default_scale,
                    ..Default::default()
                },
                ..default()
            })
            .insert(Collider::cuboid(
                default_scale.x,
                default_scale.y,
                default_scale.z,
            ))
            .insert(Name::new(object_props.name.clone()))
            .id()
    } else if char_key == 32 || char_key == 33 || char_key == 37 {
        //Spawn Floor
        commands
            .spawn(SceneBundle {
                scene: asset_server.load(object_props.path.to_owned()),
                transform: Transform {
                    translation: translation,
                    scale: default_scale,
                    ..Default::default()
                },
                ..default()
            })
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(0.4, 0.4, 0.4))
            .insert(Name::new(object_props.name.clone()))
            .id()
    } else {
        //Spawn Object
        commands
            .spawn(SceneBundle {
                scene: asset_server.load(object_props.path.to_owned()),
                transform: Transform {
                    translation: translation,
                    scale: default_scale,
                    ..Default::default()
                },
                ..default()
            })
            .insert(Collider::cuboid(
                default_scale.x,
                default_scale.y,
                default_scale.z,
            ))
            .insert(RigidBody::Fixed)
            .insert(Name::new(object_props.name.clone()))
            .id()
    }
}

fn create_basic_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file = File::open("assets/maps/level3.txt").expect("No map found");

    for (z, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                spawn_map_object(
                    &mut commands,
                    &asset_server,
                    char as i32,
                    Vec3::new(x as f32 / 2. - 6., 0.0, z as f32 / 2. - 4.),
                );
            }
        }
    }
}
