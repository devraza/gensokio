use bevy::{prelude::*, render::camera::Viewport};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

pub type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

// Load modules from other files
mod network;
mod player;
mod bullet;
use crate::network::*;
use crate::player::*;
use crate::bullet::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    prevent_default_event_handling: false,
                    title: "gensokio".to_string(),
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<Config>::default(),
        ))
        .add_systems(Startup, (setup, spawn_player, start_matchbox_socket))
        .add_systems(FixedUpdate, (confine_player, bullet_handling))
        .add_systems(Update, (shoot_bullet, wait_for_players))
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, player_movement)
        .run();
}

// Bevy engine setup
fn setup(mut commands: Commands, window: Single<&Window>) {
    let window_size = window.resolution.physical_size().as_vec2();

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 {
                    x: 200,
                    y: ((window_size.y - 768.) / 2.) as u32,
                },
                physical_size: UVec2 { x: 512, y: 768 },
                ..default()
            }),
            clear_color: ClearColorConfig::Custom(Color::hsl(0., 0., 0.1)),
            ..default()
        },
    ));
}
