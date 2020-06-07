use super::*;

lazy_static::lazy_static! {
    static ref API_INSTANCE: std::sync::Mutex<API> = {
        let api = API::connect_timeout(
            &API::default_addr(),
            core::time::Duration::from_secs(1),
        ).unwrap();
        std::sync::Mutex::new(api)
    };
}

#[test]
fn test_help() {
    let result = API_INSTANCE.lock().unwrap().help();
    assert!(result.is_ok());
    assert!(result.unwrap().len() > 0);
}