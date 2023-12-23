# Dygma Focus API (Rust)

[<img alt="crates.io" src="https://img.shields.io/crates/v/dygma_focus?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dygma_focus)

## About

This crate is a Rust implementation of the Dygma Focus API.

Make sure to not have Bazecor running and connected while trying to communicate with your keyboard.

## Usage

Cargo.toml

```toml
[dependencies]
anyhow = "1.0"
dygma_focus = "0.1"
```

main.rs

```rust
use anyhow::Result;
use dygma_focus::prelude::*;

fn main() -> Result<()> {
    // Open the first device found and declare as mutable
    // Other constructors are under Focus::new_*
    let mut focus = Focus::new_first_available()?;

    // Call whatever you want here
    let response = focus.wireless_rf_power_level_get()?;
    println!("Wireless RF Power level: {:?}", &response);

    Ok(())
}
```

## Projects using this crate

[Dygma Layer Switcher](https://github.com/mbwilding/dygma-layer-switcher)
