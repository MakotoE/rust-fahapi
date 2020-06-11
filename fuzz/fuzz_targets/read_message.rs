#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    use bytes::buf::ext::BufExt;
    let mut reader = bytes::Bytes::from(data.to_vec()).reader();
    let mut buf: Vec<u8> = Vec::new();
    let _ = fahapi::read_message(&mut reader, &mut buf);
});
