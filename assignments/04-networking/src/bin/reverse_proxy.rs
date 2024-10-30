use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <proxy-from> <proxy-to>", args[0]);
        std::process::exit(2);
    }

    let proxy_addr = &args[1];
    let origin_addr = &args[2];

    let listener = TcpListener::bind(proxy_addr).expect("Failed to bind to proxy address");
    println!("Proxy listening on {}", proxy_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(client_stream) => {
                let origin_addr = origin_addr.clone();
                thread::spawn(move || {
                    if let Err(e) = handle_client(client_stream, &origin_addr) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

fn handle_client(client_stream: TcpStream, origin_addr: &str) -> std::io::Result<()> {
    // Connect to the origin server
    let origin_stream = TcpStream::connect(origin_addr)?;

    // Create threads for bidirectional communication
    let mut client_reader = client_stream.try_clone()?;
    let mut origin_writer = origin_stream.try_clone()?;

    // Forward client request to origin server
    let client_to_origin = thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            let bytes_read = client_reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            origin_writer.write_all(&buffer[..bytes_read])?;
        }
        Ok::<_, std::io::Error>(())
    });

    // Forward origin server response to client
    let mut origin_reader = origin_stream;
    let mut client_writer = client_stream;
    let origin_to_client = thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            let bytes_read = origin_reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            client_writer.write_all(&buffer[..bytes_read])?;
        }
        Ok::<_, std::io::Error>(())
    });

    // Wait for both directions to finish
    let _ = client_to_origin.join().unwrap();
    let _ = origin_to_client.join().unwrap();

    Ok(())
}
