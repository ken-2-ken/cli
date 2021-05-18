use super::status::Status;
use std::{
    fs::File,
    io::{self, Write},
    net::TcpStream,
};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn from(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn write_message(mut self, message: &str) -> std::io::Result<()> {
        self.stream.write(&[Status::Message as u8])?;

        // include \n for BufReader
        match self.stream.write(message.as_bytes()) {
            Ok(_) => println!("Successfully wrote to ken"),
            Err(_) => println!("Failed to write to ken"),
        }

        self.stream.write(&['\n' as u8])?;
        Ok(())
    }

    pub fn write_file(mut self, path: &str) -> std::io::Result<()> {
        println!("Reading file from {}", path);
        let mut file = File::open(path.clone())?;

        self.stream.write(&[Status::File as u8])?;
        self.stream.write(path.as_bytes())?;
        self.stream.write(&['\n' as u8])?;
        io::copy(&mut file, &mut self.stream)?;
        Ok(())
    }
}
