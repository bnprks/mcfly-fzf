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

            mcfly_fzf::fzf::dump(
                &history,
                settings.session_id,
                limit,
                zero_separated,
                header,
                &dir,
                options_json,
            )
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
        Command::Toggle { path, toggle, view} => {
            if view {
                mcfly_fzf::fzf::print_toggle(path, toggle);
            } else {
                mcfly_fzf::fzf::toggle(path, toggle);
            }
        }
    }
}
