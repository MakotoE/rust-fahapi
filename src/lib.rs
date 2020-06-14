use std::net;

mod types;
pub use types::{Options};

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
    pub fn connect_timeout(addr: &net::SocketAddr, timeout: core::time::Duration) -> std::io::Result<API> {
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
    pub fn help(&mut self) -> Result<String, Error> {
        exec(&mut self.conn, "help", &mut self.buf)?;
        match std::str::from_utf8(self.buf.as_slice()) {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(Error::Other{msg: e.to_string()}),
        }
    }

    /// Enables or disables log updates. Returns current log.
    pub fn log_updates(&mut self, arg: LogUpdatesArg) -> Result<String, Error> {
        /*
            This command is weird. It returns the log after the next prompt, like this:
            > log-updates start

            >
            PyON 1 log-update...
        */

        exec(&mut self.conn, format!("log-updates {}", arg).as_str(), &mut self.buf)?;
        exec_eval(&mut self.conn, "eval", &mut self.buf)?;

        let s = match std::str::from_utf8(&mut self.buf) {
            Ok(s) => s,
            Err(e) => return Err(Error::Parse{msg: e.to_string()}),
        };

        // The string contains a bunch of \x00 sequences that are not valid JSON and cannot be
	    // parsed using parse_pyon().
        parse_log(s)
    }

    /// Unpauses all slots which are paused waiting for a screensaver and pause them again on
    /// disconnect.
    pub fn screensaver(&mut self) -> Result<(), Error> {
        exec(&mut self.conn, "screensaver", &mut self.buf)
    }

    /// Sets a slot to be always on.
    pub fn always_on<N>(&mut self, slot: N) -> Result<(), Error> where N: std::fmt::Display {
        exec(&mut self.conn, format!("always_on {}", slot).as_str(), &mut self.buf)
    }

    /// Returns true if the client has set a user, team or passkey.
    pub fn configured(&mut self) -> Result<bool, Error> {
        exec(&mut self.conn, "configured", &mut self.buf)?;

        let s = match std::str::from_utf8(&mut self.buf) {
            Ok(s) => s,
            Err(e) => return Err(Error::Parse{msg: e.to_string()}),
        };

        match serde_json::from_str(pyon_to_json(s)?.as_str()) {
            Ok(b) => Ok(b),
            Err(e) => Err(Error::Parse{msg: e.to_string()})
        }
    }

    /// Runs one client cycle.
    pub fn do_cycle(&mut self) -> Result<(), Error> {
        exec(&mut self.conn, "do-cycle", &mut self.buf)
    }

    /// Downloads a core. NOT TESTED.
    pub fn download_core(&mut self, core_type: &str, url: &str) -> Result<(), Error> {
        let command = format!("download-core {} {}", core_type, url);
        exec(&mut self.conn, command.as_str(), &mut self.buf)
    }

    /// Pauses a slot when its current work unit is completed.
    pub fn finish<N>(&mut self, slot: N) -> Result<(), Error> where N: std::fmt::Display {
        exec(&mut self.conn, format!("finish {}", slot).as_str(), &mut self.buf)
    }

    /// Pauses all slots one-by-one when their current work unit is completed.
    pub fn finish_all(&mut self) -> Result<(), Error> {
        exec(&mut self.conn, "finish", &mut self.buf)
    }

    /// Returns FAH build and machine info.
    pub fn info(&mut self) -> Result<serde_json::Value, Error> { // TODO create info_struct() to output structured data
        exec(&mut self.conn, "info", &mut self.buf)?;

        let s = match std::str::from_utf8(&mut self.buf) {
            Ok(s) => s,
            Err(e) => return Err(Error::Parse{msg: e.to_string()}),
        };

        match serde_json::from_str(pyon_to_json(s)?.as_str()) {
            Ok(b) => Ok(b),
            Err(e) => Err(Error::Parse{msg: e.to_string()})
        }
    }

    /// Returns the number of slots.
    pub fn num_slots(&mut self) -> Result<i32, Error> {
        exec(&mut self.conn, "num-slots", &mut self.buf)?;

        let s = match std::str::from_utf8(&mut self.buf) {
            Ok(s) => s,
            Err(e) => return Err(Error::Parse{msg: e.to_string()}),
        };

        match serde_json::from_str(pyon_to_json(s)?.as_str()) {
            Ok(b) => Ok(b),
            Err(e) => Err(Error::Parse{msg: e.to_string()})
        }
    }

    /// Sets a slot to run only when idle.
    pub fn on_idle<N>(&mut self, slot: N) -> Result<(), Error> where N: std::fmt::Display {
        exec(&mut self.conn, format!("on_idle {}", slot).as_str(), &mut self.buf)
    }

    /// Sets all slots to run only when idle.
    pub fn on_idle_all(&mut self) -> Result<(), Error> {
        exec(&mut self.conn, "on_idle", &mut self.buf)
    }

    /// Returns the FAH client options.
    pub fn options_get(&mut self) -> Result<Options, Error> {
        exec(&mut self.conn, "options -a", &mut self.buf)?;

        let s = match std::str::from_utf8(&mut self.buf) {
            Ok(s) => s,
            Err(e) => return Err(Error::Parse{msg: e.to_string()}),
        };

        match serde_json::from_str(pyon_to_json(s)?.as_str()) {
            Ok(b) => Ok(b),
            Err(e) => Err(Error::Parse{msg: e.to_string()})
        }
    }
}

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    #[snafu(display("command contains newline"))]
    CommandContainsNewline,

    #[snafu(display("IO error: {}", e))]
    IO{
        e: std::io::Error,
    },

    #[snafu(display("parse error: {}", msg))]
    Parse{
        msg: String,
    },

    #[snafu(display("{}", msg))]
    Other{
        msg: String,
    },
}

pub enum LogUpdatesArg {
    Start,
    Restart,
    Stop,
}

impl std::fmt::Display for LogUpdatesArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let s = match self {
            LogUpdatesArg::Start => "start",
            LogUpdatesArg::Restart => "restart",
            LogUpdatesArg::Stop => "stop",
        };
        write!(f, "{}", s)
    }
}

/// Executes a command on the FAH client. The response is written to the buffer.
pub fn exec(conn: &mut net::TcpStream, command: &str, buf: &mut Vec<u8>) -> Result<(), Error> {
    use std::io::Write;

    if command == "" {
        // FAH doesn't respond to an empty command
        buf.clear();
        return Ok(());
    }

    if command.contains("\n") {
        return Err(Error::CommandContainsNewline);
    }

    conn.write_all(format!("{}\n", command).as_bytes()).map_err(|e| Error::IO{e})?;

    read_message(conn, buf).map_err(|e| Error::IO{e})
}

/// Executes commands which do not return a trailing newline. (Some commands don't end their message
/// and cause infinite blocking.) The response is written to the buffer.
pub fn exec_eval(conn: &mut net::TcpStream, command: &str, buf: &mut Vec<u8>) -> Result<(), Error> {
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

pub fn read_message(r: &mut impl std::io::Read, buf: &mut Vec<u8>) -> std::io::Result<()> {
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

pub fn parse_log(s: &str) -> Result<String, Error> {
    // The log looks like this: PyON 1 log-update\n"..."\n---\n\n
    const SUFFIX: &str = "\n---\n\n";

    let mut removed_suffix = s;
    if s.len() > SUFFIX.len() && s[s.len() - SUFFIX.len()..] == *SUFFIX {
        removed_suffix = &s[..s.len() - SUFFIX.len()]
    }

    let start = match removed_suffix.find('\n') {
       Some(i) => i+1,
       None => 0, 
    };
    parse_pyon_string(&removed_suffix[start..])
}

pub fn parse_pyon_string(s: &str) -> Result<String, Error> {
    if s.len() < 2 || s.bytes().nth(0).unwrap() != b'"' || s.bytes().nth_back(0).unwrap() != b'"' {
        return Err(Error::Parse{msg: "".to_string()})
    }

    lazy_static::lazy_static! {
        static ref MATCH_ESCAPED: regex::Regex = regex::Regex::new(r#"\\x..|\\n|\\r|\\"|\\\\"#).unwrap();
    }

    let replace_fn: fn(&regex::Captures) -> String = |caps: &regex::Captures| {
        let capture = &caps[0];
        if capture.bytes().nth(0).unwrap() == b'\\' {
            return match capture.bytes().nth(1).unwrap() {
                b'n' => "\n".to_string(),
                b'r' => "\r".to_string(),
                b'"' => "\"".to_string(),
                b'\\' => "\\".to_string(),
                b'x' => {
                    let hex: String = capture.chars().skip(2).collect();
                    let n = match u32::from_str_radix(hex.as_str(), 16) {
                        Ok(n) => n,
                        Err(_) => return capture.to_string(),
                    };

                    match std::char::from_u32(n) {
                        Some(c) => c.to_string(),
                        None => capture.to_string(),
                    }
                },
                _ => capture.to_string(),
            };
        }
        
        capture.to_string()
    };
    
    Ok((*MATCH_ESCAPED.replace_all(&s[1..s.len()-1], replace_fn)).to_string())
}

pub fn pyon_to_json(s: &str) -> Result<String, Error> {
    // https://pypi.org/project/pon/
    const PREFIX: &str = "PyON";
    const SUFFIX: &str = "\n---";
    if s.len() < PREFIX.len() || &s[..PREFIX.len()] != PREFIX
        || s.len() < SUFFIX.len() || &s[s.len()-SUFFIX.len()..] != SUFFIX {
        return Err(Error::Parse{msg: format!("invalid PyON format: {}", s)})
    }

    let mut start = match s.find('\n') {
        Some(i) => i+1,
        None => 0, 
    };
    
    let end = s.len() - SUFFIX.len();
    if start > end {
        start = end;
    }

    Ok(s[start..end].replace("None", "\"\"") // TODO optimize
        .replace("False", "false")
        .replace("True", "true"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_message() {
        struct Test {
            s: &'static [u8],
            expected: &'static [u8],
        }

        let tests = vec![
            Test{
                s: b"\n> ",
                expected: b"",
            },
            Test{
                s: b"a\n> ",
                expected: b"a",
            },
            Test{
                s: b"a\n> \n> ",
                expected: b"a",
            },
            Test{
                s: b"\na\n> ",
                expected: b"a",
            },
        ];

        let mut buf: Vec<u8> = Vec::new();
        for (i, test) in tests.iter().enumerate() {
            use bytes::buf::ext::BufExt;
            read_message(&mut bytes::Bytes::from_static(test.s).reader(), &mut buf).unwrap();
            assert_eq!(buf.as_slice(), test.expected, "{}", i);
        }
    }

    #[test]
    fn test_parse_log() {
        struct Test {
            s: &'static str,
            expected: &'static str,
            expect_error: bool,
        }

        let tests = vec![
            Test{
                s: "",
                expected: "",
                expect_error: true,
            },
            Test{
                s: r#"PyON 1 log-update"#,
                expected: "",
                expect_error: true,
            },
            Test{
                s: r#""""#,
                expected: "",
                expect_error: false,
            },
            Test{
                s: r#"\n---\n\n"#,
                expected: "",
                expect_error: true,
            },
            Test{
                s: "\n\"\"\n---\n\n",
                expected: "",
                expect_error: false,
            },
            Test{
                s: "PyON 1 log-update\n\n---\n\n",
                expected: "",
                expect_error: true,
            },
            Test{
                s: "PyON 1 log-update\n\"a\"\n---\n\n",
                expected: "a",
                expect_error: false,
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let result = parse_log(test.s);
            assert_eq!(result.is_err(), test.expect_error, "{}", i);
            if !test.expect_error {
                assert_eq!(result.unwrap(), test.expected, "{}", i);
            }
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
            Test{
                s: "",
                expected: "",
                expect_error: true,
            },
            Test{
                s: r#""""#,
                expected: "",
                expect_error: false,
            },
            Test{
                s: r#""\n\"\\\x01""#,
                expected: "\n\"\\\x01",
                expect_error: false,
            },
            Test{
                s: r#""a\x01a""#,
                expected: "a\x01a",
                expect_error: false,
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let result = parse_pyon_string(test.s);
            assert_eq!(result.is_err(), test.expect_error, "{}", i);
            if !test.expect_error {
                assert_eq!(result.unwrap(), test.expected, "{}", i);
            }
        }
    }

    #[test]
    fn test_pyon_to_json() {
        struct Test {
            s: &'static str,
            expected: &'static str,
            expect_error: bool,
        }

        let tests = vec![
            Test{
                s: "",
                expected: "",
                expect_error: true,
            },
            Test{
                s: "PyON",
                expected: "",
                expect_error: true,
            },
            Test{
                s: "PyON\n---",
                expected: "",
                expect_error: false,
            },
            Test{
                s: "PyON\n\n---",
                expected: "",
                expect_error: false,
            },
            Test{
                s: "PyON\n1\n---",
                expected: "1",
                expect_error: false,
            },
            Test{
                s: "PyON\nTrue\n---",
                expected: "true",
                expect_error: false,
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let result = pyon_to_json(test.s);
            assert_eq!(result.is_err(), test.expect_error, "{}", i);
            if !test.expect_error {
                assert_eq!(result.unwrap(), test.expected, "{}", i);
            }
        }
    }
}

#[cfg(test)]
mod integration_tests;
