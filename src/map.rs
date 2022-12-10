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
pub struct Breakable;

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
    pub breakable: bool,
    pub custom: Option<CustomProps>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_basic_map);
    }
}
/// If the object needs a floor, spawn a floor and the object. If it's a floor, spawn a floor. If it's a
/// custom object, spawn a custom object. If it's a normal object, spawn a normal object
///
/// Arguments:
///
/// * `commands`: The commands that will be used to spawn the entity.
/// * `asset_server`: The asset server that we created in the previous section.
/// * `_object_types`: This is a HashMap of all the objects that can be spawned.
/// * `char_key`: The key of the object to be spawned.
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
/// It spawns an object with a collider and rigid body, and if it's interactive, it also adds a sensor
/// and collision events
///
/// Arguments:
///
/// * `commands`: &mut Commands,
/// * `object_props`: The object properties that we want to spawn
/// * `asset_server`: The asset server that will load the object's model
/// * `default_scale`: The default scale of the object.
/// * `translation`: The position of the object
///
/// Returns:
///
/// The entity id of the spawned object
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
/// It spawns a floor entity with the given properties
///
/// Arguments:
///
/// * `commands`: &mut Commands,
/// * `object_props`: This is the object properties that we get from the JSON file.
/// * `asset_server`: The asset server that will load the object.
/// * `default_scale`: The default scale of the object.
/// * `translation`: Vec3,
/// * `default_floor`: If true, the floor will be a default floor. If false, the floor will be the floor
/// specified in the object_props.
///
/// Returns:
///
/// Entity
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

/// It spawns a custom object, which is a 3D model that is loaded from a file
///
/// Arguments:
///
/// * `commands`: &mut Commands,
/// * `object_props`: This is the object properties that we're going to use to spawn the object.
/// * `asset_server`: The asset server that will load the object
/// * `translation`: The position of the object
///
/// Returns:
///
/// The entity id of the spawned object
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
    if object_props.breakable {
        object_spawn.insert(Breakable);
    }
    //Spawn the custom object
    object_spawn
        .insert(Collider::cuboid(0.5, 0.5, 0.3))
        .insert(RigidBody::Fixed)
        .insert(Name::new(object_props.name.clone()))
        .id()
}

/// We open the map file, create a hashmap that maps each character to a set of properties, and then for
/// each character in the map file, we spawn an object with the properties of the character
///
/// Arguments:
///
/// * `commands`: Commands is a struct that allows you to add entities to the game.
/// * `asset_server`: This is the asset server that we'll use to load the assets.
fn create_basic_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file = File::open("assets/maps/level1.txt").expect("No map found");
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
                breakable: false,
                name: String::from("Floor"),
            },
        ), //Floor
        (
            87,
            ObjectProps {
                add_floor: false,
                is_floor: false,
                interactive: false,
                path: "objects/towerSquare_sampleE.glb#Scene0".to_owned(),
                custom: None,
                breakable: false,
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
                breakable: false,
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
                breakable: false,
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
                breakable: false,
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
                breakable: false,
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
                breakable: false,
                name: String::from("TowerA"),
            },
        ), //TowerA
        (
            36,
            ObjectProps {
                add_floor: false,
                is_floor: false,
                interactive: false,
                path: "objects/towerSquare_sampleE.glb#Scene0".to_owned(),
                custom: None,
                breakable: false,
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
                breakable: false,
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
