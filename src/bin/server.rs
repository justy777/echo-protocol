use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use std::thread;

use clap::Parser;

const BUFFER_SIZE: usize = 1000;

#[derive(Parser, Debug)]
struct Args {
    /// Port to bind; must be in range 0-65536
    #[clap(default_value_t = 7)]
    port: u16,

    /// Enable UDP mode
    #[clap(short, long)]
    udp: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let address = format!("0.0.0.0:{}", args.port);

    if args.udp {
        handle_udp(address)?;
    } else {
        handle_tcp(address)?;
    }

    Ok(())
}

fn handle_tcp<A: ToSocketAddrs>(address: A) -> io::Result<()> {
    let listener = TcpListener::bind(&address)?;
    println!("Listening at {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection from {} accepted", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_connection(stream).map_err(|e| eprintln!("Error: {}", e))
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
    Ok(())
}

fn handle_udp<A: ToSocketAddrs>(address: A) -> io::Result<()> {
    let socket = UdpSocket::bind(&address)?;
    println!("Listening at {}", socket.local_addr().unwrap());

    let mut buf = [0; BUFFER_SIZE];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((read_bytes, socket_address)) => {
                socket.send_to(&buf[..read_bytes], socket_address)?;
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; BUFFER_SIZE];

    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                // Client has closed the connection
                println!("Connection to {} closed", stream.peer_addr()?);
                break;
            }
            Ok(read_bytes) => {
                stream.write_all(&buf[..read_bytes])?;
                stream.flush()?;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                stream.shutdown(Shutdown::Both)?;
                break;
            }
        }
    }
    Ok(())
}
