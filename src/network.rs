use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

pub const INPUT_UP: u8 = 1 << 0;
pub const INPUT_DOWN: u8 = 1 << 1;
pub const INPUT_LEFT: u8 = 1 << 2;
pub const INPUT_RIGHT: u8 = 1 << 3;
pub const INPUT_FIRE: u8 = 1 << 4;

use crate::Config;

pub fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket>) {
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

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/gensokio?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}
