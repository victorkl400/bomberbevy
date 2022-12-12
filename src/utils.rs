use bevy::{
    prelude::{default, AssetServer, Commands, Entity, Name, Quat, Query, Res, Transform, Vec3},
    scene::SceneBundle,
    time::Time,
};
use bevy_rapier3d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, RigidBody, Sensor};
use rand::Rng;

use crate::{
    collider::{InteractiveItem, UpgradeType},
    constants::DEFAULT_OBJECT_SCALE,
    map::{AnimatedRotation, Breakable, CustomProps, ObjectProps},
};

//---------------------------Map Helpers--------------------------//

/// It spawns an object with a collider, rigid body, and name
///
/// Arguments:
///
/// * `commands`: &mut Commands,
/// * `object_props`: This is the ObjectProps struct that we defined earlier.
/// * `asset_server`: The asset server that will load the object's mesh
/// * `translation`: The position of the object
///
/// Returns:
///
/// The entity id of the spawned object
pub fn spawn_object(
    commands: &mut Commands,
    object_props: &ObjectProps,
    asset_server: &AssetServer,
    translation: Vec3,
) -> Entity {
    //Make upgrade objects bigger
    let scale = if object_props.upgrade != UpgradeType::None {
        DEFAULT_OBJECT_SCALE.to_owned() + Vec3::new(0.5, 0.5, 0.5)
    } else {
        DEFAULT_OBJECT_SCALE.to_owned()
    };
    //Make upgrade objects floating
    let translation = if object_props.upgrade != UpgradeType::None {
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
    if object_props.upgrade != UpgradeType::None {
        //If upgrade object, add collision events and make the collider smaller
        object_spawn
            .insert(Sensor)
            .insert(InteractiveItem {
                upgrade: UpgradeType::Fire,
            })
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

/// It takes in a bunch of stuff, and returns an entity ID
///
/// Arguments:
///
/// * `commands`: &mut Commands,
/// * `object_props`: This is the ObjectProps struct that we defined earlier.
/// * `asset_server`: The asset server that will load the object.
/// * `translation`: Vec3 - The position of the object
/// * `default_floor`: If true, the floor will be spawned with the default floor model.
///
/// Returns:
///
/// Entity
pub fn spawn_floor(
    commands: &mut Commands,
    object_props: &ObjectProps,
    asset_server: &AssetServer,
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
                scale: DEFAULT_OBJECT_SCALE.to_owned(),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1., 0.2, 1.))
        .insert(Name::new(format!("Floor#{}", object_props.name.clone())))
        .id()
}

/// It spawns a custom object, which is a 3D model, and adds the necessary components to it
///
/// Arguments:
///
/// * `commands`: &mut Commands - This is the command buffer that we will use to spawn the object.
/// * `object_props`: The object properties that are defined in the level file.
/// * `asset_server`: The asset server that will load the object
/// * `translation`: The position of the object
///
/// Returns:
///
/// The entity id of the spawned object
pub fn spawn_custom(
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
    if object_props.animated_rotation {
        object_spawn.insert(AnimatedRotation);
    }
    if object_props.breakable {
        object_spawn
            .insert(Breakable)
            .insert(RigidBody::KinematicPositionBased);
    }
    if object_props.upgrade != UpgradeType::None {
        //If upgrade object, add collision events and make the collider smaller
        object_spawn
            .insert(Sensor)
            .insert(InteractiveItem {
                upgrade: object_props.upgrade,
            })
            .insert(ActiveCollisionTypes::KINEMATIC_STATIC)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Collider::cuboid(0.3, 0.3, 0.3))
            .insert(RigidBody::Fixed);
    } else {
        object_spawn.insert(Collider::cuboid(0.5, 0.5, 0.3));
    }
    //Spawn the custom object
    object_spawn
        .insert(Name::new(object_props.name.clone()))
        .id()
}

//---------------------------Items Helpers--------------------------//

/// "For each upgrade item, rotate it around the y axis."
///
/// The first line of the function is a query. A query is a way to get a list of entities that have
/// certain components. In this case, we're getting a list of entities that have both the
/// `InteractiveItem` and `Transform` components
///
/// Arguments:
///
/// * `item_query`: Query<(&mut InteractiveItem, &mut Transform)>
/// * `time`: Res<Time>
pub fn animate_interactive_items(
    mut item_query: Query<(&mut AnimatedRotation, &mut Transform)>,
    time: Res<Time>,
) {
    for (_interactive_item, mut item_transform) in item_query.iter_mut() {
        item_transform.rotation = Quat::from_rotation_y(time.elapsed_seconds() * 2 as f32);
    }
}

pub fn possibly_spawn_upgrade(
    commands: &mut Commands,
    asset_server: &AssetServer,
    translation: Vec3,
) {
    //Possibly spawn an item

    //Get a random value between 0 and 100
    let random_value = rand::thread_rng().gen_range(0..100);

    let upgrade_to_spawn;
    let upgrade_name;
    let upgrade_type;

    //Since 20% or 20 numbers between 0 and 100 are possible
    //we divide the 20 number into 4 options within a range of 5 numbers each
    if random_value >= 0 && random_value <= 5 {
        upgrade_to_spawn = "objects/fireup.glb#Scene0";
        upgrade_name = "FireUp";
        upgrade_type = UpgradeType::Fire;
    } else if random_value >= 5 && random_value <= 15 {
        //More chance to get bomb upgrade
        upgrade_to_spawn = "objects/bombup.glb#Scene0";
        upgrade_name = "BombUp";
        upgrade_type = UpgradeType::Bomb;
    } else {
        upgrade_to_spawn = "objects/speedup.glb#Scene0";
        upgrade_name = "SpeedUp";
        upgrade_type = UpgradeType::Speed;
    };
    if random_value >= 0 && random_value <= 20 {
        let object_props = ObjectProps {
            add_floor: true,
            is_floor: false,
            upgrade: upgrade_type,
            path: upgrade_to_spawn.to_owned(),
            custom: Some(CustomProps {
                scale: Vec3::new(0.2, 0.3, 0.2),
                rotation: Quat::from_rotation_y(0.0),
                sum_translation: Vec3::ZERO,
            }),
            animated_rotation: true,
            breakable: true,
            name: String::from(upgrade_name),
        };

        let upgrade = spawn_custom(commands, &object_props, &asset_server, translation);
    }
}
