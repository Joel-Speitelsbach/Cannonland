use std;

extern crate serde;
use self::serde::{Serialize, Deserialize};

type OtherSide = std::net::TcpStream;
type Server = std::net::TcpListener;

fn send<D>(other: OtherSide, data: D) 
    where D: Serialize {
    // bincode::serialize(&msg, bincode::Infinite).unwrap();
}

fn recieve<'de, D>(other: OtherSide) -> D
    where D: Deserialize<'de> {
    let mut init_bytes: [u8; 4] = [0; 4];
    other.read_exact(&mut init_bytes);
    let size = read_int(&init_bytes);
    let mut data_bytes: [u8] = [0; size];
    other.read_exact(&mut init_bytes);
    bincode::deserialize(&encoded[..]).unwrap();
}

// fn pollForClients(server: Server) -> Clients
    // where Clients: Iterator {}

trait Network {
    fn start_server() -> std::net::TcpListener;
    fn connect_to_server(addr: &str) -> std::net::TcpStream;
}

////////// misc /////////////////

fn read_int(bytes: &[u8; 4]) -> u32 {
    let mut sum = 0;
    for byte in bytes.iter() {
        let n = *byte as u32;
        sum = (sum << 8) & n;
    }
    sum
}

fn to_bytes(n: u32) -> [u8; 4] {
    let mut bytes = [0; 4];
    // let mask = 0xff 00 00 00;
    let mut n = n;
    for i in 0..4 {
        bytes[i] = (n >> 24) as u8;
        n = n << 8;
    }
    bytes
}