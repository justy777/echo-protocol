use std::error;
use std::io::{stdin, Read, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs, UdpSocket};

use clap::Parser;

const BUFFER_SIZE: usize = 1000;

#[derive(Parser, Debug)]
struct Args {
    /// IP Address to communicate with
    ip_address: String,

    /// Port to communicate with; must be in range 0-65536
    #[clap(default_value_t = 7)]
    port_number: u16,

    /// Enable UDP mode
    #[clap(short, long)]
    udp: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    let address = format!("{}:{}", &args.ip_address, &args.port_number);

    if args.udp {
        connect_user_datagram(address)?;
    } else {
        connect_transmission_control(address)?;
    }

    println!("Connection Terminated");
    Ok(())
}

fn connect_user_datagram<A: ToSocketAddrs>(address: A) -> Result<(), Box<dyn error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    match socket.connect(&address) {
        Ok(_) => {
            let mut stdin_buf = String::new();
            let mut socket_buf = [0; BUFFER_SIZE];
            loop {
                stdin().read_line(&mut stdin_buf)?;
                stdin_buf = stdin_buf.strip_trailing_newline().into();
                if stdin_buf.is_empty() {
                    break;
                }
                socket.send(stdin_buf.as_bytes())?;
                stdin_buf.clear();

                match socket.recv(&mut socket_buf) {
                    Ok(read_bytes) => {
                        let s = std::str::from_utf8(&socket_buf[..read_bytes])?;
                        println!("{}", s);
                    }
                    Err(e) => println!("Failed to receive data: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to connect: {}", e),
    }
    Ok(())
}

fn connect_transmission_control<A: ToSocketAddrs>(address: A) -> Result<(), Box<dyn error::Error>> {
    match TcpStream::connect(&address) {
        Ok(mut stream) => {
            let mut stdin_buf = String::new();
            let mut stream_buf = [0; BUFFER_SIZE];
            loop {
                stdin().read_line(&mut stdin_buf)?;
                stdin_buf = stdin_buf.strip_trailing_newline().into();
                if stdin_buf.is_empty() {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                stream.write_all(stdin_buf.as_bytes())?;
                stream.flush()?;
                stdin_buf.clear();

                match stream.read(&mut stream_buf) {
                    Ok(read_bytes) => {
                        let s = std::str::from_utf8(&stream_buf[..read_bytes])?;
                        println!("{}", s);
                    }
                    Err(e) => println!("Failed to receive data: {}", e),
                }
            }
        }
        Err(e) => println!("Failed to connect: {}", e),
    }
    Ok(())
}

trait StrExt {
    #[must_use]
    fn strip_trailing_newline(&self) -> &str;
}

impl StrExt for str {
    #[must_use]
    fn strip_trailing_newline(&self) -> &str {
        self.strip_suffix("\r\n")
            .or(self.strip_suffix("\n"))
            .unwrap_or(self)
    }
}
