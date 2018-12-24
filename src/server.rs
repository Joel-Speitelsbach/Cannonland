// module info: server specific code / data

use std::collections::HashMap;
use network;
use super::message::{ServerMessage,ClientMessage,PlayerID,ServerMessageInit,PlayerAction};
use battlefield::Battlefield;
use std::cmp;


pub fn run(opts: &[String]) {
    println!("opts: {:?}", opts);
    let mut clients: HashMap<PlayerID,network::OtherSide> = HashMap::new();
    let server_handle = network::Simple::start_server().unwrap();
    
    // create battlefield
    let mut battlefield = Battlefield::new();
    
    // main loop
    loop {
        // recieve messages from clients
        let mut messages: Vec<(PlayerID,ClientMessage)> = Vec::new();
        for (id, cl) in &clients {
            if let Ok(msg) = network::Simple::recieve(&cl) {
                // println!("client nr.{}: {:?}", &id, &msg);
                messages.push((*id,msg));
            }
        }
        
        
        // (maybe) add new client
        if clients.len() == 0 {
            server_handle.set_nonblocking(false).unwrap();
        }
        if let Some(client) = network::Simple::poll_for_client(&server_handle) {
            let next_player_id = next_player_id(&clients);
            let init_message = ServerMessageInit {
                player_id: next_player_id,
                battlefield: battlefield.clone(),
            };
            network::Simple::send(&client, &init_message).unwrap();
            
            clients.insert(next_player_id, client);
            messages.push((next_player_id,
                ClientMessage {
                    actions: vec!(PlayerAction::NewBunker),
                }
            ));
        }
        server_handle.set_nonblocking(true).unwrap();
            
            
        // resend client messages
        let msg = ServerMessage {
            client_messages: messages.clone(),
        };
        // if messages.len() > 0 {
        //     println!("{:?}", &msg);
        // }
        let mut clients_to_remove = vec!();
        for (id, cl) in &clients {
            // cl.set_nonblocking(true).unwrap();
            if let Err(err) = network::Simple::send(&cl, &msg) {
                println!("client {} disconnected", &id);
                println!("debug info: {}", err);
                clients_to_remove.push(id.clone());
            } else {
                // cl.set_nonblocking(false).unwrap();
            }
        }
        for id in clients_to_remove {
            clients.remove(&id);
        }
        
        
        // update battlefield
        for (player_id,client_message) in &messages {
            for action in &client_message.actions {
                battlefield.execute_action(*player_id, &action);
            }
        }
        battlefield.stride();
        
        // sleep(Duration::from_millis(15));
    }
}

fn next_player_id<T>(player_map: &HashMap<PlayerID, T>) -> PlayerID {
    if player_map.is_empty() { return 0 };
    
    let mut min = 10000000;
    for id in player_map.keys() {
        min = cmp::min(min,*id);
    }
    min + 1
}
