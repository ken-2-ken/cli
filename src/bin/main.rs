use ken2ken::{
    client::Client,
    socket::{Socket, ORIGIN},
};
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> std::io::Result<()> {
    let socket = Socket::new();
    println!("Hosting ken2ken on {}:{}", ORIGIN, socket.port);

    thread::spawn(move || {
        socket.listen();
    });

    let stdin = io::stdin();
    loop {
        let mut addr = String::new();
        print!("Addr: ");
        io::stdout().flush()?;
        let _ = stdin.read_line(&mut addr);

        // pop \n
        addr.pop();

        println!("Connecting to {}", addr);

        let stream = match TcpStream::connect(addr) {
            Ok(stream) => stream,
            Err(error) => {
                println!("Error in connecting to ken: {}", error);
                continue;
            }
        };

        let client = Client::from(stream);

        let mut message = String::new();
        print!("Message: ");
        io::stdout().flush()?;
        let _ = stdin.read_line(&mut message);
        message.pop();

        match message.strip_prefix("file:") {
            Some(path) => {
                client.write_file(path)?;
            }
            None => {
                client.write_message(message.as_str())?;
            }
        }
    }
}
