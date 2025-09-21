use bevy::{
    prelude::*, render::camera::Viewport, platform::collections::HashMap,
};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;
