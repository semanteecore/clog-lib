#[macro_use]
mod macros;
mod clog;
pub mod error;
pub mod fmt;
pub mod git;
mod link_style;
mod sectionmap;

pub use crate::clog::Clog;
pub use crate::link_style::LinkStyle;
pub use crate::sectionmap::SectionMap;

// The default config file
const CLOG_CONFIG_FILE: &'static str = ".clog.toml";
