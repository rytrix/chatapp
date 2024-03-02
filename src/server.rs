use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

pub struct Server {
    socket: TcpListener,
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl Server {
    pub fn new(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpListener::bind(addr)?;
        Ok(Server {
            socket: stream,
            clients: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        for stream in self.socket.incoming() {
            let stream = stream?;
            self.clients.lock().unwrap().push(stream.try_clone()?);

            let arc = self.clients.clone();
            std::thread::spawn(|| {
                Self::handle_client(arc, stream).unwrap();
            });
        }
        Ok(())
    }

    fn handle_client(clients: Arc<Mutex<Vec<TcpStream>>>, stream: TcpStream) -> Result<(), std::io::Error> {
        let mut reader = BufReader::new(stream);
        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf)?;
            
            let mut remove_list = Vec::<usize>::new();
            let mut clients_vec = clients.lock().unwrap();
            for i in 0..clients_vec.len() {
                let result = clients_vec[i].write_all(buf.as_bytes());
                if let Err(_err) = result {
                    remove_list.push(i);
                }
            }

            for i in remove_list {
                clients_vec.remove(i);
            }
        }
    }
}
