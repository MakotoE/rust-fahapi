use super::*;

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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Power {
    PowerNull,
    PowerLight,
    PowerMedium,
    PowerFull,
}

impl Power {
    fn new(s: &str) -> Result<Self> {
        Ok(match s.to_uppercase().as_str() {
            "" => Power::PowerNull,
            "LIGHT" => Power::PowerLight,
            "MEDIUM" => Power::PowerMedium,
            "FULL" => Power::PowerFull,
            _ => return Err(format!("s is invalid: {}", s).into()),
        })
    }
}

impl std::default::Default for Power {
    fn default() -> Self {
        Power::PowerNull
    }
}

impl From<&str> for Power {
    /// Panics if s is invalid.
    fn from(s: &str) -> Self {
        Self::new(s).unwrap()
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
        Self::new(serde::de::Deserialize::deserialize(deserializer)?)
            .map_err(|e| serde::de::Error::custom(e.to_string()))
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
        let s = serde::de::Deserialize::deserialize(deserializer)?;
        if s == UNKNOWN_TIME {
            return Ok(None.into());
        }

        // TODO don't rely on parse_duration since it has a terrible worst-case performance
        match parse_duration::parse(s) {
            Ok(d) => match chrono::Duration::from_std(d) {
                Ok(d) => Ok(d.into()),
                Err(e) => Err(serde::de::Error::custom(e.to_string())),
            },
            Err(e) => Err(serde::de::Error::custom(e.to_string())),
        }
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
