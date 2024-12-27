// SPDX-License-Identifier: GPL-2.0

//! Rust hello_world

use kernel::prelude::*;

module! {
    type: RustHello,
    name: "rust_hello",
    author: "Edson Juliano Drosdeck",
    description: "Rust hello world",
    license: "GPL",
}

struct RustHello;

impl kernel::Module for RustHello {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World RUST\n");
        Ok(RustHello)
    }
}

impl Drop for RustHello {
    fn drop(&mut self) {
        pr_info!("Rust Hello world (exit)\n");
    }
}



