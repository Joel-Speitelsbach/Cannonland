
extern crate serde;
extern crate bincode;

// mod network;

use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Msg<'a> {
    int: i32,
    string: &'a str,
}

pub fn run() {
    let args: Vec<_> = env::args().collect();
    let opt = &args[3];
    match args[2].as_str() {
        "client" => client(&opt),
        "server" => server(),
        _        => panic!("wrong mode (must be either 'client' or 'server')"),
    }
}

fn client(addr: &str) {
    println!("client");
    let msg = Msg { int: 3, string: "Testing" };
    let encoded = bincode::serialize(&msg, bincode::Infinite).unwrap();
    
    let decoded: Msg = bincode::deserialize(&encoded[..]).unwrap();
    println!("{:?}", decoded);
}

fn server() {
    println!("server");
}
