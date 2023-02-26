use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("connection accept!");
                let mut buf = [0; 512];

                loop {
                    let bytes_read = _stream.read(&mut buf).unwrap();
                    if bytes_read == 0 {
                        println!("connection close");
                        break
                    }

                    _stream.write("+PONG\r\n".as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
