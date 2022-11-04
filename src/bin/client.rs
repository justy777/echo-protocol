use std::io;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs, UdpSocket};

use clap::Parser;
use echo_protocol::io::{BufTcpStream, MAX_DATAGRAM_SIZE};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP Address to communicate with
    address: IpAddr,

    /// Message to send to the server
    message: String,

    /// Port to communicate with; must be in range 0-65536
    #[arg(short, long, default_value_t = 7)]
    port: u16,

    /// Enable UDP mode
    #[arg(short, long)]
    udp: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let address = SocketAddr::new(args.address, args.port);

    if args.udp {
        connect_udp(address, &args.message)?;
    } else {
        connect_tcp(address, &args.message)?;
    }

    Ok(())
}

fn connect_udp<A: ToSocketAddrs>(address: A, message: &str) -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.connect(&address)?;
    socket.send(message.as_bytes())?;

    let mut buf = [0; MAX_DATAGRAM_SIZE];
    let read_bytes = socket.recv(&mut buf)?;
    let response = std::str::from_utf8(&buf[..read_bytes]).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not parse received string as UTF-8",
        )
    })?;
    println!("{}", response);

    Ok(())
}

fn connect_tcp<A: ToSocketAddrs>(address: A, message: &str) -> io::Result<()> {
    let mut buf_stream = BufTcpStream::connect(address)?;

    buf_stream.send(message)?;
    println!("{}", buf_stream.recv()?);

    Ok(())
}
