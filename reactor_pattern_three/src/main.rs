use std::io::{Read,Write,Result};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_incoming_connections(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }


    }
    Ok(())
}

fn handle_incoming_connections(mut stream: TcpStream) ->Result<()>{
    let mut buffer=[0;1024];
    let response="HTTP:1.1 200 OK\r\n\r\nHello from Rust\r\n\r\n";
    loop {
        match stream.read(&mut buffer) {
            Ok(0)=>{
                break;
            }
            Ok(n)=>{
                println!("Received {} bytes", n);
                stream.write_all(response.as_ref()).unwrap();
                break;
            }
            Err(e)=>{
                eprintln!("Failed to handle connection. Error: {}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

