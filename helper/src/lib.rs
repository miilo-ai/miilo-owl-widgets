//! Provider-neutral application logic with explicit infrastructure adapters.
pub mod action_policy;
pub mod adapters;
pub mod application;
pub mod artifacts;
#[path = "adapters/config.rs"]
pub mod config;
#[path = "adapters/desktop_install.rs"]
pub mod desktop_install;
#[path = "adapters/display_names.rs"]
pub mod display_names;
pub mod domain;
pub mod graph;
#[path = "adapters/local_agents.rs"]
pub mod local_agents;
#[path = "adapters/local_import.rs"]
pub mod local_import;
#[path = "adapters/local_snapshot.rs"]
mod local_snapshot;
pub mod operations;
pub mod projection;
#[path = "adapters/setup.rs"]
pub mod setup;
pub mod snapshot;
pub mod transport;

pub type Result<T> = std::result::Result<T, String>;

pub use domain::identity::key;
