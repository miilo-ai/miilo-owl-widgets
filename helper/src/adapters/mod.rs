pub mod http;
pub mod pairing;
pub mod store;
pub use crate::{config, desktop_install, display_names, local_agents, local_import, setup};

pub mod chat_command;
pub mod chat_config;
pub mod chat_http;
pub mod chat_sse;
pub mod chat_text;
pub mod widget_chat;
pub mod widget_chat_guard;
pub mod widget_speech;
pub mod widget_speech_setup;

pub mod uninstall;

pub mod paired_selection;

pub mod telemetry;

pub mod worker_process;
pub mod workers;
