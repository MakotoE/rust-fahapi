# Folding@home client API wrapper for Rust

[![Latest version](https://img.shields.io/crates/v/fahapi.svg)](https://crates.io/crates/fahapi) [![Documentation](https://docs.rs/fahapi/badge.svg)](https://docs.rs/fahapi/0.1.0/fahapi/)

```rust
let api = fahapi::API::connect_timeout(&DEFAULT_ADDR, std::time::Duration::from_secs(1))?;
api.pause_all()?;
api.unpause_all()?;
```

This library is unstable as FAH presents itself with more surprises every now and then.