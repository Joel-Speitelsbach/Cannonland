
use std::net::{TcpListener, TcpStream};
use std::io::Error;

extern crate serde;
use self::serde::{Serialize, Deserialize};
extern crate bincode;
use self::bincode::{Infinite, Result as Result_};
use config;


pub const PORT: &str = "4242";


pub struct OtherSide { tcp_stream: TcpStream }
pub struct Server { tcp_listener: TcpListener }


pub fn start_server() -> Result<Server, Error> {
    start_server_with_addr(&format!("0.0.0.0:{}", PORT))
}


fn start_server_with_addr(addr: &str) -> Result<Server, Error> {
    match TcpListener::bind(&addr) {
        Ok(tcp_listener) => {
            tcp_listener.set_nonblocking(true).expect("could not set nonblocking mode");
            Ok(Server{tcp_listener})
        },
        Err(err) => Err(err),
    }
}


pub fn connect_to_server(addr: &str) -> Result<OtherSide, Error> {
    let addr = format!("{}:{}", addr, PORT);
    TcpStream::connect(&addr).map(|tcp_stream| {
        tcp_stream.set_nonblocking(false).expect("could not set nonblocking mode");
        tcp_stream.set_nodelay(true).unwrap();
        OtherSide{tcp_stream}
    })
}


impl OtherSide {
    pub fn send<D>(&self, data: &D) -> Result_<()>
            where D: Serialize {
        let OtherSide{ref tcp_stream} = self;
        let mut tcp_stream = tcp_stream;
        bincode::serialize_into(&mut tcp_stream, &data, Infinite)
    }


    pub fn send_large<D>(&self, data: &D) -> Result_<()>
            where D: Serialize {
        self.set_nondelay(false);
        let res = self.send(data);
        self.set_nondelay(true);
        res
    }


    pub fn recieve<D>(&self) -> Result_<D>
            where for<'de> D: Deserialize<'de> {
        let OtherSide{ref tcp_stream} = self;
        let mut tcp_stream = tcp_stream;
        bincode::deserialize_from(&mut tcp_stream, bincode::Infinite)
    }

    fn set_nondelay(&self, nodelay: bool) {
        self.tcp_stream.set_nodelay(nodelay).expect("could not set nodelay");
    }
}


impl Server {
    pub fn poll_for_client(&self) -> Option<OtherSide> {
        self.tcp_listener.accept().map(|(tcp_stream,_)| {
            tcp_stream.set_nonblocking(false).expect("could not set nonblocking mode");
            tcp_stream.set_nodelay(true).expect("could not set nodelay");
            OtherSide{tcp_stream}
        }).ok()
    }


    pub fn wait_for_client(&self) -> Option<OtherSide> {
        self.tcp_listener.set_nonblocking(false).expect("could not set nonblocking mode");
        let client = self.poll_for_client();
        self.tcp_listener.set_nonblocking(true).expect("could not set nonblocking mode");
        client
    }
}
