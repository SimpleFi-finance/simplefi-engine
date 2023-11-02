use clap::{value_parser, ArgAction, Args, Parser, Subcommand, ValueEnum};
use crate::{runner::CliRunner, server::ServerCommand, dirs::{PlatformPath, LogsDir}};
use simp_tracing::{
    tracing::{metadata::LevelFilter, Level, Subscriber},
    tracing_subscriber::{filter::Directive, registry::LookupSpan, EnvFilter},
    BoxedLayer, FileWorkerGuard,
};
use std::{fmt, fmt::Display};

pub mod config;
pub mod components;

/// The main simp cli interface.
///
/// This is the entrypoint to the executable.
#[derive(Debug, Parser)]
#[command(about = "Simp", long_about = None)]
pub struct Cli {
    /// The subcommand to run.
    #[clap(subcommand)]
    command: Commands,

    /// Add a new instance of a node.
    ///
    /// Configures the ports of the node to avoid conflicts with the defaults.
    /// This is useful for running multiple nodes on the same machine.
    ///
    /// Max number of instances is 200. It is chosen in a way so that it's not possible to have
    /// port numbers that conflict with each other.
    ///
    /// Changes to the following port numbers:
    /// - DISCOVERY_PORT: default + `instance` - 1
    /// - AUTH_PORT: default + `instance` * 100 - 100
    /// - HTTP_RPC_PORT: default - `instance` + 1
    /// - WS_RPC_PORT: default + `instance` * 2 - 2
    #[arg(long, value_name = "INSTANCE", global = true, default_value_t = 1, value_parser = value_parser!(u16).range(..=200))]
    instance: u16,

    #[clap(flatten)]
    logs: Logs,

    #[clap(flatten)]
    verbosity: Verbosity,
}

impl Cli {
    pub fn run(self) -> eyre::Result<()> {
        /* // add network name to logs dir
        self.logs.log_directory = self.logs.log_directory.join(self.chain.chain.to_string()); */

        let _guard = self.init_tracing()?;

        let runner = CliRunner;

        match self.command {
            Commands::Server(command) => runner.run_command_until_exit(|ctx| command.execute(ctx)),
       }
    }

    /// Initializes tracing with the configured options.
    ///
    /// If file logging is enabled, this function returns a guard that must be kept alive to ensure
    /// that all logs are flushed to disk.
    pub fn init_tracing(&self) -> eyre::Result<Option<FileWorkerGuard>> {
        let mut layers =
            vec![simp_tracing::stdout(self.verbosity.directive(), &self.logs.color.to_string())];
        let guard = self.logs.layer()?.map(|(layer, guard)| {
            layers.push(layer);
            guard
        });

        simp_tracing::init(layers);

        Ok(guard.flatten())
    }

}

/// Convenience function for parsing CLI options, set up logging and run the chosen command.
#[inline]
pub fn run() -> eyre::Result<()> {
    Cli::parse().run()
}


/// Commands to be executed
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start the node
    #[command(name = "server")]
    Server(ServerCommand),

}

/// The log configuration.
#[derive(Debug, Args)]
#[command(next_help_heading = "Logging")]
pub struct Logs {
    /// The path to put log files in.
    #[arg(
        long = "log.directory",
        value_name = "PATH",
        global = true,
        default_value_t,
        conflicts_with = "journald"
    )]
    log_directory: PlatformPath<LogsDir>,

    /// The maximum size (in MB) of log files.
    #[arg(long = "log.max-size", value_name = "SIZE", global = true, default_value_t = 200)]
    log_max_size: u64,

    /// The maximum amount of log files that will be stored. If set to 0, background file logging
    /// is disabled.
    #[arg(long = "log.max-files", value_name = "COUNT", global = true, default_value_t = 5)]
    log_max_files: usize,

    /// Log events to journald.
    #[arg(long = "log.journald", global = true, conflicts_with = "log_directory")]
    journald: bool,

    /// The filter to use for logs written to the log file.
    #[arg(long = "log.filter", value_name = "FILTER", global = true, default_value = "error")]
    filter: String,

    /// Sets whether or not the formatter emits ANSI terminal escape codes for colors and other
    /// text formatting.
    #[arg(
        long,
        value_name = "COLOR",
        global = true,
        default_value_t = ColorMode::Always
    )]
    color: ColorMode,
}


/// Constant to convert megabytes to bytes
const MB_TO_BYTES: u64 = 1024 * 1024;

impl Logs {
    /// Builds a tracing layer from the current log options.
    pub fn layer<S>(&self) -> eyre::Result<Option<(BoxedLayer<S>, Option<FileWorkerGuard>)>>
    where
        S: Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        let filter = EnvFilter::builder().parse(&self.filter)?;

        if self.journald {
            Ok(Some((simp_tracing::journald(filter).expect("Could not connect to journald"), None)))
        } else if self.log_max_files > 0 {
            let (layer, guard) = simp_tracing::file(
                filter,
                &self.log_directory,
                "simp.log",
                self.log_max_size * MB_TO_BYTES,
                self.log_max_files,
            );
            Ok(Some((layer, Some(guard))))
        } else {
            Ok(None)
        }
    }
}


/// The verbosity settings for the cli.
#[derive(Debug, Copy, Clone, Args)]
#[command(next_help_heading = "Display")]
pub struct Verbosity {
    /// Set the minimum log level.
    ///
    /// -v      Errors
    /// -vv     Warnings
    /// -vvv    Info
    /// -vvvv   Debug
    /// -vvvvv  Traces (warning: very verbose!)
    #[clap(short, long, action = ArgAction::Count, global = true, default_value_t = 3, verbatim_doc_comment, help_heading = "Display")]
    verbosity: u8,

    /// Silence all log output.
    #[clap(long, alias = "silent", short = 'q', global = true, help_heading = "Display")]
    quiet: bool,
}

impl Verbosity {
    /// Get the corresponding [Directive] for the given verbosity, or none if the verbosity
    /// corresponds to silent.
    pub fn directive(&self) -> Directive {
        if self.quiet {
            LevelFilter::OFF.into()
        } else {
            let level = match self.verbosity - 1 {
                0 => Level::ERROR,
                1 => Level::WARN,
                2 => Level::INFO,
                3 => Level::DEBUG,
                _ => Level::TRACE,
            };

            format!("{level}").parse().unwrap()
        }
    }
}

/// The color mode for the cli.
#[derive(Debug, Copy, Clone, ValueEnum, Eq, PartialEq)]
pub enum ColorMode {
    /// Colors on
    Always,
    /// Colors on
    Auto,
    /// Colors off
    Never,
}

impl Display for ColorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorMode::Always => write!(f, "always"),
            ColorMode::Auto => write!(f, "auto"),
            ColorMode::Never => write!(f, "never"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli() {
        let args = Cli::parse_from(&["simp", "server"]);
        assert!(matches!(args.command, Commands::Server(_)));
    }

    #[test]
    fn parse_color_mode() {
        let simp = Cli::try_parse_from(["simp", "server", "--color", "always"]).unwrap();
        assert_eq!(simp.logs.color, ColorMode::Always);
    }

    /// Tests that the log directory is parsed correctly. It's always tied to the specific chain's
    /// name
    #[test]
    fn parse_logs_path() {
        let simp = Cli::try_parse_from(["simp", "server"]).unwrap();
        
        println!("{}", simp.logs.log_directory);

        let log_dir = simp.logs.log_directory;

        // TODO: missing chain/mode

        assert!(log_dir.as_ref().ends_with("simp/logs"), "{:?}", log_dir);
    }
}
