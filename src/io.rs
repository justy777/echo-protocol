use std::io::{self, BufRead, BufReader, LineWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub const MAX_DATAGRAM_SIZE: usize = 65_507;

pub struct BufTcpStream {
    reader: BufReader<TcpStream>,
    writer: LineWriter<TcpStream>,
}

impl BufTcpStream {
    /// # Errors
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let reader = BufReader::new(stream.try_clone()?);
        let writer = LineWriter::new(stream);

        Ok(Self { reader, writer })
    }

    /// # Errors
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Self::new(stream)
    }

    /// # Errors
    pub fn send(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_all(s.as_bytes())?;
        self.writer.write_all(b"\n")
    }

    /// # Errors
    pub fn recv(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        line.pop();
        Ok(line)
    }
}
