use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;

pub struct Client {
    socket: TcpStream,
}

impl Client {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr)?;

        Ok(Self { socket: stream })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let socket = Arc::new(self.socket.try_clone()?);

        let t = std::thread::spawn(move || {
            let mut buffer: String = String::new();
            socket.read_to_string(&mut buffer).unwrap();
            if buffer == "end" {
                return ();
            }
            println!("{}", buffer);
        });

        Ok(())
    }
}
