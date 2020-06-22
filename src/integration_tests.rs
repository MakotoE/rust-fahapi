use super::*;

lazy_static::lazy_static! {
    static ref API_INSTANCE: std::sync::Mutex<API> = {
        let api = API::connect_timeout(&DEFAULT_ADDR, core::time::Duration::from_secs(1)).unwrap();
        api.conn.conn.set_read_timeout(Some(core::time::Duration::from_secs(10))).unwrap();
        api.conn.conn.set_write_timeout(Some(core::time::Duration::from_secs(10))).unwrap();

        std::sync::Mutex::new(api)
    };
}

#[test]
fn test_help() {
    let result = API_INSTANCE.lock().unwrap().help();
    assert!(!result.unwrap().is_empty());
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_log_updates() {
    let result = API_INSTANCE
        .lock()
        .unwrap()
        .log_updates(LogUpdatesArg::Start);
    assert!(!result.unwrap().is_empty());
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_screensaver() {
    API_INSTANCE.lock().unwrap().screensaver().unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_always_on() {
    API_INSTANCE.lock().unwrap().always_on(0).unwrap();
}

#[test]
fn test_configured() {
    API_INSTANCE.lock().unwrap().configured().unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_do_cycle() {
    API_INSTANCE.lock().unwrap().do_cycle().unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_finish() {
    API_INSTANCE.lock().unwrap().finish_slot(0).unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_finish_all() {
    API_INSTANCE.lock().unwrap().finish_all().unwrap();
}

#[test]
fn test_info() {
    let result = API_INSTANCE.lock().unwrap().info().unwrap();
    assert!(!result.is_empty())
}

#[test]
fn test_num_slots() {
    API_INSTANCE.lock().unwrap().num_slots().unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_on_idle() {
    API_INSTANCE.lock().unwrap().on_idle(0).unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_on_idle_all() {
    API_INSTANCE.lock().unwrap().on_idle_all().unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_options_set_get() {
    let mut api = API_INSTANCE.lock().unwrap();
    assert!(api.options_set("a", "").is_err());

    assert!(api.options_set("power=", Power::PowerNull).is_err());

    let old_options = api.options_get().unwrap();
    assert!(!old_options.log.is_empty());

    api.options_set("power", Power::PowerLight).unwrap();

    let new_options = api.options_get().unwrap();
    assert_eq!(new_options.power, Power::PowerLight);

    api.options_set("power", old_options.power).unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_pause_unpause() {
    let mut api = API_INSTANCE.lock().unwrap();
    api.pause_all().unwrap();
    api.pause_slot(0).unwrap();
    api.unpause_all().unwrap();
    api.unpause_slot(0).unwrap();
}

#[test]
fn test_ppd() {
    API_INSTANCE.lock().unwrap().ppd().unwrap();
}

#[test]
fn test_queue_info() {
    API_INSTANCE.lock().unwrap().queue_info().unwrap();
}

// request_id() and request_ws() causes tests to break

#[test]
fn test_simulation_info() {
    API_INSTANCE.lock().unwrap().simulation_info(0).unwrap();
}

#[test]
fn test_slot_info() {
    API_INSTANCE.lock().unwrap().slot_info().unwrap();
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_slot_options_get_set() {
    let mut api = API_INSTANCE.lock().unwrap();
    assert!(api.slot_options_get(-1).is_err());

    let old_options = api.slot_options_get(0).unwrap();
    assert!(!old_options.machine_id.is_empty());

    api.slot_options_set(0, "paused", false).unwrap();

    let new_options = api.slot_options_get(0).unwrap();
    assert!(!new_options.paused.0);

    api.slot_options_set(0, "paused", old_options.paused)
        .unwrap();
}

#[test]
fn test_uptime() {
    API_INSTANCE.lock().unwrap().uptime().unwrap();
}

#[test]
fn test_exec() {
    let mut api = API_INSTANCE.lock().unwrap();
    let mut buf: Vec<u8> = Vec::new();
    api.conn.exec("", &mut buf).unwrap();
    assert!(buf.is_empty());

    api.conn.exec("\n", &mut buf).expect_err("");
    assert!(buf.is_empty());
}

#[test]
fn test_exec_eval() {
    let mut api = API_INSTANCE.lock().unwrap();
    let mut buf: Vec<u8> = Vec::new();
    api.conn.exec_eval("", &mut buf).unwrap();
    assert!(buf.is_empty());

    api.conn.exec_eval("date", &mut buf).unwrap();
    assert!(!buf.is_empty());
}
