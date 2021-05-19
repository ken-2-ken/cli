use ken2ken::{client::Client, listener::Listener};
use std::io::{self, Write};
use std::net::TcpStream;
use std::thread::spawn;

fn main() -> std::io::Result<()> {
    let listener = Listener::new();

    println!("Hosting ken2ken on port {}", listener.port);
    spawn(move || listener.listen());

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let mut address = String::new();
        print!("Send to: ");

        stdout.flush()?;
        stdin.read_line(&mut address)?;

        let stream = match TcpStream::connect(address.trim()) {
            Ok(stream) => stream,
            Err(error) => {
                println!("Error connecting to ken: {}", error);
                continue;
            }
        };

        let mut client = match Client::connect(stream) {
            Ok(client) => client,
            Err(err) => {
                println!("Error in making handshake: {}", err);
                continue;
            }
        };

        let mut message = String::new();
        print!("Message: ");

        stdout.flush()?;
        stdin.read_line(&mut message)?;

        let message = message.trim();

        match message.strip_prefix("file:") {
            Some(path) => client.write_file(path)?,
            None => client.write_message(message)?,
        }
    }
}
