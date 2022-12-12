use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::Inspectable;

use crate::{
    collider::UpgradeType,
    utils::{spawn_custom, spawn_floor, spawn_object},
    GameState,
};

pub struct MapPlugin;

#[derive(Component, Inspectable)]
pub struct Breakable;

#[derive(Component, Inspectable)]
pub struct AnimatedRotation;

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
    pub upgrade: UpgradeType,
    pub animated_rotation: bool,
    pub breakable: bool,
    pub custom: Option<CustomProps>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(create_basic_map));
    }
}

/// It spawns a map object
///
/// Arguments:
///
/// * `commands`: The commands that will be used to spawn the object.
/// * `asset_server`: The asset server that we created in the previous section.
/// * `_object_types`: A HashMap of all the object types that can be spawned.
/// * `char_key`: The key of the object to spawn.
/// * `translation`: The position of the object
///
/// Returns:
///
/// Entity
pub fn spawn_map_object(
    commands: &mut Commands,
    asset_server: &AssetServer,
    _object_types: &HashMap<i32, ObjectProps>,
    char_key: i32,
    translation: Vec3,
) -> Entity {
    let object_props = _object_types.get(&char_key).unwrap();

    //If floor is needed , spawn floor and the object
    if object_props.add_floor {
        //Spawn default floor
        spawn_floor(commands, object_props, asset_server, translation, true);
        if object_props.custom.is_some() {
            return spawn_custom(commands, object_props, asset_server, translation);
        }
        //Spawn Object
        return spawn_object(commands, object_props, asset_server, translation);
    }
    //If is a floor, custom collider
    if object_props.is_floor {
        //Spawn custom Floor
        return spawn_floor(commands, object_props, asset_server, translation, false);
    }
    //Spawn Object
    return spawn_object(commands, object_props, asset_server, translation);
}
/// It reads a text file and spawns the objects in the scene based on the characters in the text file
///
/// Arguments:
///
/// * `commands`: Commands,
/// * `asset_server`: Res<AssetServer> - This is the asset server that we will use to load the assets.

fn create_basic_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file = File::open("assets/maps/level1.txt").expect("No map found");
    //Hashmap that maps each character index and relates to the rendering
    let object_types = HashMap::from([
        (
            32,
            ObjectProps {
                add_floor: false,
                is_floor: true,
                upgrade: UpgradeType::None,
                path: "objects/tile.glb#Scene0".to_owned(),
                custom: None,
                animated_rotation: false,
                breakable: false,
                name: String::from("Floor"),
            },
        ), //Floor
        (
            87,
            ObjectProps {
                add_floor: false,
                is_floor: false,
                upgrade: UpgradeType::None,
                path: "objects/towerSquare_sampleE.glb#Scene0".to_owned(),
                custom: None,
                animated_rotation: false,
                breakable: false,
                name: String::from("Wall"),
            },
        ), //Wall
        (
            66,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                upgrade: UpgradeType::None,
                path: "objects/towerSquare_middleA.glb#Scene0".to_owned(),
                custom: None,
                animated_rotation: false,
                breakable: false,
                name: String::from("Block"),
            },
        ), //Block
        (
            67,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                upgrade: UpgradeType::Speed,
                path: "objects/speedup.glb#Scene0".to_owned(),
                custom: Some(CustomProps {
                    scale: Vec3::new(0.1, 0.3, 0.2),
                    rotation: Quat::from_rotation_y(0.0),
                    sum_translation: Vec3::new(0.0, 0.5, 0.0),
                }),
                animated_rotation: true,
                breakable: true,
                name: String::from("Coin"),
            },
        ), //Coin
        (
            82,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                upgrade: UpgradeType::None,
                path: "objects/towerSquare_sampleE.glb#Scene0".to_owned(),
                custom: None,
                animated_rotation: false,
                breakable: false,
                name: String::from("RoundedWall"),
            },
        ), //Rounded Wall
        (
            35,
            ObjectProps {
                add_floor: false,
                is_floor: false,
                upgrade: UpgradeType::None,
                path: "objects/towerSquare_sampleA.glb#Scene0".to_owned(),
                custom: None,
                animated_rotation: false,
                breakable: false,
                name: String::from("TowerA"),
            },
        ), //TowerA
        (
            36,
            ObjectProps {
                add_floor: false,
                is_floor: false,
                upgrade: UpgradeType::None,
                path: "objects/towerSquare_sampleE.glb#Scene0".to_owned(),
                custom: None,
                animated_rotation: false,
                breakable: false,
                name: String::from("TowerC"),
            },
        ), //TowerC
        (
            37,
            ObjectProps {
                add_floor: false,
                is_floor: true,
                upgrade: UpgradeType::None,
                path: "objects/tile_straight.glb#Scene0".to_owned(),
                custom: None,
                animated_rotation: false,
                breakable: false,
                name: String::from("FloorStraight"),
            },
        ), //FloorStraight
        (
            64,
            ObjectProps {
                add_floor: true,
                is_floor: false,
                upgrade: UpgradeType::None,
                path: "objects/workbench.glb#Scene0".to_owned(),
                custom: Some(CustomProps {
                    scale: Vec3::new(0.5, 0.5, 0.6),
                    rotation: Quat::from_rotation_y(0.0),
                    sum_translation: Vec3::new(0.0, 0.2, 0.0),
                }),
                animated_rotation: false,
                breakable: true,
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
