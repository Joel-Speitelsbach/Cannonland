#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate bincode;

mod network;
mod client;
mod server;
mod msg;

use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    int: i32,
    string: String,
    eid: Eid,
}

#[derive(Serialize, Deserialize, Debug)]
struct Eid {
    mat: (i32, i32, i32),
    bs: Vec<u8>,
}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(&args[1..]);
}

pub fn run(opts: &[String]) {
    match opts[0].as_str() {
        "client" => client::run(&opts[1..]),
        "server" => server::run(&opts[1..]),
        _        => panic!("wrong mode (must be either 'client' or 'server')"),
    }
}

// fn client_() {
    // println!("client");
    // let msg = Msg {
        // int: 3,
        // string: "Testing".to_owned(),
        // eid: Eid {
            // mat: (1,2,3),
            // bs: [0; 6].to_vec(),
        // },
    // };
    // let other = network::Simple::connect_to_server("127.0.0.1:8080");
    // network::Simple::send(&other, msg).unwrap();
// }

// fn server_() {
    // println!("server");
    // let server = network::Simple::start_server().unwrap();
    // let other: Option<network::OtherSide>;
    // loop {
        // match network::Simple::poll_for_client(&server) {
            // None => (),
            // x => {
                // other = x;
                // break;
            // }
        // }
    // }
    // let mut other = other.unwrap();
    // let msg: Msg = network::Simple::recieve(&mut other).unwrap();
    // println!("{:?}", msg);
// }
