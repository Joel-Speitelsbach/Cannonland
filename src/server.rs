// module info: server specific code / data

use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use network;
use super::message::{ServerMessage,ClientMessage,PlayerID,ServerMessageInit};
use battlefield::Battlefield;


pub fn run(opts: &[String]) {
    println!("opts: {:?}", opts);
    let mut next_player_id = 0;
    let mut clients: HashMap<PlayerID,network::OtherSide> = HashMap::new();
    let server_handle = network::Simple::start_server().unwrap();
    
    // create battlefield
    let mut battlefield = Battlefield::new();
    
    // main loop
    loop {
        // (maybe) add new client
        if let Some(client) = network::Simple::poll_for_client(&server_handle) {
            let init_message = ServerMessageInit {
                player_id: next_player_id,
                battlefield: battlefield.clone(),
            };
            network::Simple::send(&client, &init_message).unwrap();
            clients.insert(next_player_id, client);
            next_player_id += 1;
        }
        
        
        // recieve messages from clients
        let mut messages: Vec<(PlayerID,ClientMessage)> = Vec::new();
        for (id, cl) in &clients {
            while let Ok(msg) = network::Simple::recieve(&cl) {
                // println!("client nr.{}: {:?}", &id, &msg);
                messages.push((*id,msg));
            }
        }
        
        
        // resend client messages
        let msg = ServerMessage {
            client_messages: messages.clone(),
        };
        let mut clients_to_remove = vec!();
        for (id, cl) in &clients {
            if let Err(err) = network::Simple::send(&cl, &msg) {
                println!("client {} disconnected", &id);
                println!("debug info: {}", err);
                clients_to_remove.push(id.clone());
            } else {
                println!("{:?}", &msg);
            }
        }
        for id in clients_to_remove {
            clients.remove(&id);
        }
        
        
        // update battlefield
        for (player_id,client_message) in messages {
            for action in client_message.actions {
                battlefield.execute_action(player_id, &action);
            }
        }
        battlefield.stride();
        
        
        sleep(Duration::from_millis(15));
    }
}
