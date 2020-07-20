# Folding@home client API wrapper for Rust

[![Latest version](https://img.shields.io/crates/v/fahapi.svg)](https://crates.io/crates/fahapi) [![Documentation](https://docs.rs/fahapi/badge.svg)](https://docs.rs/fahapi/)

```rust
fn example() -> fahapi::Result<()> {
    let mut api = fahapi::API::connect_timeout(&fahapi::DEFAULT_ADDR, std::time::Duration::from_secs(1))?;
    api.pause_all()?;
    api.unpause_all()
}
```

This is a Rust port of [go-fahapi](https://github.com/MakotoE/go-fahapi).

This library is unstable as FAH presents itself with more surprises every now and then.
