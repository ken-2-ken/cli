use aes::{Aes128, BlockEncrypt, NewBlockCipher, cipher::{generic_array::GenericArray}};
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

use super::status::Status;
use std::{fs::File, io::{self, Read, Write}, net::TcpStream};

pub struct Client {
    stream: TcpStream,
    cipher: Aes128,
    buffer: Vec<u8>,
}

impl Client {
    pub fn connect(mut stream: TcpStream) -> io::Result<Self> {
        let secret = EphemeralSecret::new(OsRng);
        let public = PublicKey::from(&secret);

        stream.write(public.as_bytes())?;

        let mut other_public = [0u8; 32];
        stream.read(&mut other_public)?;

        let other_public = PublicKey::from(other_public);
        let shared_secret = secret.diffie_hellman(&other_public);
        let key = GenericArray::from_slice(&shared_secret.as_bytes()[0..16]);
        let cipher = Aes128::new(&key);

        Ok(Self { stream, cipher, buffer: vec![] })
    }

    fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    // convert plaintext stored in buffer into 16 byte chunks w/ padding
    // encrypt each block with AES and then write to socket
    fn flush(&mut self) -> io::Result<()> {
        let length = (self.buffer.len() as u64).to_le_bytes();
        let mut block = GenericArray::clone_from_slice(&[length, [0u8; 8]].concat());
        self.cipher.encrypt_block(&mut block);
        self.stream.write(block.as_slice())?;
        self.write(vec![0u8; 16 - self.buffer.len() % 16].as_slice());

        for chunk in self.buffer.chunks_mut(16) {
            self.cipher.encrypt_block(GenericArray::from_mut_slice(chunk));
        }

        self.stream.write(self.buffer.as_slice())?;

        Ok(())
    }

    pub fn write_message(&mut self, message: &str) -> io::Result<()> {
        self.write(&[Status::Message as u8]);
        self.write(message.as_bytes());

        self.flush()?;

        Ok(())
    }

    pub fn write_file(&mut self, path: &str) -> io::Result<()> {
        let mut file = File::open(path.clone())?;

        self.write(&[Status::File as u8]);
        self.write(path.as_bytes());
        self.write(&['\n' as u8]);
        file.read_to_end(&mut self.buffer)?;

        self.flush()?;

        Ok(())
    }
}
