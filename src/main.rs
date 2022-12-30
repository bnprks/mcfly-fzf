use mcfly::history::History;
use mcfly_fzf::cli::{Cli, Command};

fn main() {
    let settings = Cli::parse_args();

    match settings.subcommand {
        Command::Init { shell } => mcfly_fzf::fzf::init(shell),
        Command::Dump {
            limit,
            zero_separated,
            header,
            dir,
            options_json,
        } => {
            let history = History::load(settings.history_format.into());
            let dir = dir.unwrap_or_else(|| panic!("Mcfly-fzf error: Could not detect dir"));
            history.build_cache_table(
                &dir,
                &Some(
                    settings
                        .session_id
                        .unwrap_or_else(|| panic!("Mcfly-fzf error: Could not detect session_id")),
                ),
                None,
                None,
                None,
                limit,
            );
            mcfly_fzf::fzf::dump(&history, zero_separated, header, &dir, options_json)
        }
        Command::Select { command, dir } => {
            let history = History::load(settings.history_format.into());
            history.record_selected_from_ui(
                &command.join(" "),
                &settings
                    .session_id
                    .unwrap_or_else(|| panic!("Mcfly-fzf error: Could not detect session_id")),
                &dir.unwrap_or_else(|| panic!("Mcfly-fzf error: Could not detect dir")),
            );
        }
        Command::Toggle { path, toggle } => {
            mcfly_fzf::fzf::toggle(path, toggle);
        }
    }
}
