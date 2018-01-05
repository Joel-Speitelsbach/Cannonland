// module info: data types for sending messages between server and client

use std::vec::Vec;

pub type PlayerID = u32; //can be changed and maybe this belongs into 'game'

// enum ChangeWeapon {
    // Next,Prev,
// }

// 'game' uses this to alter its state
#[derive(Serialize, Deserialize, Debug)]
pub enum PlayerAction {
    TurnCannon { diff_angle: f32 }, /*radian, clockwise*/ 
    IncreaseLoad { inc: f32 }, /* 'load' ranges from 0. to 1. */
    CangeWeapon (ChangeWeapon),
    Fire,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChangeWeapon {
    Next, 
    Prev,
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
