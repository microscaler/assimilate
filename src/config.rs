//! Assimilate Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.
//!
//! Config is based on TOML
//! see spec at https://toml.io/en/
//!


use serde::{Deserialize, Serialize};

/// Assimilate Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssimilateConfig {
    /// An example configuration section
    pub hello: ExampleSection,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on AssimilateConfig instead.
impl Default for AssimilateConfig {
    fn default() -> Self {
        Self {
            hello: ExampleSection::default(),
        }
    }
}

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExampleSection {
    /// Example configuration value
    pub recipient: String,
}

impl Default for ExampleSection {
    fn default() -> Self {
        Self {
            recipient: "world".to_owned(),
        }
    }
}
