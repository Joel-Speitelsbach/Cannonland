
use std::net::{TcpListener, TcpStream};
use std::io::Error;

extern crate serde;
use self::serde::{Serialize, Deserialize};
extern crate bincode;
use self::bincode::{Infinite, Result as Result_};


pub struct OtherSide { tcp_stream: TcpStream }
pub struct Server { tcp_listener: TcpListener }


pub fn send<D>(other: &OtherSide, data: &D) -> Result_<()>
        where D: Serialize {
    let OtherSide{ref tcp_stream} = other;
    let mut tcp_stream = tcp_stream;
    bincode::serialize_into(&mut tcp_stream, &data, Infinite)
}


pub fn recieve<D>(other: &OtherSide) -> Result_<D>
        where for<'de> D: Deserialize<'de> {
    let OtherSide{ref tcp_stream} = other;
    let mut tcp_stream = tcp_stream;
    bincode::deserialize_from(&mut tcp_stream, bincode::Infinite)
}


fn start_server_with_addr(addr: String) -> Result<Server, Error> {
    match TcpListener::bind(&addr) {
        Ok(tcp_listener) => {
            tcp_listener.set_nonblocking(true).expect("could not set nonblocking mode");
            Ok(Server{tcp_listener})
        },
        Err(err) => Err(err),
    }
}


pub fn start_server() -> Result<Server, Error> {
    start_server_with_addr("0.0.0.0:8080".to_string())
}


pub fn poll_for_client(server: &Server) -> Option<OtherSide> {
    server.tcp_listener.accept().map(|(tcp_stream,_)| {
        tcp_stream.set_nonblocking(false).expect("could not set nonblocking mode");
        tcp_stream.set_nodelay(true).unwrap();
        OtherSide{tcp_stream}
    }).ok()
}


pub fn wait_for_client(server: &Server) -> Option<OtherSide> {
    server.tcp_listener.set_nonblocking(false).unwrap();
    let client = poll_for_client(&server);
    server.tcp_listener.set_nonblocking(true).unwrap();
    client
}


pub fn connect_to_server(addr: String) -> Result<OtherSide, Error> {
    TcpStream::connect(&addr).map(|tcp_stream| {
        tcp_stream.set_nonblocking(false).expect("could not set nonblocking mode");
        tcp_stream.set_nodelay(true).unwrap();
        OtherSide{tcp_stream}
    })
}
