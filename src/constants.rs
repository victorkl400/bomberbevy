use bevy::{
    prelude::{Quat, Vec3},
    utils::HashMap,
};

use crate::map::{CustomProps, ObjectProps};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

//Bomb
pub const BOMB_SPAWN_DELAY: u64 = 350;
pub const BOMB_EXPLOSTION_TIME: u64 = 3;

//Audio
pub const SFX_AUDIO_CHANNEL: &str = "sfx";

//Map
pub const DEFAULT_OBJECT_SCALE: &Vec3 = &Vec3::new(0.57, 1., 0.57);
