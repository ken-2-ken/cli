use super::status::Status;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, ErrorKind, Read},
    net::{TcpListener, TcpStream},
};

pub const ORIGIN: &str = "0.0.0.0";
const PORT: u16 = 8000;

pub struct Listener {
    pub port: u16,
    socket: TcpListener,
}

impl Listener {
    pub fn new() -> Self {
        Self::at(PORT)
    }

    fn at(port: u16) -> Listener {
        match TcpListener::bind(format!("{}:{}", ORIGIN, port)) {
            Ok(socket) => Self { port, socket },
            Err(_) => Self::at(port + 1),
        }
    }

    pub fn listen(&self) {
        for stream in self.socket.incoming() {
            match Self::on_incoming(stream) {
                Ok(result) => println!("{}", result),
                Err(error) => println!("{}", error),
            }
        }
    }

    fn on_incoming(stream: Result<TcpStream, Error>) -> io::Result<String> {
        let mut stream = stream?;
        let address = stream.peer_addr()?;

        let mut status = [0];
        stream.read(&mut status)?;

        let mut reader = BufReader::new(stream);

        match status[0].into() {
            Status::File => {
                let mut path = String::new();
                reader.read_line(&mut path)?;

                let path = format!("{}/Downloads/{}", std::env::var("HOME").unwrap(), path.trim());
                let mut file = File::create(path.as_str())?;

                io::copy(&mut reader, &mut file)?;

                return Ok(format!("Downloaded file from {} into {}", address, path));
            }
            Status::Message => {
                let mut message = String::new();
                reader.read_line(&mut message)?;

                Ok(format!("Received message from {}: {}", address, message.trim()))
            }
            Status::Unknown => Err(Error::new(ErrorKind::Other, "Invalid message status")),
        }
    }
}
