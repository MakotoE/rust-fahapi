use super::*;

lazy_static::lazy_static! {
    static ref API_INSTANCE: std::sync::Mutex<API> = {
        let api = API::connect_timeout(
            &API::default_addr(),
            core::time::Duration::from_millis(100),
        ).unwrap();
        api.conn.set_read_timeout(Some(core::time::Duration::from_millis(100))).unwrap();
        api.conn.set_write_timeout(Some(core::time::Duration::from_millis(100))).unwrap();

        std::sync::Mutex::new(api)
    };
}

#[test]
fn test_help() {
    let result = API_INSTANCE.lock().unwrap().help();
    assert!(result.unwrap().len() > 0);
}

#[test]
#[cfg_attr(not(feature = "all-tests"), ignore)]
fn test_log_updates() {
    let result = API_INSTANCE
        .lock()
        .unwrap()
        .log_updates(LogUpdatesArg::Start);
    assert!(result.unwrap().len() > 0);
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
    assert!(result.as_array().unwrap().len() > 0)
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
    assert!(api.options_set("power=", Power::PowerNull).is_err());

    let old_options = api.options_get().unwrap();
    assert!(old_options.log.len() > 0);

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

#[test]
fn test_simulation_info() {
    API_INSTANCE.lock().unwrap().simulation_info(0).unwrap();
}

#[test]
fn test_slot_info() {
    API_INSTANCE.lock().unwrap().slot_info().unwrap();
}

#[test]
fn test_uptime() {
    API_INSTANCE.lock().unwrap().uptime().unwrap();
}

#[test]
fn test_exec() {
    let mut api = API_INSTANCE.lock().unwrap();
    let mut buf: Vec<u8> = Vec::new();
    exec(&mut api.conn, "", &mut buf).unwrap();
    assert!(buf.is_empty());

    exec(&mut api.conn, "\n", &mut buf).expect_err("");
    assert!(buf.is_empty());
}

#[test]
fn test_exec_eval() {
    let mut api = API_INSTANCE.lock().unwrap();
    let mut buf: Vec<u8> = Vec::new();
    exec_eval(&mut api.conn, "", &mut buf).unwrap();
    assert!(buf.is_empty());

    exec_eval(&mut api.conn, "date", &mut buf).unwrap();
    assert!(!buf.is_empty());
}
