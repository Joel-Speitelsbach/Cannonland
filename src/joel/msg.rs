// module info: data types for sending messages between server and client
#![allow(dead_code,unused_variables,unused_imports)]

use std::vec::Vec;
use std::time::Duration;
use std::fmt::Debug;

pub type PlayerID = i32;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    pub actions: Vec<Box<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMessageInit {
    pub player_id: PlayerID,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMessage {
    pub client_messages: Vec<(PlayerID, ClientMessage)>,
}

pub struct GameState;
impl GameState {
    fn step(g: &mut GameState) {}
}

pub fn delay() -> Duration {Duration::from_millis(50)}