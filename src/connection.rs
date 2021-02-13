use super::*;
use std::net;

/// Contains the TCP connection to the FAH client, as well as its address for reconnecting.
#[derive(Debug)]
pub struct Connection {
    pub conn: net::TcpStream,
    pub addr: net::SocketAddr,
    pub connect_timeout: core::time::Duration,
}

impl Connection {
    pub fn connect_timeout(addr: &net::SocketAddr, timeout: core::time::Duration) -> Result<Self> {
        Ok(Self {
            conn: connect_timeout(addr, timeout)?,
            addr: *addr,
            connect_timeout: timeout,
        })
    }

    /// Executes a command on the FAH client. The response is written to the buffer.
    pub fn exec(&mut self, command: &str, buf: &mut Vec<u8>) -> Result<()> {
        use std::io::Write;

        if command.is_empty() {
            // FAH doesn't respond to an empty command
            buf.clear();
            return Ok(());
        }

        if command.contains('\n') {
            return Err(Error::msg("command contains newline"));
        }

        self.conn.write_all(format!("{}\n", command).as_bytes())?;

        if let Err(e) = read_message(&mut self.conn, buf) {
            // Try to reconnect on disconnection
            if e.to_string() == EOF {
                self.conn = connect_timeout(&self.addr, self.connect_timeout)?;
            }
            return Err(e);
        }

        Ok(())
    }

    /// Executes commands which do not return a trailing newline. (Some commands don't end their message
    /// and cause infinite blocking.) The response is written to the buffer.
    pub fn exec_eval(&mut self, command: &str, buf: &mut Vec<u8>) -> Result<()> {
        if command.is_empty() {
            // FAH doesn't respond to an empty command
            buf.clear();
            return Ok(());
        }

        self.exec(format!(r#"eval "$({})\n""#, command).as_str(), buf)?;

        // When using eval with a newline, the response contains an extra trailing backslash.
        if let Some(b) = buf.last() {
            if *b == b'\\' {
                buf.pop();
            }
        }
        Ok(())
    }
}

fn connect_timeout(
    addr: &net::SocketAddr,
    timeout: core::time::Duration,
) -> Result<net::TcpStream> {
    let mut conn = net::TcpStream::connect_timeout(addr, timeout)?;

    // Discard welcome message
    read_message(&mut conn, &mut Vec::new())?;
    Ok(conn)
}

const EOF: &str = "EOF";

pub fn read_message(r: &mut impl std::io::Read, buf: &mut Vec<u8>) -> Result<()> {
    buf.clear();
    loop {
        let mut b: [u8; 1] = [0];
        if r.read(&mut b)? == 0 {
            // If we haven't reached END_OF_MESSAGE and 0 bytes was read, then EOF was returned.
            // This can occur if the command was invalid.
            return Err(Error::msg(EOF));
        }

        buf.push(b[0]);

        const END_OF_MESSAGE: &str = "\n> ";
        if buf.len() >= END_OF_MESSAGE.len()
            && buf.as_slice()[buf.len() - END_OF_MESSAGE.len()..] == *END_OF_MESSAGE.as_bytes()
        {
            buf.truncate(buf.len() - END_OF_MESSAGE.len());
            if let Some(b) = buf.get(0) {
                if *b == b'\n' {
                    buf.drain(..1);
                }
            }
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_read_message() {
        struct Test {
            s: &'static [u8],
            expected: &'static [u8],
            expect_error: bool,
        }

        let tests = vec![
            Test {
                s: b"",
                expected: b"",
                expect_error: true,
            },
            Test {
                s: b"\n",
                expected: b"\n",
                expect_error: true,
            },
            Test {
                s: b"\n> ",
                expected: b"",
                expect_error: false,
            },
            Test {
                s: b"a\n> ",
                expected: b"a",
                expect_error: false,
            },
            Test {
                s: b"a\n> \n> ",
                expected: b"a",
                expect_error: false,
            },
            Test {
                s: b"\na\n> ",
                expected: b"a",
                expect_error: false,
            },
        ];

        let mut buf: Vec<u8> = Vec::new();
        for (i, test) in tests.iter().enumerate() {
            use bytes::buf::ext::BufExt;
            let result = read_message(&mut bytes::Bytes::from_static(test.s).reader(), &mut buf);
            assert_eq!(result.is_err(), test.expect_error);
            assert_eq!(buf.as_slice(), test.expected, "{}", i);
        }
    }
}
