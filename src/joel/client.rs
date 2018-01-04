// client specific code / data
#![allow(dead_code,unused_variables,unused_imports)]

use std::thread::sleep;
use std::time::Duration;

use super::network;
use super::msg::{ServerMessage,ClientMessage,PlayerID,ServerMessageInit,delay};

pub fn run(opts: &[String]) {
    println!("opts: {:?}", opts);
    // connect to server
    let other = match network::Simple::connect_to_server("127.0.0.1:8080") {
        Ok(ok) => ok,
        Err(err) => {
            println!("failed to connect to server");
            println!("debug info: {}", err);
            return;
        },
    };
    // recieve init message
    other.set_nonblocking(false).unwrap();
    let init_msg: ServerMessageInit = network::Simple::recieve(&other)
        .expect("failed to recieve init msg");
    other.set_nonblocking(true).unwrap();
    println!("init_msg: {:?}", init_msg);
    let ServerMessageInit {player_id: my_player_id,..} = init_msg;
    // init client state
    let mut counter = 0;
    // main loop
    loop {
        // recieve
        if let Ok(msg) = network::Simple::recieve(&other) {
            let msg: ServerMessage = msg;
            println!("server: {:?}", &msg);
        }
        // send
        let msg = ClientMessage {
            actions: vec!(Box::new(format!("counter={}", &counter)), Box::new("snd".to_owned())),
        };
        counter += 1;
        if let Err(err) = network::Simple::send(&other, &msg) {
            // connection lost
            println!("server disconnected");
            println!("debug info: {}", err);
            break;
        }
        
        sleep(delay());
    }
}
