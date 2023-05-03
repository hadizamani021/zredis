use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
};

fn handle_pong(mut stream: TcpStream) {
    let response = "+PONG\r\n";
    stream.write(response.as_bytes()).unwrap();
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 512];
                stream.read(&mut buf).unwrap();
                handle_pong(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
