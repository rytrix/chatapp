use std::net::TcpStream;
use std::io::Read;

pub struct Client {
    socket: TcpStream,
}


impl Client {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr)?;

        Ok(Self {
            socket: stream,
        })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let buffer: &mut [u8] = &mut [0; 256];
        self.socket.read(buffer)?;
        println!("{}", std::str::from_utf8(buffer).unwrap());
        Ok(())
    }
}
