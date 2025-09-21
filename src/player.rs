use bevy::{prelude::*, platform::collections::HashMap};
use bevy_ggrs::*;

#[derive(Component)]
pub struct Player {
    pub attack_speed: f32,
    pub handle: usize,
    pub cooldown: Timer,
}

// Load modules from the crate
use crate::Config;
use crate::network::*;

// Spawn player outside of initial setup
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_handle = asset_server.load("reimu.png");

    commands
        .spawn((
            Sprite::from_image(sprite_handle.clone()),
            Player {
                attack_speed: 20.,
                handle: 0,
                cooldown: Timer::from_seconds(1.0 / 20., TimerMode::Repeating),
            },
        ))
        .add_rollback();

    commands
        .spawn((
            Sprite::from_image(sprite_handle.clone()),
            Player {
                attack_speed: 20.,
                handle: 1,
                cooldown: Timer::from_seconds(1.0 / 20., TimerMode::Repeating),
            },
        ))
        .add_rollback();
}

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = 0u8;

        if keys.pressed(KeyCode::ArrowUp) {
            input |= INPUT_UP;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            input |= INPUT_DOWN;
        }
        if keys.pressed(KeyCode::ArrowLeft) {
            input |= INPUT_LEFT
        }
        if keys.pressed(KeyCode::ArrowRight) {
            input |= INPUT_RIGHT;
        }
        if keys.pressed(KeyCode::KeyZ) {
            input |= INPUT_FIRE;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}

// Define the player movement system
pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    inputs: Res<PlayerInputs<Config>>,
    mut players: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let mut translation = Vec2::ZERO;

        let movement_speed = 6.;

        if input & INPUT_UP != 0 {
            translation.y += 1.;
        }
        if input & INPUT_DOWN != 0 {
            translation.y -= 1.;
        }
        if input & INPUT_LEFT != 0 {
            translation.x -= 1.;
        }
        if input & INPUT_RIGHT != 0 {
            translation.x += 1.;
        }

        // Focus mode
        if keys.pressed(KeyCode::ShiftLeft) {
            translation *= Vec2::splat(0.6);
        }

        let movement_delta = (translation * movement_speed).extend(0.);

        transform.translation += movement_delta;
    }
}

pub fn confine_player(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut players: Query<(&mut Transform, &Player)>,
) {
    let Ok((camera, cam_transform)) = camera_q.single() else {
        panic!()
    };

    for (mut transform, _) in &mut players {
        if let Some(viewport_size) = camera.logical_viewport_size() {
            let half_width = viewport_size.x / 2.0;
            let half_height = viewport_size.y / 2.0;

            let cam_pos = cam_transform.translation();

            let left_bound = cam_pos.x - half_width;
            let right_bound = cam_pos.x + half_width;
            let bottom_bound = cam_pos.y - half_height;
            let top_bound = cam_pos.y + half_height;

            transform.translation.x = transform.translation.x.clamp(left_bound, right_bound);
            transform.translation.y = transform.translation.y.clamp(bottom_bound, top_bound);
        }
    }
}
