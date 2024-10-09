// module info: data types for sending messages between server and client

use std::vec::Vec;
use crate::battlefield::Battlefield;


pub type PlayerID = i32; //maybe this belongs into 'battlefield'


// 'battlefield' uses this to alter its state
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerAction {
    TurnCannon { diff_angle: f32 }, /*radian, clockwise*/
    IncreaseLoad { inc: f32 }, /* 'load' ranges from 0. to 1. */
    CangeWeapon (ChangeWeapon),
    Fire,
    NewBunker,
    DeleteBunker,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ChangeWeapon {
    Next,
    Prev,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientMessage {
    pub actions: Vec<PlayerAction>,
}


#[derive(Serialize, Deserialize)]
pub struct ServerMessageInit {
    pub player_id: PlayerID,
    pub battlefield: Battlefield,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ServerMessage {
    pub client_messages: Vec<(PlayerID, ClientMessage)>,
}

