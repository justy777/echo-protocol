use std::io;
use std::net::{TcpListener, TcpStream, ToSocketAddrs, UdpSocket};

use clap::Parser;
use echo_protocol::io::{BufTcpStream, MAX_DATAGRAM_SIZE};
use echo_protocol::thread_pool::ThreadPool;

#[derive(Parser, Debug)]
struct Args {
    /// Port to bind; must be in range 0-65536
    #[clap(short, long, default_value_t = 7)]
    port: u16,

    /// Enable UDP mode
    #[clap(short, long)]
    udp: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let address = format!("0.0.0.0:{}", args.port);
    println!("Starting server on port '{}'", args.port);

    if args.udp {
        handle_udp(address)?;
    } else {
        handle_tcp(address)?;
    }

    Ok(())
}

fn handle_tcp<A: ToSocketAddrs>(address: A) -> io::Result<()> {
    let listener = TcpListener::bind(&address)?;
    let pool = ThreadPool::new(num_cpus::get());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move || {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Error: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
    Ok(())
}

fn handle_udp<A: ToSocketAddrs>(address: A) -> io::Result<()> {
    let socket = UdpSocket::bind(&address)?;

    let mut buf = [0; MAX_DATAGRAM_SIZE];
    loop {
        let (read_bytes, peer_addr) = socket.recv_from(&mut buf)?;
        println!("Incoming from {}", peer_addr);
        socket.send_to(&buf[..read_bytes], peer_addr)?;
    }
}

fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Incoming from {}", peer_addr);

    let mut buf_stream = BufTcpStream::new(stream)?;

    let message = buf_stream.recv()?;

    buf_stream.send(&message)
}
