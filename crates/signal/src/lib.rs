pub mod handler;
pub mod module;
pub mod platform;

pub use handler::{SignalHandle, SignalName, with_handle};
pub use module::SignalModule;
