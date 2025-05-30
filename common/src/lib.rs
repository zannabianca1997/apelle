mod cli;
pub mod error_reporter;
mod logging;
pub mod main_wrapper;

pub use figment::value::magic::RelativePathBuf;
pub use main_wrapper::service_main;
