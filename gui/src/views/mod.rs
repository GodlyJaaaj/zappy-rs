// gui/src/views/mod.rs
mod logs;
mod map;
mod settings;

pub use logs::{LogsMessage, LogsView};
pub use map::{MapMessage, MapView};
pub use settings::{SettingsMessage, SettingsView};
