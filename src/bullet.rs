use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet;

// Load modules from the crate
use crate::Player;

pub fn bullet_handling(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform), With<Bullet>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) {
    let Ok((camera, cam_transform)) = camera_query.single() else {
        panic!()
    };
    let cam_pos = cam_transform.translation();

    if let Some(viewport_size) = camera.logical_viewport_size() {
        let half_height = viewport_size.y / 2.0;
        let top_bound = cam_pos.y + half_height;

        for (entity, mut transform) in bullet_query.iter_mut() {
            transform.translation.y += 1500. * time.delta_secs();

            if transform.translation.y > top_bound {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn shoot_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut player, transform)) = player_query.single_mut() {
        player.cooldown.tick(time.delta());

        if keys.pressed(KeyCode::KeyZ) && player.cooldown.finished() {
            let bullet_handle = asset_server.load("textures/bullet-kunai.png");

            commands.spawn((
                Sprite::from_image(bullet_handle.clone()),
                Transform::from_xyz(
                    transform.translation.x - 20.,
                    transform.translation.y + 60.,
                    0.,
                ),
                Bullet,
            ));
            commands.spawn((
                Sprite::from_image(bullet_handle.clone()),
                Transform::from_xyz(
                    transform.translation.x + 20.,
                    transform.translation.y + 60.,
                    0.,
                ),
                Bullet,
            ));

            let attack_speed = player.attack_speed;
            player
                .cooldown
                .set_duration(std::time::Duration::from_secs_f32(1.0 / attack_speed));
            player.cooldown.reset();
        }
    }
}
