use bevy::{
    core_pipeline::tonemapping::Tonemapping, prelude::*, render::camera::Viewport, window::*,
};

#[derive(Component)]
struct Player_Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player {
    velocity: Vec2,
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
        .add_systems(FixedUpdate, (player_movement, confine_player))
        .run();
}

// Define the player movement system
fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let Ok((mut player, mut transform)) = player_query.single_mut() else {
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
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let window = windows.single();

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
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
            velocity: Vec2::ZERO,
        },
    ));
}
