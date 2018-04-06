use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
use std::io::{Read, Write};

fn handle_connections(mut stream: TcpStream) {
    let mut buffer = String::new();

    loop {
        match stream.read_to_string(&mut buffer) {
            Ok(buf) => println!("Data received: {:?}", buf),
            Err(_) => { println!("No data received"); break },
        }
    }
}

fn main () {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    match TcpListener::bind(address) {
        Ok(listener) => {
            println!("Started server...\n");
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => handle_connections(stream),
                    Err(_) => println!("Connection failed");
                }
            }
        },
        Err(_) => println!("Error starting server");
    }
}