use std::{fs::File, io::{BufRead, BufReader, Error, ErrorKind, Read, Write}, net::{TcpListener, TcpStream}};
use super::status::Status;

pub const ORIGIN: &str = "0.0.0.0";
const PORT: u16 = 8000;

pub struct Listener {
    pub port: u16,
    listener: TcpListener,
}

impl Listener {
	pub fn get() -> Self {
		Self::get_at(PORT)
	}
    
    fn get_at(port: u16) -> Listener {
		match TcpListener::bind(format!("{}:{}", ORIGIN, port)) {
			Ok(listener) => Self { port, listener },
			Err(_) => Self::get_at(port + 1),
		}
	}
	
    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            println!("Received connection");
            match Self::on_incoming(stream) {
                Ok(result) => println!("{}", result),
                Err(error) => println!("{:?}", error),
            }
        }
    }

    fn on_incoming(stream: Result<TcpStream, Error>) -> std::io::Result<String> {
        let mut stream = stream?;
        
        println!("Received connection from {}", stream.peer_addr()?);
        
        let mut status = [0];
        stream.read(&mut status)?;
        
        let mut reader = BufReader::new(stream);

        let status: Status = status[0].into();
        match status {
            Status::File => {
                let mut path = String::new();
                reader.read_line(&mut path)?;
                path.pop();
        
                let mut buffer = vec![];
                let _ = reader.read_to_end(&mut buffer)?;
                    
                let mut file = File::create(format!("downloads/{}", path))?;
                file.write(&buffer)?;
        
                return Ok(String::from("Downloaded file from peer"));
            },
            Status::Message => {
                let mut buffer = String::new();
                let _ = reader.read_line(&mut buffer)?;
           
                // pop \n
                buffer.pop();
                Ok(format!("Received message from client: {}", buffer))
            },
            Status::Unknown => Err(Error::new(ErrorKind::Other, "Invalid message status")),
        }
    }
}