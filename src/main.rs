use bevy::{
    prelude::*, render::camera::Viewport, platform::collections::HashMap,
};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Player {
    attack_speed: f32,
    handle: usize,
    cooldown: Timer,
}

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

// Spawn player outside of initial setup
fn spawn_player(mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sprite_handle = asset_server.load("reimu.png");

    commands.spawn((
        Sprite::from_image(sprite_handle.clone()),
        Player {
            attack_speed: 20.,
            handle: 0,
            cooldown: Timer::from_seconds(1.0 / 20., TimerMode::Repeating),
        },
    )).add_rollback();

    commands.spawn((
        Sprite::from_image(sprite_handle.clone()),
        Player {
            attack_speed: 20.,
            handle: 1,
            cooldown: Timer::from_seconds(1.0 / 20., TimerMode::Repeating),
        },
    )).add_rollback();
}

fn read_local_inputs(
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

fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket>) {
    if socket.get_channel(0).is_err() {
        return;
    }

    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    info!("All peers have joined, going in-game");

    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let channel = socket.take_channel(0).unwrap();

    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
}


fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/gensokio?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

// Define the player movement system
fn player_movement(
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

fn confine_player(
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

// Bevy engine setup
fn setup(
    mut commands: Commands,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();

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
}
