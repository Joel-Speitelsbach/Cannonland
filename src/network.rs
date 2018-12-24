
use std::net::{TcpListener, TcpStream};
use std::io::Error;

extern crate serde;
use self::serde::{Serialize, Deserialize};
extern crate bincode;
use self::bincode::{Infinite, Result as Result_};


pub type OtherSide = TcpStream;
pub type Server = TcpListener;

pub struct Simple;
impl Simple {
    pub fn send<D>(mut other: &OtherSide, data: &D) -> Result_<()>
            where D: Serialize {
        bincode::serialize_into(&mut other, &data, Infinite) //todo
    }

    pub fn recieve<D>(mut other: &OtherSide) -> Result_<D>
            where for<'de> D: Deserialize<'de> {
        bincode::deserialize_from(&mut other, bincode::Infinite)
    }

    pub fn start_server() -> Result<Server, Error> {
        match TcpListener::bind("127.0.0.1:8080") {
            Ok(x) => {
                x.set_nonblocking(true).expect("could not set nonblocking mode");
                Ok(x)
            },
            err => err,
        }
    }
    
    pub fn poll_for_client(server: &Server) -> Option<OtherSide> {
        match server.accept() {
            Ok((stream,_)) => {
                stream.set_nonblocking(false).expect("could not set nonblocking mode");
                stream.set_nodelay(true).unwrap();
                Some(stream)
            },
            Err(_) => None,
        }
    }
    
    pub fn connect_to_server(addr: String) -> Result<OtherSide, Error> {
        let connect = TcpStream::connect(&addr);
        if let Ok(stream) = connect {
            stream.set_nonblocking(false).expect("could not set nonblocking mode");
            stream.set_nodelay(true).unwrap();
            return Ok(stream);
        }
        connect
    }
}
