use bevy::{
    prelude::{default, AssetServer, Commands, Entity, Name, Quat, Query, Res, Transform, Vec3},
    scene::SceneBundle,
    time::Time,
};
use bevy_rapier3d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, RigidBody, Sensor};

use crate::{
    collider::InteractiveItem,
    constants::DEFAULT_OBJECT_SCALE,
    map::{Breakable, ObjectProps},
};

//---------------------------Map Helpers--------------------------//

pub fn spawn_object(
    commands: &mut Commands,
    object_props: &ObjectProps,
    asset_server: &AssetServer,
    translation: Vec3,
) -> Entity {
    //Make interactive objects bigger
    let scale = if object_props.interactive {
        DEFAULT_OBJECT_SCALE.to_owned() + Vec3::new(0.5, 0.5, 0.5)
    } else {
        DEFAULT_OBJECT_SCALE.to_owned()
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
            .insert(InteractiveItem)
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
/// * `translation`: Vec3,
/// * `default_floor`: If true, the floor will be a default floor. If false, the floor will be the floor
/// specified in the object_props.
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
    if object_props.breakable {
        object_spawn
            .insert(Breakable)
            .insert(RigidBody::KinematicPositionBased);
    }
    if object_props.interactive {
        //If interactive object, add collision events and make the collider smaller
        object_spawn
            .insert(Sensor)
            .insert(InteractiveItem)
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

/// "For each interactive item, rotate it around the y-axis."
///
/// The first line of the function is a query. A query is a way to get a list of entities that have
/// certain components. In this case, we're getting a list of entities that have both the
/// `InteractiveItem` and `Transform` components
///
/// Arguments:
///
/// * `item_query`: This is the query that will be used to get the components that we want to animate.
/// * `time`: Res<Time>
pub fn animate_interactive_items(
    mut item_query: Query<(&mut InteractiveItem, &mut Transform)>,
    time: Res<Time>,
) {
    for (_interactive_item, mut item_transform) in item_query.iter_mut() {
        item_transform.rotation = Quat::from_rotation_y(time.elapsed_seconds() as f32);
    }
}
