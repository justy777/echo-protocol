use std::io::{self, BufRead, BufReader, LineWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub const MAX_DATAGRAM_SIZE: usize = 65_507;

pub struct BufTcpStream {
    reader: BufReader<TcpStream>,
    writer: LineWriter<TcpStream>,
}

impl BufTcpStream {
    /// # Errors
    ///
    /// This function has the same error semantics as `try_clone`.
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let reader = BufReader::new(stream.try_clone()?);
        let writer = LineWriter::new(stream);

        Ok(Self { reader, writer })
    }

    /// # Errors
    ///
    /// This function has the same error semantics as `TcpStream::connect` and will return an
    /// error if a connection cannot be made.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Self::new(stream)
    }

    /// # Errors
    ///
    /// This function has the same error semantics as `write_all` and may generate an I/O error
    /// indicating that the operation could not be completed.
    /// If an error is returned then no bytes in the buffer were written to this writer.
    pub fn send(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_all(s.as_bytes())?;
        self.writer.write_all(b"\n")
    }

    /// # Errors
    ///
    /// This function has the same error semantics as `read_line` and will return an I/O error
    /// if the underlying reader was read, but returned an error.
    pub fn recv(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        line.pop();
        Ok(line)
    }
}
