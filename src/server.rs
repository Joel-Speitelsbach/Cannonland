// module info: server specific code / data

use std::collections::HashMap;
use network;
use super::message::{ServerMessage,ClientMessage,PlayerID,ServerMessageInit,PlayerAction};
use battlefield::Battlefield;
use std::cmp;
use sound::Sound;


pub fn run() {
    let mut clients: HashMap<PlayerID,network::OtherSide> = HashMap::new();

    let server_handle = network::start_server().unwrap();

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
            let next_player_id = next_player_id(&clients);
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
        let msg = ServerMessage {
            client_messages: messages.clone(),
        };
        let mut clients_to_remove = vec!();
        for (id, cl) in &clients {
            if let Err(err) = cl.send(&msg) {
                println!("client {} disconnected", &id);
                println!("debug info: {}", err);
                clients_to_remove.push(id.clone());
            }
        }
        for id in clients_to_remove {
            clients.remove(&id);
        }


        // update battlefield
        for (player_id,client_message) in &messages {
            for action in &client_message.actions {
                battlefield.execute_action(*player_id, &action, &Sound::stub());
            }
        }
        battlefield.stride(&Sound::stub());
    }
}


fn next_player_id<T>(player_map: &HashMap<PlayerID, T>) -> PlayerID {
    if player_map.is_empty() { return 0 };

    let mut min = 10000000;
    for id in player_map.keys() {
        min = cmp::min(min,*id);
    }
    println!("next playerid = {}", min + 1);
    min + 1
}
