// module info: data types for sending messages between server and client

use std::vec::Vec;

pub type PlayerID = u32; //can be changed and maybe this belongs into 'game'

// 'game' can use this to alter its state
#[derive(Serialize, Deserialize, Debug)]
pub enum PlayerAction {
    ChangeAngle,
    ChangeLoad,
    Fire,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientMessage {
    pub actions: Vec<PlayerAction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMessageInit {
    pub player_id: PlayerID,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMessage {
    pub client_messages: Vec<(PlayerID, ClientMessage)>,
}
