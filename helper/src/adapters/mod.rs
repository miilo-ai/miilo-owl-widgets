pub mod http;
pub mod pairing;
pub mod store;
pub use crate::{config, desktop_install, display_names, local_agents, local_import, setup};

pub mod chat_command;
pub mod chat_config;
pub mod chat_http;
pub mod chat_sse;

pub mod uninstall;

pub mod paired_selection;
