use super::status::Status;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

pub const ORIGIN: &str = "0.0.0.0";
const PORT: u16 = 8000;

pub struct Socket {
    pub port: u16,
    listener: TcpListener,
}

impl Socket {
    pub fn new() -> Self {
        Self::at(PORT)
    }

    fn at(port: u16) -> Socket {
        match TcpListener::bind(format!("{}:{}", ORIGIN, port)) {
            Ok(listener) => Self { port, listener },
            Err(_) => Self::at(port + 1),
        }
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            println!("Received connection");

            match Self::on_incoming(stream) {
                Ok(result) => println!("{}", result),
                Err(error) => println!("{}", error),
            }
        }
    }

    fn on_incoming(stream: Result<TcpStream, Error>) -> io::Result<String> {
        let mut stream = stream?;
        println!("Received connection from {}", stream.peer_addr()?);

        let mut status = [0];
        stream.read(&mut status)?;

        let mut reader = BufReader::new(stream);

        match status[0].into() {
            Status::File => {
                let mut path = String::new();
                reader.read_line(&mut path)?;

                let path = format!("~/Downloads/{}", path.trim()).as_str();
                File::create(path)?.write(reader.buffer())?;

                return Ok(format!("Downloaded file into {}", path));
            }
            Status::Message => {
                let mut message = String::new();
                reader.read_line(&mut message)?;

                Ok(format!("Received message: {}", message.trim()))
            }
            Status::Unknown => Err(Error::new(ErrorKind::Other, "Invalid message status")),
        }
    }
}
