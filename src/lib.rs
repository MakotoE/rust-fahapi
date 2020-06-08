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

    /// Connects to your FAH client with a timeout. Use API::default_addr() to get the default
    /// address.
    pub fn connect_timeout(addr: &net::SocketAddr, timeout: core::time::Duration) ->
    std::io::Result<API> {
        let mut conn = net::TcpStream::connect_timeout(addr, timeout)?;
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

    /// Enables or disables log updates. Returns current log.
    pub fn logUpdates(&mut self, arg: LogUpdatesArg) -> Result<String, Box<dyn std::error::Error>> {
        /*
            This command is weird. It returns the log after the next prompt, like this:
            > log-updates start

            >
            PyON 1 log-update...
        */

        exec(&mut self.conn, format!("log-updates {}", arg).as_str(), &mut self.buf)?;
        exec_eval(&mut self.conn, "eval", &mut self.buf)?;

        unimplemented!()
    }
}

#[derive(Debug, snafu::Snafu, Clone)]
pub enum Error {
    #[snafu(display("command contains newline"))]
    CommandContainsNewline,

    #[snafu(display("not a valid PyON string"))]
    NotValidPyONString,
}

pub enum LogUpdatesArg {
    Start,
    Restart,
    Stop,
}

impl std::fmt::Display for LogUpdatesArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let s = match self {
            LogUpdatesArg::Start => "Start",
            LogUpdatesArg::Restart => "Restart",
            LogUpdatesArg::Stop => "Stop",
        };
        write!(f, "{}", s)
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
        if r.read(&mut b)? == 0 {
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

pub fn parse_pyon_string(b: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    if b.len() < 2 || b[0] != b'"' || b[b.len() - 1] != b'"' {
        return Err(Box::new(Error::NotValidPyONString))
    }

    lazy_static::lazy_static! {
        static ref MATCH_ESCAPED: regex::bytes::Regex
            = regex::bytes::Regex::new(r#"\\x..|\\n|\\r|\\"|\\\\"#).unwrap();
    }

    let replace_fn: fn(&regex::bytes::Captures) -> Vec<u8> = |caps: &regex::bytes::Captures| {
        let capture = &caps[0];
        if capture[0] == b'\\' {
            return match capture[1] {
                b'n' => b"\n".to_vec(),
                b'r' => b"\r".to_vec(),
                b'"' => b"\"".to_vec(),
                b'\\' => b"\\".to_vec(),
                b'x' => {
                    assert_eq!(capture.len(), 4);
                    
                    let s = match std::str::from_utf8(&capture[2..]) {
                        Ok(s) => s,
                        Err(e) => return capture.to_vec(),
                    };
                    let n: u32 = str::parse(s).unwrap();
                    match std::char::from_u32(n) {
                        Some(c) => c.to_string().as_str().as_bytes().to_vec(),
                        None => capture.to_vec(),
                    }
                },
                _ => capture.to_vec(),
            };
        }
        
        capture.to_vec()
    };
    
    match std::str::from_utf8(&*MATCH_ESCAPED.replace_all(&b[1..b.len()-1], replace_fn)) {
        Ok(s) => Ok(s.to_string()),
        Err(e) => Err(Box::new(e)),
    }
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

    #[test]
    fn test_parse_pyon_string() {
        struct Test {
            s: &'static str,
            expected: &'static str,
            expect_error: bool,
        }

        let tests= vec![
            Test {
                s: "",
                expected: "",
                expect_error: true,
            },
            Test {
                s: r#""""#,
                expected: "",
                expect_error: false,
            },
            Test {
                s: r#""\n\"\\\x01""#,
                expected: "\n\"\\\x01",
                expect_error: false,
            },
            Test {
                s: r#""a\x01a""#,
                expected: "a\x01a",
                expect_error: false,
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let result = parse_pyon_string(test.s.as_bytes());
            assert_eq!(result.is_err(), test.expect_error, "{}", i);
            if !test.expect_error {
                assert_eq!(result.unwrap(), test.expected, "{}", i);
            }
        }
    }
}

#[cfg(test)]
mod integration_tests;
