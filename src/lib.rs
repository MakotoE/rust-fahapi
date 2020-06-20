use std::net;

mod types;
pub use types::*;

mod connection;
pub use connection::*;

#[derive(Debug)]
pub struct API {
    pub conn: Connection,
    pub buf: Vec<u8>,
}

impl API {
    /// Default TCP address of the FAH client.
    pub fn default_addr() -> net::SocketAddr {
        net::SocketAddr::V4(net::SocketAddrV4::new(net::Ipv4Addr::LOCALHOST, 36330))
    }

    /// Connects to your FAH client with a timeout. Use API::default_addr() to get the default
    /// address.
    pub fn connect_timeout(addr: &net::SocketAddr, timeout: core::time::Duration) -> Result<API> {
        Ok(API {
            conn: Connection::connect_timeout(addr, timeout)?,
            buf: Vec::new(),
        })
    }

    /// Returns a listing of the FAH API commands.
    pub fn help(&mut self) -> Result<String> {
        self.conn.exec("help", &mut self.buf)?;
        Ok(std::str::from_utf8(self.buf.as_slice())?.to_string())
    }

    /// Enables or disables log updates. Returns current log.
    pub fn log_updates(&mut self, arg: LogUpdatesArg) -> Result<String> {
        /*
            This command is weird. It returns the log after the next prompt, like this:
            > log-updates start

            >
            PyON 1 log-update...
        */

        let command = format!("log-updates {}", arg);
        self.conn.exec(command.as_str(), &mut self.buf)?;
        self.conn.exec_eval("eval", &mut self.buf)?;

        // The string contains a bunch of \x00 sequences that are not valid JSON and cannot be
        // parsed using parse_pyon().
        parse_log(std::str::from_utf8(&self.buf)?)
    }

    /// Unpauses all slots which are paused waiting for a screensaver and pause them again on
    /// disconnect.
    pub fn screensaver(&mut self) -> Result<()> {
        self.conn.exec("screensaver", &mut self.buf)
    }

    /// Sets a slot to be always on.
    pub fn always_on(&mut self, slot: i64) -> Result<()> {
        let command = format!("always_on {}", slot);
        self.conn.exec(command.as_str(), &mut self.buf)
    }

    /// Returns true if the client has set a user, team or passkey.
    pub fn configured(&mut self) -> Result<bool> {
        self.conn.exec("configured", &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Runs one client cycle.
    pub fn do_cycle(&mut self) -> Result<()> {
        self.conn.exec("do-cycle", &mut self.buf)
    }

    /// Pauses a slot when its current work unit is completed.
    pub fn finish_slot(&mut self, slot: i64) -> Result<()> {
        let command = format!("finish {}", slot);
        self.conn.exec(command.as_str(), &mut self.buf)
    }

    /// Pauses all slots one-by-one when their current work unit is completed.
    pub fn finish_all(&mut self) -> Result<()> {
        self.conn.exec("finish", &mut self.buf)
    }

    /// Returns FAH build and machine info.
    pub fn info(&mut self) -> Result<serde_json::Value> {
        // TODO create info_struct() to output structured data
        self.conn.exec("info", &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Returns the number of slots.
    pub fn num_slots(&mut self) -> Result<i64> {
        self.conn.exec("num-slots", &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Sets a slot to run only when idle.
    pub fn on_idle<N>(&mut self, slot: N) -> Result<()>
    where
        N: std::fmt::Display,
    {
        let command = format!("on_idle {}", slot);
        self.conn.exec(command.as_str(), &mut self.buf)
    }

    /// Sets all slots to run only when idle.
    pub fn on_idle_all(&mut self) -> Result<()> {
        self.conn.exec("on_idle", &mut self.buf)
    }

    /// Returns the FAH client options.
    pub fn options_get(&mut self) -> Result<Options> {
        self.conn.exec("options -a", &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Sets an option.
    pub fn options_set<N>(&mut self, key: &str, value: N) -> Result<()>
    where
        N: std::fmt::Display,
    {
        let value_str = format!("{}", value);

        if key.contains(&['=', ' ', '!'] as &[char]) || value_str.contains(' ') {
            return Err(format!("key or value contains bad character: {}={}", key, value).into());
        }

        let command = format!("options {}={}", key, value_str);
        self.conn.exec(command.as_str(), &mut self.buf)
    }

    /// Pauses all slots.
    pub fn pause_all(&mut self) -> Result<()> {
        self.conn.exec("pause", &mut self.buf)
    }

    /// Pauses a slot.
    pub fn pause_slot(&mut self, slot: i64) -> Result<()> {
        let command = format!("pause {}", slot);
        self.conn.exec(command.as_str(), &mut self.buf)
    }

    // Returns the total estimated points per day.
    pub fn ppd(&mut self) -> Result<f64> {
        self.conn.exec("ppd", &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Returns info about the current work unit.
    pub fn queue_info(&mut self) -> Result<Vec<SlotQueueInfo>> {
        self.conn.exec("queue-info", &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Requests an ID from the assignment server.
    pub fn request_id(&mut self) -> Result<()> {
        self.conn.exec("request-id", &mut self.buf)
    }

    /// Requests work server assignment from the assignment server.
    pub fn request_ws(&mut self) -> Result<()> {
        self.conn.exec("request-ws", &mut self.buf)
    }

    /// Ends all FAH processes.
    pub fn shutdown(&mut self) -> Result<()> {
        self.conn.exec("shutdown", &mut self.buf)
    }

    /// Returns the simulation information for a slot.
    pub fn simulation_info(&mut self, slot: i64) -> Result<SimulationInfo> {
        // "just like the simulations"
        let command = format!("simulation-info {}", slot);
        self.conn.exec(command.as_str(), &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Deletes a slot.
    pub fn slot_delete(&mut self, slot: i64) -> Result<()> {
        let command = format!("slot-delete {}", slot);
        self.conn.exec(command.as_str(), &mut self.buf)
    }

    /// Returns information about each slot.
    pub fn slot_info(&mut self) -> Result<Vec<SlotInfo>> {
        self.conn.exec("slot-info", &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Returns slot options.
    pub fn slot_options_get(&mut self, slot: i64) -> Result<SlotOptions> {
        let command = format!("slot-options {} -a", slot);
        self.conn.exec(command.as_str(), &mut self.buf)?;
        let s = std::str::from_utf8(&self.buf)?;
        Ok(serde_json::from_str(pyon_to_json(s)?.as_str())?)
    }

    /// Sets slot option.
    pub fn slot_options_set<N>(&mut self, slot: i64, key: &str, value: N) -> Result<()>
    where
        N: std::fmt::Display,
    {
        let command = format!("slot-options {} {} {}", slot, key, value);
        self.conn.exec(command.as_str(), &mut self.buf)
    }

    /// Unpauses all slots.
    pub fn unpause_all(&mut self) -> Result<()> {
        self.conn.exec("unpause", &mut self.buf)
    }

    /// Unpauses a slot.
    pub fn unpause_slot(&mut self, slot: i64) -> Result<()> {
        let command = format!("unpause {}", slot);
        self.conn.exec(command.as_str(), &mut self.buf)
    }

    /// Returns FAH uptime.
    pub fn uptime(&mut self) -> Result<FAHDuration> {
        self.conn.exec_eval("uptime", &mut self.buf)?;
        let duration = parse_duration::parse(std::str::from_utf8(&self.buf)?)?;
        match chrono::Duration::from_std(duration) {
            Ok(d) => Ok(d.into()),
            Err(e) => Err(e.to_string().into()),
        }
    }

    /// Blocks until all slots are paused.
    pub fn wait_for_units(&mut self) -> Result<()> {
        self.conn.exec("wait-for-units", &mut self.buf)
    }
}

error_chain::error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        IO(std::io::Error);
        UTF8(std::str::Utf8Error);
        JSON(serde_json::Error);
    }

    errors {
        EOF {
            description("EOF; the command might have been invalid")
        }
    }
}

impl From<parse_duration::parse::Error> for Error {
    fn from(e: parse_duration::parse::Error) -> Self {
        Error::from(ErrorKind::Msg(e.to_string()))
    }
}

pub enum LogUpdatesArg {
    Start,
    Restart,
    Stop,
}

impl std::fmt::Display for LogUpdatesArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
        let s = match self {
            LogUpdatesArg::Start => "start",
            LogUpdatesArg::Restart => "restart",
            LogUpdatesArg::Stop => "stop",
        };
        write!(f, "{}", s)
    }
}

pub fn parse_log(s: &str) -> Result<String> {
    // The log looks like this: PyON 1 log-update\n"..."\n---\n\n
    const SUFFIX: &str = "\n---\n\n";

    let mut removed_suffix = s;
    if s.len() > SUFFIX.len() && s[s.len() - SUFFIX.len()..] == *SUFFIX {
        removed_suffix = &s[..s.len() - SUFFIX.len()]
    }

    let start = match removed_suffix.find('\n') {
        Some(i) => i + 1,
        None => 0,
    };
    parse_pyon_string(&removed_suffix[start..])
}

#[allow(clippy::iter_nth_zero)]
pub fn parse_pyon_string(s: &str) -> Result<String> {
    if s.len() < 2 || s.bytes().nth(0).unwrap() != b'"' || s.bytes().nth_back(0).unwrap() != b'"' {
        return Err(format!("cannot parse {}", s).into());
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
                }
                _ => capture.to_string(),
            };
        }

        capture.to_string()
    };

    Ok((*MATCH_ESCAPED.replace_all(&s[1..s.len() - 1], replace_fn)).to_string())
}

pub fn pyon_to_json(s: &str) -> Result<String> {
    // https://pypi.org/project/pon/
    const PREFIX: &str = "PyON";
    const SUFFIX: &str = "\n---";
    if s.len() < PREFIX.len()
        || s.bytes().take(PREFIX.len()).partial_cmp(PREFIX.bytes()) != Some(core::cmp::Ordering::Equal)
        || s.len() < SUFFIX.len()
        || s.bytes().skip(s.len() - SUFFIX.len()).partial_cmp(SUFFIX.bytes()) != Some(core::cmp::Ordering::Equal)
    {
        return Err(format!("invalid PyON format: {}", s).into());
    }

    let mut start = match s.find('\n') {
        Some(i) => i + 1,
        None => 0,
    };

    let end = s.len() - SUFFIX.len();
    if start > end {
        start = end;
    }

    Ok(match &s[start..end] {
        "True" => "true".to_string(),
        "False" => "false".to_string(),
        _ => s[start..end]
            .replace(": None", r#": """#)
            .replace(": False", ": false")
            .replace(": True", ": true"),
    })
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_log() {
        struct Test {
            s: &'static str,
            expected: &'static str,
            expect_error: bool,
        }

        let tests = vec![
            Test {
                s: "",
                expected: "",
                expect_error: true,
            },
            Test {
                s: r#"PyON 1 log-update"#,
                expected: "",
                expect_error: true,
            },
            Test {
                s: r#""""#,
                expected: "",
                expect_error: false,
            },
            Test {
                s: r#"\n---\n\n"#,
                expected: "",
                expect_error: true,
            },
            Test {
                s: "\n\"\"\n---\n\n",
                expected: "",
                expect_error: false,
            },
            Test {
                s: "PyON 1 log-update\n\n---\n\n",
                expected: "",
                expect_error: true,
            },
            Test {
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

        let tests = vec![
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
            Test {
                s: "",
                expected: "",
                expect_error: true,
            },
            Test {
                s: "PyON",
                expected: "",
                expect_error: true,
            },
            Test {
                s: "PyON\n---",
                expected: "",
                expect_error: false,
            },
            Test {
                s: "PyON\n\n---",
                expected: "",
                expect_error: false,
            },
            Test {
                s: "PyON\n1\n---",
                expected: "1",
                expect_error: false,
            },
            Test {
                s: "PyON\nTrue\n---",
                expected: "true",
                expect_error: false,
            },
            Test {
                s: "PyON\n{\"\": None}\n---",
                expected: "{\"\": \"\"}",
                expect_error: false,
            },
            Test {
                s: "\n}÷ ",
                expected: "",
                expect_error: true,
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

bencher::benchmark_group!(benches, bench_pyon_to_json);
bencher::benchmark_main!(benches);

fn bench_pyon_to_json(b: &mut bencher::Bencher) {
    // test bench_pyon_to_json ... bench:          33 ns/iter (+/- 1)
    b.iter(|| pyon_to_json("PyON\nFalse\n---"))
}

#[cfg(test)]
mod integration_tests;
