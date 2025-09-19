use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*, window::*};

#[derive(Component)]
struct Player_Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "gensokio".to_string(),
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                player_movement,
            ),
        )
        .run();
}

// Define the player movement system
fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let Ok((mut player, mut transform)) = player_query.single_mut() else { panic!() };

    let mut rotation_factor = 0.;
    let mut movement_factor = 1.;

    // Initialise the movement distance variable (to bring it into scope)
    let mut movement_distance: f32 = 0.;
    movement_distance = movement_factor * 1. * time.delta_secs();

    if keys.pressed(KeyCode::ArrowUp) {
        transform.translation.y += 5.;
    } else if keys.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= 5.;
    }

    if keys.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= 5.;
    } else if keys.pressed(KeyCode::ArrowRight) {
        transform.translation.x += 5.;
    }
}

// Bevy engine setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_handle = asset_server.load("reimu.png");

    // Spawn the 2D camera
    commands.spawn(Camera2d);

    // Spawn the player
    commands.spawn((
        Sprite::from_image(sprite_handle),
        Player { },
    ));
}
