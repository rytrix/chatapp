use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

pub struct Server {
    socket: TcpListener,
}

impl Server {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpListener::bind(addr)?;
        Ok(Server { socket: stream })
    }

    pub fn run(&self) {
        for stream in self.socket.incoming() {
            let stream = stream.unwrap();
            Self::handle_client(stream).unwrap();
        }
    }

    fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
        let msg = b"hello world\n";
        stream.write(msg)?;
        Ok(())
    }
}
