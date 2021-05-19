use aes::{Aes128, BlockDecrypt, NewBlockCipher, cipher::generic_array::GenericArray};
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

use super::status::Status;
use std::{convert::TryInto, fs::File, io::{self, Error, ErrorKind, Read, Write}, net::{TcpListener, TcpStream}};

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
        
        let secret = EphemeralSecret::new(OsRng);
        let public = PublicKey::from(&secret);
        stream.write(public.as_bytes())?;

        let mut other_public = [0u8; 32];
        stream.read(&mut other_public)?;

        let other_public = PublicKey::from(other_public);
        let shared_secret = secret.diffie_hellman(&other_public);
        
        let key = GenericArray::from_slice(&shared_secret.as_bytes()[0..16]);
        let cipher = Aes128::new(&key);

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer)?;

        for block in buffer.chunks_mut(16) {
            cipher.decrypt_block(GenericArray::from_mut_slice(block));
        }

        let len = u64::from_le_bytes(buffer[0..8].try_into().unwrap());

        let buffer = &buffer[16..16 + len as usize];

        let status = buffer[0].into();

        let buffer = &buffer[1..];

        match status {
            Status::File => {
                let (path, data) = buffer.split_at(buffer.iter().position(|&b| b == '\n' as u8).unwrap());

                // remove delimiter from right side of split
                let data = &data[1..];

                let path = std::str::from_utf8(path).unwrap();

                let path = format!("{}/Downloads/{}", std::env::var("HOME").unwrap(), path.trim());
                let mut file = File::create(path.as_str())?;

                file.write(data)?;

                Ok(format!("Downloaded file from {} into {}", address, path))
            }
            Status::Message => {
                let message = std::str::from_utf8(&buffer).unwrap();

                Ok(format!("Received message from {}: {}", address, message))
            }
            Status::Unknown => Err(Error::new(ErrorKind::Other, "Invalid message status")),
        }
    }
}
