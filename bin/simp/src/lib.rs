pub mod args;
pub mod cli;
pub mod runner;
pub mod server;
pub mod dirs;
pub mod utils;
pub mod prometheus_exporter;


/// Re-export of `simp_rpc_*` crates.
pub mod rpc {
    pub mod builder {
        pub use simp_rpc_builder::*;
    }
}

#[cfg(all(feature = "jemalloc", unix))]
use jemallocator as _;

// for rendering diagrams
use aquamarine as _;