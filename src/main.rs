use bevy::{
    prelude::*, render::camera::Viewport,
};

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Player {
    attack_speed: f32,
    cooldown: Timer,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "gensokio".to_string(),
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player_movement, confine_player, bullet_handling))
        .add_systems(Update, shoot_bullet)
        .run();
}

fn bullet_handling(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform), With<Bullet>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) {
    let Ok((camera, cam_transform)) = camera_query.single() else { panic!() };
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

fn shoot_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut player, transform)) = player_query.single_mut() {
        player.cooldown.tick(time.delta());

        if keys.pressed(KeyCode::KeyZ) && player.cooldown.finished() {
            let bullet_handle = asset_server.load("bullet-kunai.png");

            commands.spawn((
                Sprite::from_image(bullet_handle.clone()),
                Transform::from_xyz(transform.translation.x - 20., transform.translation.y + 60., 0.),
                Bullet,
            ));
            commands.spawn((
                Sprite::from_image(bullet_handle.clone()),
                Transform::from_xyz(transform.translation.x + 20., transform.translation.y + 60., 0.),
                Bullet,
            ));


            let attack_speed = player.attack_speed;
            player.cooldown.set_duration(std::time::Duration::from_secs_f32(1.0 / attack_speed));
            player.cooldown.reset();
        }
    }
}

// Define the player movement system
fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        panic!()
    };

    let mut translation = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowDown) {
        translation.y -= 6.;
    } else if keys.pressed(KeyCode::ArrowUp) {
        translation.y += 6.;
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        translation.x -= 6.;
    } else if keys.pressed(KeyCode::ArrowRight) {
        translation.x += 6.;
    }

    // Focus mode
    if keys.pressed(KeyCode::ShiftLeft) {
        translation *= Vec2::splat(0.6);
    }

    transform.translation.x += translation.x;
    transform.translation.y += translation.y;
}

fn confine_player(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let Ok((camera, cam_transform)) = camera_q.single() else {
        panic!()
    };
    let Ok(mut player_transform) = player_q.single_mut() else {
        panic!()
    };

    if let Some(viewport_size) = camera.logical_viewport_size() {
        let half_width = viewport_size.x / 2.0;
        let half_height = viewport_size.y / 2.0;

        let cam_pos = cam_transform.translation();

        let left_bound = cam_pos.x - half_width;
        let right_bound = cam_pos.x + half_width;
        let bottom_bound = cam_pos.y - half_height;
        let top_bound = cam_pos.y + half_height;

        player_transform.translation.x = player_transform.translation.x.clamp(left_bound, right_bound);
        player_transform.translation.y = player_transform.translation.y.clamp(bottom_bound, top_bound);
    }
}

// Bevy engine setup
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();
    let sprite_handle = asset_server.load("reimu.png");

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 {
                    x: 200,
                    y: ((window_size.y - 768.) / 2.) as u32,
                },
                physical_size: UVec2 {
                    x: 512,
                    y: 768,
                },
                ..default()
            }),
            clear_color: ClearColorConfig::Custom(Color::hsl(0., 0., 0.1)),
            ..default()
        },
    ));

    // Spawn the player
    commands.spawn((
        Sprite::from_image(sprite_handle),
        Player {
            attack_speed: 20.,
            cooldown: Timer::from_seconds(1.0 / 20., TimerMode::Repeating),
        },
    ));
}
