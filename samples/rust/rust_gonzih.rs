//! Gonzih module

use kernel::prelude::*;

module! {
    type: Gonzih,
    name: "gonzih",
    author: "Max Gonzih",
    description: "gonzih",
    license: "GPL",
}

struct Gonzih {}

impl kernel::Module for Gonzih {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("====================================");
        pr_info!("Rust gonzih sample (init)\n");
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));
        pr_info!("====================================");
        Ok(Gonzih {})
    }
}
