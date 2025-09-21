use bevy::prelude::*;
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
