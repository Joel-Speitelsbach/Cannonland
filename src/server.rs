// module info: server specific code / data

use std::collections::HashMap;
use crate::network;
use super::message::{ServerMessage,ClientMessage,PlayerID,ServerMessageInit,PlayerAction};
use crate::battlefield::Battlefield;
use crate::sound::Sound;


pub fn run() {
    let mut clients: HashMap<PlayerID,network::OtherSide> = HashMap::new();
    let mut clients_to_remove = vec!();

    let server_handle = network::start_server().unwrap();

    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    let _ = fps_manager.set_framerate(60);

    // create battlefield
    let mut battlefield = Battlefield::new();

    // main loop
    loop {
        // recieve messages from clients
        let mut messages: Vec<(PlayerID,ClientMessage)> = Vec::new();
        for (id, cl) in &clients {
            if let Ok(msg) = cl.recieve() {
                messages.push((*id,msg));
            }
        }


        // (maybe) add new client
        let client = if clients.len() == 0 {
            server_handle.wait_for_client()
        } else {
            server_handle.poll_for_client()
        };
        if let Some(client) = client {
            let next_player_id = next_player_id(&clients).unwrap();
            let init_message = ServerMessageInit {
                player_id: next_player_id,
                battlefield: battlefield.clone(),
            };
            client.send_large(&init_message).unwrap();

            clients.insert(next_player_id, client);
            messages.push((next_player_id,
                ClientMessage {
                    actions: vec!(PlayerAction::NewBunker),
                }
            ));
        }


        // resend client messages
        for cl in clients_to_remove {
            messages.push((cl,ClientMessage{actions: vec!(PlayerAction::DeleteBunker)}));
        }
        let msg = ServerMessage {
            client_messages: messages.clone(),
        };
        clients_to_remove = vec!();
        for (id, cl) in &clients {
            if let Err(err) = cl.send(&msg) {
                println!("client {} disconnected", &id);
                println!("debug info: {}", err);
                clients_to_remove.push(id.clone());
            }
        }
        for id in &clients_to_remove {
            clients.remove(&id);
        }


        // limit frame rate
        fps_manager.delay();


        // update battlefield
        for (player_id,client_message) in &messages {
            for action in &client_message.actions {
                battlefield.execute_action(*player_id, &action, &Sound::stub());
            }
        }
        battlefield.stride(&Sound::stub());
    }
}


fn next_player_id<T>(player_map: &HashMap<PlayerID, T>) -> Option<PlayerID> {
    for id in 0..8 {
        if !player_map.contains_key(&id){
            return Some(id)
        }
    }
    None
}
