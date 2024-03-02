use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

pub struct Client {
    socket: TcpStream,
    name: String,
}

impl Client {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr)?;

        let mut name = String::new();
        let mut reader = BufReader::new(std::io::stdin());
        println!("enter username");
        reader.read_line(&mut name)?;
        
        let name = name.split('\n').nth(0).unwrap().to_string();

        Ok(Self {
            socket: stream,
            name,
        })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let msg_socket = self.socket.try_clone()?;
        std::thread::spawn(move || {
            Self::handle_server_msgs(msg_socket);
        });

        self.write_to_server()
    }

    fn write_to_server(&mut self) -> Result<(), std::io::Error> {
        let mut reader = BufReader::new(std::io::stdin());

        loop {
            let mut buffer = String::new();
            let _ = reader.read_line(&mut buffer)?;

            if buffer.trim() == "/end" {
                return Ok(());
            }

            let buffer = std::format!("{}: {}", self.name, buffer);
            self.socket.write_all(buffer.as_bytes())?;
        }
    }

    fn handle_server_msgs(stream: TcpStream) {
        let mut reader = BufReader::new(stream);
        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            print!("{buf}");
        }
    }
}
