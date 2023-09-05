use std::env;

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

/// Provide fzf interface for mcfly
#[derive(Parser)]
#[command(author, version)]
pub struct Cli {
    /// Shell history file format
    #[arg(value_name = "HISTORY_FORMAT", value_enum, long, default_value_t)]
    pub history_format: HistoryFormat,

    /// Session ID to record or search under (defaults to $MCFLY_SESSION_ID)
    #[arg(long = "session_id")]
    pub session_id: Option<String>,

    #[command(subcommand)]
    pub subcommand: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Print the shell configuration code
    Init {
        /// Shell to init
        #[arg(value_enum)]
        shell: InitMode,
    },
    /// Print the contents of the database as fzf input
    Dump {
        /// Maximum entries to query (default $MCFLY_HISTORY_LIMIT)
        #[arg(short, long)]
        limit: Option<i64>,
        /// Output entries separated by null-bytes rather than newlines
        #[arg(value_name = "NULL", long = "null", short = '0')]
        zero_separated: bool,
        /// Write header line before output
        #[arg(long)]
        header: bool,
        /// Current directory where search is happening (default $PWD)
        #[arg(short, long)]
        dir: Option<String>,
        #[arg(short, long)]
        /// JSON file to read/write view options from (e.g. set by toggle)
        options_json: Option<String>,
    },
    /// Mark a command as selected from the fzf interface
    Select {
        /// The command that was run
        command: Vec<String>,

        /// Directory where command was run (default $PWD)
        #[arg(short, long)]
        dir: Option<String>,
    },
    /// Toggle/create view options in a json file
    Toggle {
        /// Path of the file holding toggle state
        path: String,
        /// Setting to toggle
        toggle: ToggleChoice,
    },
    /// Delete a command
    Delete {
        /// The command string to be deleted
        command: String,
    }
}

#[derive(Clone, Copy, ValueEnum, Default)]
pub enum HistoryFormat {
    #[default]
    Bash,
    Zsh,
    ZshExtended,
    Fish,
}

impl From<HistoryFormat> for mcfly::settings::HistoryFormat {
    fn from(format: HistoryFormat) -> Self {
        match format {
            HistoryFormat::Bash => mcfly::settings::HistoryFormat::Bash,
            HistoryFormat::Zsh => mcfly::settings::HistoryFormat::Zsh {
                extended_history: false,
            },
            HistoryFormat::ZshExtended => mcfly::settings::HistoryFormat::Zsh {
                extended_history: true,
            },
            HistoryFormat::Fish => mcfly::settings::HistoryFormat::Fish,
        }
    }
}
#[derive(Clone, Copy, ValueEnum, Default)]
pub enum ToggleChoice {
    #[default]
    SortOrder,
    CurrentDirOnly,
    ExitCode,
}

#[derive(Clone, Copy, ValueEnum, Serialize, Deserialize)]
pub enum ResultSort {
    Rank,
    LastRun,
}

impl Default for ResultSort {
    fn default() -> Self {
        env::var("MCFLY_RESULTS_SORT")
            .map(|val| match val.as_str() {
                "RANK" => ResultSort::Rank,
                "LAST_RUN" => ResultSort::LastRun,
                _ => ResultSort::Rank,
            })
            .unwrap_or(ResultSort::Rank)
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum InitMode {
    Bash,
    Zsh,
    Fish,
}

#[derive(Clone, Serialize, Deserialize, Default, ValueEnum)]
pub(crate) enum ExitCode {
    Success,
    Fail,
    #[default]
    Any,
}
#[derive(Args, Clone, Serialize, Deserialize, Default)]
pub struct DumpOptions {
    /// Sort ordering of results
    #[arg(value_name = "SORT", value_enum, long = "sort")]
    pub(crate) sort_order: ResultSort,
    /// Only show commands run from current directory
    pub(crate) current_dir_only: bool,

    #[arg(value_name = "EXIT_CODE", value_enum, long = "exit", default_value_t)]
    pub(crate) exit_code: ExitCode,
}

impl Cli {
    pub fn parse_args() -> Cli {
        let mut cli = Cli::parse();

        cli.subcommand = match cli.subcommand {
            Command::Init { shell } => Command::Init { shell },
            Command::Dump {
                limit,
                zero_separated,
                header,
                dir,
                options_json,
            } => Command::Dump {
                limit: limit.or_else(|| {
                    env::var("MCFLY_HISTORY_LIMIT")
                        .ok()
                        .and_then(|o| o.parse::<i64>().ok())
                }),
                zero_separated,
                header,
                dir: dir.or_else(|| env::var("PWD").ok()),
                options_json,
            },
            Command::Select { command, dir } => Command::Select {
                command,
                dir: dir.or_else(|| env::var("PWD").ok()),
            },
            Command::Toggle { path, toggle } => Command::Toggle { path, toggle },
            Command::Delete { command } => Command::Delete { command },
        };
        cli.session_id = cli.session_id.or_else(|| env::var("MCFLY_SESSION_ID").ok());
        cli
    }
}
