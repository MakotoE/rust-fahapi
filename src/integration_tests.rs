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
    let result = API_INSTANCE.lock().unwrap()
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