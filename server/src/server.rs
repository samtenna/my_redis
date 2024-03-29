use std::{str, io::Read, net::{SocketAddr, TcpListener, TcpStream}};

pub fn run_server(addr: &'static str, port: u16) -> std::io::Result<()> {
    let addr_string = format!("{}:{}", addr, port.to_string());
    let listener = TcpListener::bind(addr_string)?;

    println!("Server listening at {addr}:{port} 🚀");

    loop {
        match listener.accept() {
            Ok(conn) => handle_client(conn),
            Err(_) => println!("couldn't get client: {addr:?}"),
        }
    }
}

fn handle_client((mut socket, addr): (TcpStream, SocketAddr)) {
    println!("Incoming connection from {addr}");

    let mut buffer = [0u8; 256];
    loop {
        match socket.read(&mut buffer) {
            Ok(n) => if n > 0 {
                match str::from_utf8(&buffer) {
                    Ok(s) => println!("received: {s}"),
                    Err(_) => println!("Invalid string received"),
                }
            },
            Err(_) => {},
        }
    }
}
