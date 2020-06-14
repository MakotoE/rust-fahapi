#[derive(serde::Deserialize, std::default::Default)]
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
    pub checkpoint: String,
    pub child: StringBool,
    pub client_subtype: String,
    pub client_threads: String,
    pub client_type: String,
    pub command_address: String,
    pub command_allow_no_pass: String,
    pub deny: String,
    pub command_deny_no_pass: String,
    pub command_enable: StringBool,
    pub command_port: String,
    pub config_rotate: StringBool,
    pub config_rotate_dir: String,
    pub config_rotate_max: String,
    pub connection_timeout: String,
    pub core_priority: String,
    pub cpu_species: String,
    pub cpu_type: String,
    pub cpu_usage: String,
    pub cpus: String,
    pub crl_file: String,
    pub cuda_index: String,
    pub cycle_rate: String,
    pub cycles: String,
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
    pub gpu_usage: String,
    pub gui_enabled: String,
    pub http_addresses: String,
    pub https_addresses: String,
    pub idle: StringBool,
    pub log: String,
    pub log_color: StringBool,
    pub log_crlf: StringBool,
    pub log_date: StringBool,
    pub log_date_periodically: String,
    pub log_domain: StringBool,
    pub log_domain_levels: String,
    pub log_header: StringBool,
    pub log_level: StringBool,
    pub log_no_info_header: StringBool,
    pub log_redirect: StringBool,
    pub log_rotate: StringBool,
    pub log_rotate_dir: String,
    pub log_rotate_max: String,
    pub log_short_level: StringBool,
    pub log_simple_domains: StringBool,
    pub log_thread_id: StringBool,
    pub log_thread_prefix: StringBool,
    pub log_time: StringBool,
    pub log_to_screen: StringBool,
    pub log_truncate: StringBool,
    pub machine_id: String,
    pub max_connect_time: String,
    pub max_connections: String,
    pub max_packet_size: String,
    pub max_queue: String,
    pub max_request_length: String,
    pub max_shutdown_wait: String,
    pub max_slot_errors: String,
    pub max_unit_errors: String,
    pub max_units: String,
    pub memory: String,
    pub min_connect_time: String,
    pub next_unit_percentage: String,
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
    pub power: String,
    pub private_key_file: String,
    pub project_key: String,
    pub proxy: String,
    pub proxy_enable: StringBool,
    pub proxy_pass: String,
    pub proxy_user: String,
    pub respawn: StringBool,
    pub service: StringBool,
    pub service_description: String,
    pub service_restart: StringBool,
    pub service_restart_delay: String,
    pub session_cookie: String,
    pub session_lifetime: String,
    pub session_timeout: String,
    pub smp: StringBool,
    pub stack_traces: StringBool,
    pub stall_detection_enabled: StringBool,
    pub stall_percent: String,
    pub stall_timeout: String,
    pub team: String,
    pub user: String,
    pub verbosity: String,
    pub web_allow: String,
    pub web_deny: String,
    pub web_enable: StringBool,
}

pub struct StringBool(bool);

impl std::default::Default for StringBool {
    fn default() -> Self {
        Self(false)
    }
}

impl<'de> serde::de::Deserialize<'de> for StringBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let b: &[u8] = serde::de::Deserialize::deserialize(deserializer)?;
        match serde_json::from_slice(b) {
            Ok(b) => Ok(StringBool(b)),
            Err(e) => {
                let msg = match std::str::from_utf8(b) {
                    Ok(s) => format!("invalid StringBool: {}", s),
                    Err(_) => format!("invalid StringBool: {:?}", b),
                };
                Err(serde::de::Error::custom(msg))
            },
        }
    }
}