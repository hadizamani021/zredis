use std::{
    io::{prelude::*},
    net::{TcpListener},
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let response = "+PONG\r\n";
                let mut buf = [0; 512];
                loop{                    
                    let number_of_read_bytes = stream.read(&mut buf).unwrap();
                    
                    if number_of_read_bytes == 0{
                        break;
                    }
                    stream.write(response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
