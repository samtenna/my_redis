use std::{io::Write, net::TcpStream};

fn main() {
    // TODO: replace this with command line input
    const ADDR: &'static str = "localhost";
    const PORT: u16 = 8080;

    println!("Attempting to connect to {ADDR}:{PORT}");
    connect(ADDR, PORT).expect("Failed to establish connection to the server");
}

fn connect(addr: &'static str, port: u16) -> std::io::Result<()> {
    let bind_string = format!("{addr}:{port}");
    let mut stream = TcpStream::connect(bind_string)?;
    println!("Connection established");

    // send a hello message
    stream.write_all(b"Hello, world!")?;
    stream.flush()?;

    loop {
    }
}
