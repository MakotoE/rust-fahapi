use std::net;

pub struct API {
    pub conn: net::TcpStream,
    pub buf: Vec<u8>,
}

impl API {
    /// Default TCP address of the FAH client.
    pub fn default_addr() -> net::SocketAddr {
        net::SocketAddr::V4(net::SocketAddrV4::new(
            net::Ipv4Addr::LOCALHOST,
            36330,
        ))
    }

    /// Connects to your FAH client with specified timeout for all read and writes. Use
    /// API::default_addr() to get the default address.
    pub fn connect_timeout(addr: &net::SocketAddr, timeout: core::time::Duration) ->
    std::io::Result<API> {
        let mut conn = net::TcpStream::connect_timeout(addr, timeout)?;
        conn.set_read_timeout(Some(timeout))?;
        conn.set_write_timeout(Some(timeout))?;

        let mut buf: Vec<u8> = Vec::new();

        // Discard welcome message
        read_message(&mut conn, &mut buf)?;

        Ok(API{
            conn,
            buf,
        })
    }

    /// Returns a listing of the FAH API commands.
    pub fn help(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        exec(&mut self.conn, "help", &mut self.buf)?;
        match std::str::from_utf8(self.buf.as_slice()) {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

/// Executes a command on the FAH client. The response is written to the buffer.
pub fn exec(conn: &mut net::TcpStream, command: &str, buf: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    if command == "" {
        // FAH doesn't respond to an empty command
        buf.clear();
        return Ok(());
    }

    if command.contains("\n") {
        return Err(Box::new(Error::CommandContainsNewline));
    }

    conn.write_all(format!("{}\n", command).as_bytes())?;
    match read_message(conn, buf) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

/// Executes commands which do not return a trailing newline. (Some commands don't end their message
/// and cause infinite blocking.) The response is written to the buffer.
pub fn exec_eval(conn: &mut net::TcpStream, command: &str, buf: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    if command == "" {
        // FAH doesn't respond to an empty command
        buf.clear();
        return Ok(());
    }

    exec(conn, format!(r#"eval "$({})\n""#, command).as_str(), buf)?;

    // When using eval with a newline, the response contains an extra trailing backslash.
    match buf.last() {
        Some(b) => if *b == b'\\' {
            buf.pop();
        },
        None => {},
    }
    Ok(())
}

fn read_message(r: &mut impl std::io::Read, buf: &mut Vec<u8>) -> std::io::Result<()> {
    buf.clear();
    loop {
        let mut b: [u8; 1] = [0];
        if r.read(b.as_mut())? == 0 {
            return Ok(())
        }

        buf.push(b[0]);

        const END_OF_MESSAGE: &str = "\n> ";
        if buf.len() >= END_OF_MESSAGE.len()
            && buf.as_slice()[buf.len()-END_OF_MESSAGE.len()..] == *END_OF_MESSAGE.as_bytes() {
            buf.truncate(buf.len() - END_OF_MESSAGE.len());
            match buf.get(0) {
                Some(b) => if *b == b'\n' {
                    buf.drain(..1);
                },
                None => {},
            }
            return Ok(())
        }
    }
}

#[derive(Debug, snafu::Snafu, Clone)]
pub enum Error {
    #[snafu(display("command contains newline"))]
    CommandContainsNewline,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_message() {
        struct Test {
            s: &'static str,
            expected: &'static str,
        }

        let tests = vec![
            Test{
                s: "\n> ",
                expected: "",
            },
            Test{
                s: "a\n> ",
                expected: "a",
            },
            Test{
                s: "a\n> \n> ",
                expected: "a",
            },
            Test{
                s: "\na\n> ",
                expected: "a",
            },
        ];

        let mut buf: Vec<u8> = Vec::new();
        for (i, test) in tests.iter().enumerate() {
            use bytes::buf::ext::BufExt;
            read_message(&mut bytes::Bytes::from(test.s).reader(), &mut buf).unwrap();
            assert_eq!(std::str::from_utf8(buf.as_slice()).unwrap(), test.expected, "{}", i);
        }
    }
}

#[cfg(test)]
mod integration_tests;
