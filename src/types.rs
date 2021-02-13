use super::*;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Options {
    pub allow: String,
    pub capture_directory: String,
    pub capture_on_error: StringBool,
    pub capture_packets: StringBool,
    pub capture_requests: StringBool,
    pub capture_responses: StringBool,
    pub capture_sockets: StringBool,
    pub cause: String,
    pub certificate_file: String,
    pub checkpoint: StringInt,
    pub child: StringBool,
    pub client_subtype: String,
    pub client_threads: StringInt,
    pub client_type: String,
    pub command_address: String,
    pub command_allow_no_pass: String,
    pub deny: String,
    pub command_deny_no_pass: String,
    pub command_enable: StringBool,
    pub command_port: StringInt,
    pub config_rotate: StringBool,
    pub config_rotate_dir: String,
    pub config_rotate_max: StringInt,
    pub connection_timeout: StringInt,
    pub core_priority: String,
    pub cpu_species: String,
    pub cpu_type: String,
    pub cpu_usage: StringInt,
    pub cpus: StringInt,
    pub crl_file: String,
    pub cuda_index: String,
    pub cycle_rate: StringInt,
    pub cycles: StringInt,
    pub daemon: StringBool,
    pub debug_sockets: StringBool,
    pub disable_sleep_when_active: StringBool,
    pub disable_viz: StringBool,
    pub dump_after_deadline: StringBool,
    pub exception_locations: StringBool,
    pub exit_when_done: StringBool,
    pub extra_core_args: String,
    pub fold_anon: String,
    pub gpu: String,
    pub gpu_index: String,
    pub gpu_usage: StringInt,
    pub gui_enabled: String,
    pub http_addresses: String,
    pub https_addresses: String,
    pub idle: StringBool,
    pub log: String,
    pub log_color: StringBool,
    pub log_crlf: StringBool,
    pub log_date: StringBool,
    pub log_date_periodically: StringInt,
    pub log_domain: StringBool,
    pub log_domain_levels: String,
    pub log_header: StringBool,
    pub log_level: StringBool,
    pub log_no_info_header: StringBool,
    pub log_redirect: StringBool,
    pub log_rotate: StringBool,
    pub log_rotate_dir: String,
    pub log_rotate_max: StringInt,
    pub log_short_level: StringBool,
    pub log_simple_domains: StringBool,
    pub log_thread_id: StringBool,
    pub log_thread_prefix: StringBool,
    pub log_time: StringBool,
    pub log_to_screen: StringBool,
    pub log_truncate: StringBool,
    pub machine_id: StringInt,
    pub max_connect_time: StringInt,
    pub max_connections: StringInt,
    pub max_packet_size: String,
    pub max_queue: StringInt,
    pub max_request_length: StringInt,
    pub max_shutdown_wait: StringInt,
    pub max_slot_errors: StringInt,
    pub max_unit_errors: StringInt,
    pub max_units: StringInt,
    pub memory: String,
    pub min_connect_time: StringInt,
    pub next_unit_percentage: StringInt,
    pub priority: String,
    pub no_assembly: StringBool,
    pub open_web_control: StringBool,
    pub opencl_index: String,
    pub os_species: String,
    pub os_type: String,
    pub passkey: String,
    pub password: String,
    pub pause_on_battery: StringBool,
    pub pause_on_start: StringBool,
    pub paused: StringBool,
    pub pid: StringBool,
    pub pid_file: String,
    pub power: Power,
    pub private_key_file: String,
    pub project_key: StringInt,
    pub proxy: String,
    pub proxy_enable: StringBool,
    pub proxy_pass: String,
    pub proxy_user: String,
    pub respawn: StringBool,
    pub service: StringBool,
    pub service_description: String,
    pub service_restart: StringBool,
    pub service_restart_delay: StringInt,
    pub session_cookie: String,
    pub session_lifetime: StringInt,
    pub session_timeout: StringInt,
    pub smp: StringBool,
    pub stack_traces: StringBool,
    pub stall_detection_enabled: StringBool,
    pub stall_percent: StringInt,
    pub stall_timeout: StringInt,
    pub team: StringInt,
    pub user: String,
    pub verbosity: StringInt,
    pub web_allow: String,
    pub web_deny: String,
    pub web_enable: StringBool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct StringBool(pub bool);

impl From<bool> for StringBool {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl std::fmt::Display for StringBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> serde::de::Deserialize<'de> for StringBool {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match serde_json::from_str(serde::de::Deserialize::deserialize(deserializer)?) {
            Ok(result) => Ok(Self(result)),
            Err(e) => Err(serde::de::Error::custom(e.to_string())),
        }
    }
}

impl core::str::FromStr for StringBool {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(str::parse(s)?))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct StringInt(pub i64);

impl From<i64> for StringInt {
    fn from(n: i64) -> Self {
        Self(n)
    }
}

impl std::fmt::Display for StringInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> serde::de::Deserialize<'de> for StringInt {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match serde_json::from_str(serde::de::Deserialize::deserialize(deserializer)?) {
            Ok(n) => Ok(Self(n)),
            Err(e) => Err(serde::de::Error::custom(e.to_string())),
        }
    }
}

impl core::str::FromStr for StringInt {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(str::parse(s)?))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Power {
    PowerNull,
    PowerLight,
    PowerMedium,
    PowerFull,
}

impl std::default::Default for Power {
    fn default() -> Self {
        Power::PowerNull
    }
}

impl core::str::FromStr for Power {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.to_uppercase().as_str() {
            "" => Power::PowerNull,
            "LIGHT" => Power::PowerLight,
            "MEDIUM" => Power::PowerMedium,
            "FULL" => Power::PowerFull,
            _ => return Err(Error::msg(format!("s is invalid: {}", s))),
        })
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Power::PowerNull => "",
            Power::PowerLight => "LIGHT",
            Power::PowerMedium => "MEDIUM",
            Power::PowerFull => "FULL",
        };
        write!(f, "{}", s)
    }
}

impl<'de> serde::de::Deserialize<'de> for Power {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match str::parse(serde::de::Deserialize::deserialize(deserializer)?) {
            Ok(p) => Ok(p),
            Err(e) => Err(serde::de::Error::custom(e.to_string())),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct SlotQueueInfo {
    pub id: String,
    pub state: String,
    pub error: String,
    pub project: i64,
    pub run: i64,
    pub clone: i64,
    pub gen: i64,
    pub core: String,
    pub unit: String,
    pub percent_done: String,
    pub eta: FAHDuration,
    pub ppd: StringInt,
    pub credit_estimate: StringInt,
    pub waiting_on: String,
    pub next_attempt: FAHDuration,
    pub time_remaining: FAHDuration,
    pub total_frames: i64,
    pub frames_done: i64,
    pub assigned: FAHTime,
    pub timeout: FAHTime,
    pub deadline: FAHTime,
    pub ws: String,
    pub cs: String,
    pub attempts: i64,
    pub slot: String,
    pub tpf: FAHDuration,
    pub base_credit: StringInt,
}

/// None means invalid time.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct FAHTime(pub Option<chrono::DateTime<chrono::offset::Utc>>);

impl From<Option<chrono::DateTime<chrono::offset::Utc>>> for FAHTime {
    fn from(t: Option<chrono::DateTime<chrono::offset::Utc>>) -> Self {
        Self(t)
    }
}

impl From<chrono::DateTime<chrono::offset::Utc>> for FAHTime {
    fn from(t: chrono::DateTime<chrono::offset::Utc>) -> Self {
        Self(Some(t))
    }
}

const INVALID_TIME: &str = "<invalid>";

impl std::fmt::Display for FAHTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(t) => write!(f, "{}", t),
            None => write!(f, "{}", INVALID_TIME),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for FAHTime {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = serde::de::Deserialize::deserialize(deserializer)?;
        if s == INVALID_TIME {
            return Ok(None.into());
        }

        match chrono::DateTime::parse_from_rfc3339(s) {
            Ok(t) => Ok(t.with_timezone(&chrono::offset::Utc).into()),
            Err(e) => Err(serde::de::Error::custom(e.to_string())),
        }
    }
}

/// None means unknown duration.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct FAHDuration(pub Option<chrono::Duration>);

impl From<Option<chrono::Duration>> for FAHDuration {
    fn from(d: Option<chrono::Duration>) -> Self {
        Self(d)
    }
}

impl From<chrono::Duration> for FAHDuration {
    fn from(d: chrono::Duration) -> Self {
        Self(Some(d))
    }
}

const UNKNOWN_TIME: &str = "unknowntime";

impl std::fmt::Display for FAHDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(d) => write!(f, "{}", d),
            None => write!(f, "{}", UNKNOWN_TIME),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for FAHDuration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
        if s == UNKNOWN_TIME {
            return Ok(None.into());
        }

        // humantime cannot parse "x.x days"
        if let Some(number_of_days) = s.strip_suffix(" days") {
            if number_of_days.contains('.') {
                const MILLIS_PER_DAY: f64 = (1000 * 60 * 60 * 24) as f64;
                let n = f64::from_str(number_of_days)
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                return Ok(
                    chrono::Duration::milliseconds((MILLIS_PER_DAY * n).round() as i64).into(),
                );
            }
        }

        let duration =
            humantime::parse_duration(s).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        let result = chrono::Duration::from_std(duration)
            .map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(result.into())
    }
}

#[derive(Clone, PartialEq, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct SimulationInfo {
    pub user: String,
    pub team: String,
    pub project: i64,
    pub run: i64,
    pub clone: i64,
    pub gen: i64,
    pub core_type: i64,
    pub core: String,
    pub total_iterations: i64,
    pub iterations_done: i64,
    pub energy: i64,
    pub temperature: i64,
    pub start_time: FAHTime,
    pub timeout: i64,
    pub deadline: i64,
    pub eta: i64,
    pub progress: f64,
    pub slot: i64,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct SlotInfo {
    pub id: String,
    pub status: String,
    pub description: String,
    pub options: serde_json::map::Map<String, serde_json::Value>,
    pub reason: String,
    pub idle: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct SlotOptions {
    pub machine_id: String,
    pub paused: StringBool,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, serde::Deserialize)]
pub struct Info {
    pub fah_client: FAHClient,
    pub cbang: CBang,
    pub system: System,
    pub libfah: LibFAH,
}

impl Info {
    pub fn new(src: Vec<Vec<serde_json::Value>>) -> Result<Self> {
        if src.len() < 4 {
            return Err(Error::msg(format!(
                "src should have 4 arrays but has {}",
                src.len()
            )));
        }

        let mut info = Info::default();

        for row in src {
            let mut row_iter = row.iter();
            let field: &mut dyn FieldSetter = match row_iter
                .next()
                .and_then(|e| e.as_str())
                .ok_or_else(|| Error::msg("unexpected type"))?
            {
                "FAHClient" => &mut info.fah_client,
                "CBang" => &mut info.cbang,
                "System" => &mut info.system,
                "libFAH" => &mut info.libfah,
                s => {
                    eprintln!("unexpected row type: {}", s);
                    continue;
                }
            };

            for v in row_iter {
                let entry = v.as_array().ok_or_else(|| Error::msg("unexpected type"))?;
                let k = entry
                    .get(0)
                    .and_then(|k| k.as_str())
                    .ok_or_else(|| Error::msg("unexpected type"))?;
                let v = entry
                    .get(1)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| Error::msg("unexpected type"))?;
                field.set(k, v)?;
            }
        }

        Ok(info)
    }
}

trait FieldSetter {
    fn set(&mut self, k: &str, value: &str) -> Result<()>;
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, serde::Deserialize)]
pub struct FAHClient {
    pub version: String,
    pub author: String,
    pub copyright: String,
    pub homepage: String,
    pub date: String,
    pub time: String,
    pub revision: String,
    pub branch: String,
    pub compiler: String,
    pub options: String,
    pub platform: String,
    pub bits: String,
    pub mode: String,
    pub args: String,
    pub config: String,
}

impl FieldSetter for FAHClient {
    fn set(&mut self, k: &str, value: &str) -> Result<()> {
        let v = value.to_string();
        match k {
            "Version" => self.version = v,
            "Author" => self.author = v,
            "Copyright" => self.copyright = v,
            "Homepage" => self.homepage = v,
            "Date" => self.date = v,
            "Time" => self.time = v,
            "Revision" => self.revision = v,
            "Branch" => self.branch = v,
            "Compiler" => self.compiler = v,
            "Options" => self.options = v,
            "Platform" => self.platform = v,
            "Bits" => self.bits = v,
            "Mode" => self.mode = v,
            "Args" => self.args = v,
            "Config" => self.config = v,
            _ => eprintln!("discarded unknown field: {}", k),
        };
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, serde::Deserialize)]
pub struct CBang {
    pub date: String,
    pub time: String,
    pub revision: String,
    pub branch: String,
    pub compiler: String,
    pub options: String,
    pub platform: String,
    pub bits: String,
    pub mode: String,
}

impl FieldSetter for CBang {
    fn set(&mut self, k: &str, value: &str) -> Result<()> {
        let v = value.to_string();
        match k {
            "Date" => self.date = v,
            "Time" => self.time = v,
            "Revision" => self.revision = v,
            "Branch" => self.branch = v,
            "Compiler" => self.compiler = v,
            "Options" => self.options = v,
            "Platform" => self.platform = v,
            "Bits" => self.bits = v,
            "Mode" => self.mode = v,
            _ => eprintln!("discarded unknown field: {}", k),
        };
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, serde::Deserialize)]
pub struct System {
    pub cpu: String,
    pub cpu_id: String,
    pub cpus: StringInt,
    pub memory: String,
    pub free_memory: String,
    pub threads: String,
    pub os_version: String,
    pub has_battery: String,
    pub on_battery: String,
    pub utc_offset: String,
    pub pid: String,
    pub cwd: String,
    pub os: String,
    pub os_arch: String,
    pub gpus: StringInt,
    // I don't have multiple GPUs so I can't test the "GPU 0" field
}

impl FieldSetter for System {
    fn set(&mut self, k: &str, value: &str) -> Result<()> {
        let v = value.to_string();
        match k {
            "CPU" => self.cpu = v,
            "CPU ID" => self.cpu_id = v,
            "CPUs" => self.cpus = str::parse(value)?,
            "Memory" => self.memory = v,
            "Free Memory" => self.free_memory = v,
            "Threads" => self.threads = v,
            "OS Version" => self.os_version = v,
            "Has Battery" => self.has_battery = v,
            "On Battery" => self.on_battery = v,
            "UTC Offset" => self.utc_offset = v,
            "PID" => self.pid = v,
            "CWD" => self.cwd = v,
            "OS" => self.os = v,
            "OS Arch" => self.os_arch = v,
            "GPUs" => self.gpus = str::parse(value)?,
            _ => {
                if !k.starts_with("GPU") && !k.starts_with("CUDA") && !k.starts_with("OpenCL") {
                    eprintln!("discarded unknown field: {}", k);
                }
            }
        };
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, serde::Deserialize)]
pub struct LibFAH {
    pub date: String,
    pub time: String,
    pub revision: String,
    pub branch: String,
    pub compiler: String,
    pub options: String,
    pub platform: String,
    pub bits: String,
    pub mode: String,
}

impl FieldSetter for LibFAH {
    fn set(&mut self, k: &str, value: &str) -> Result<()> {
        let v = value.to_string();
        match k {
            "Date" => self.date = v,
            "Time" => self.time = v,
            "Revision" => self.revision = v,
            "Branch" => self.branch = v,
            "Compiler" => self.compiler = v,
            "Options" => self.options = v,
            "Platform" => self.platform = v,
            "Bits" => self.bits = v,
            "Mode" => self.mode = v,
            _ => eprintln!("discarded unknown field: {}", k),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_info_new() {
        let src = vec![
            vec![
                serde_json::Value::String("FAHClient".into()),
                serde_json::Value::Array(vec!["Version".into(), "7.6.13".into()]),
            ],
            vec![
                serde_json::Value::String("CBang".into()),
                serde_json::Value::Array(vec!["Date".into(), "Apr 20 2020".into()]),
            ],
            vec![
                serde_json::Value::String("System".into()),
                serde_json::Value::Array(vec![
                    "CPU ID".into(),
                    "Intel Management Engine is a backdoor".into(),
                ]),
                serde_json::Value::Array(vec!["CPUs".into(), "1".into()]),
            ],
            vec![
                serde_json::Value::String("libFAH".into()),
                serde_json::Value::Array(vec!["Date".into(), "Apr 20 2020".into()]),
            ],
        ];

        assert!(Info::new(Vec::new()).is_err());
        let result = Info::new(src).unwrap();
        assert!(!result.fah_client.version.is_empty());
        assert!(!result.system.cpu_id.is_empty());
        assert_eq!(result.system.cpus, StringInt(1));
    }

    #[test]
    fn test_fahduration_deserialize() {
        let s = r#""1 days""#;
        let result: FAHDuration = serde_json::from_str(s).unwrap();
        assert_eq!(result.0.unwrap().num_days(), 1);

        let s = r#""1.1 days""#;
        let result: FAHDuration = serde_json::from_str(s).unwrap();
        assert_eq!(result.0.unwrap().num_days(), 1);
        assert_eq!(result.0.unwrap().num_milliseconds(), 95040000);
    }
}
