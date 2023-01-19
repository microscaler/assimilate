//! `chart` subcommand - example of how to write a subcommand

use crate::config::AssimilateConfig;
/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use gumdrop::Options;

/// `chart` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
/// assimilate chart --repo https://somerepo.com \
///                  --chart mysql \
///                  --version 0.1.0 \
///                  --type app \
///                  --name mysql \
///                  --namespace my-namespace \ # optional
///                  --dry-run \
///
#[derive(clap::Parser, Command, Debug, Options)]
pub struct ChartCmd {
    /// To whom are we saying hello?
    // recipient: Vec<String>,

    #[clap(long = "repo", help = "Helm repo http url")]
    pub repo: Option<String>,

    #[clap(
        long = "chart",
        required = true,
        help = "Chart name, including recommended but optional provider e.g. bitnami/redis"
    )]
    pub chart: Option<String>,

    #[clap(
        long = "version",
        required = false,
        help = "Version of chart to onboard. This will default to the latest available for the chart if version is not specified."
    )]
    pub version: Option<String>,

    #[clap(
        long = "type",
        required = true,
        help = "App types to onboard, e.g. applications, eks-addons."
    )]
    pub component: Option<String>,

    #[clap(
        long = "name",
        required = true,
        help = "friendly name for chart, often compared to the helm release name.

            This name will also be used for the namespace unless specified otherwise.

            This name is also often the name of the helm chart without the repo name."
    )]
    pub name: Option<String>,

    #[clap(
    long = "namespace",
    required = false,
    help = "Overide the namespace which is usually set to <chart name>, ."
    )]
    pub namespace: Option<String>,

    #[clap(
        long = "dry-run",
        help = "Dry run pulls all charts but does not push to registry."
    )]
    pub dryrun: Option<String>,
}

impl Runnable for ChartCmd {
    /// Run the application.
    fn run(&self) {
        let _config = APP.config();
        //println!("Hello, {}!", &config.hello.recipient);
        println!(
            "Args passed: {:?} {:?} {:?} {:?} {:?}",
            &self.repo, &self.chart, &self.version, &self.component, &self.name
        );
    }
}

impl config::Override<AssimilateConfig> for ChartCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        // mut config: AssimilateConfig,
        config: AssimilateConfig,
    ) -> Result<AssimilateConfig, FrameworkError> {
        // if !self.recipient.is_empty() {
        //     config.hello.recipient = self.recipient.join(" ");
        // }

        Ok(config)
    }
}
