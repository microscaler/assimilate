//! Assimilate Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `--version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod start;
mod chart;

use self::start::StartCmd;
use self::chart::ChartCmd;
use crate::config::AssimilateConfig;
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Runnable};
use once_cell::sync::Lazy;
use std::path::PathBuf;

/// Application configuration filename
/// Generate os agnostic path to config file from home directory
static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| {
    dirs::home_dir()
        .unwrap()
        .join(".config")
        .join("assimilate")
        .join("config.toml")
});

/// Assimilate Subcommands
/// Subcommands need to be listed in an enum.
#[derive(clap::Parser, Command, Debug, Runnable)]
pub enum AssimilateCmd {
    /// The `start` subcommand
    Start(StartCmd),
    /// The `chart` subcommand
    Chart(ChartCmd),
}

/// Entry point for the application. It needs to be a struct to allow using subcommands!
#[derive(clap::Parser, Command, Debug)]
#[command(author, about, version)]
pub struct EntryPoint {
    #[command(subcommand)]
    cmd: AssimilateCmd,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[arg(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<AssimilateConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = self
            .config
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| CONFIG_FILE.clone());

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(&self, config: AssimilateConfig) -> Result<AssimilateConfig, FrameworkError> {
        match &self.cmd {
            AssimilateCmd::Start(cmd) => cmd.override_config(config),
            //
            // If you don't need special overrides for some
            // subcommands, you can just use a catch all
            // _ => Ok(config),
            AssimilateCmd::Chart(cmd) => cmd.override_config(config),
            //
            // If you don't need special overrides for some
            // subcommands, you can just use a catch all
            // _ => Ok(config),
        }
    }
}
