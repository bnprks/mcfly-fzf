use chrono::{Duration, TimeZone, Utc};
use rusqlite::named_params;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use crate::cli::{DumpOptions, ExitCode, InitMode, ResultSort, ToggleChoice};
use mcfly::history::History;

pub fn init(mode: InitMode) {
    let text = match mode {
        InitMode::Bash => include_str!("../shell/mcfly-fzf.bash"),
        InitMode::Zsh => include_str!("../shell/mcfly-fzf.zsh"),
        InitMode::Fish => include_str!("../shell/mcfly-fzf.fish"),
    };
    print!("{}", text)
}

pub fn dump(
    history: &History,
    session_id: Option<String>,
    limit: Option<i64>,
    zero_separated: bool,
    header: bool,
    current_dir: &String,
    option_json: Option<String>,
) {
    let options = if let Some(path) = option_json {
        read_dump_options(&path)
    } else {
        DumpOptions::default()
    };

    // Only build cache table if we're using neural network ranking
    if let ResultSort::Rank = options.sort_order {
        history.build_cache_table(current_dir, &mcfly::settings::ResultFilter::Global, &session_id, None, None, None, limit);
    }

    let order_clause: &str = match options.sort_order {
        ResultSort::LastRun => "last_run DESC",
        _ => "contextual_commands.rank DESC",
    };

    let dir_clause: &str = match options.current_dir_only {
        true => "commands.dir = :current_dir",
        false => ":current_dir = :current_dir",
    };

    let status_clause: &str = match options.exit_code {
        ExitCode::Any => "true",
        ExitCode::Fail => "commands.exit_code != 0",
        ExitCode::Success => "commands.exit_code == 0",
    };

    // Math for limiting search history copied from history::build_cache_table
    let max_id: i64 = history
        .connection
        .query_row("SELECT IFNULL(MAX(id), 0) FROM commands", [], |row| {
            row.get(0)
        })
        .unwrap_or(0);

    let min_id = if let Some(limit_value) = limit {
        if limit_value > max_id {
            0
        } else {
            (max_id as f64 * (1.0 - (limit_value as f64 / max_id as f64))) as i64
        }
    } else {
        0
    };

    let query: String = match options.sort_order {
        ResultSort::Rank => format!(
            "SELECT contextual_commands.cmd, MAX(commands.when_run) as last_run
            FROM contextual_commands
                INNER JOIN commands 
                ON contextual_commands.cmd = commands.cmd
            WHERE commands.id > :min_id AND {} AND {}
            GROUP BY contextual_commands.cmd
            ORDER BY {};",
            dir_clause, status_clause, order_clause,
        ),
        ResultSort::LastRun => format!(
            "SELECT commands.cmd, MAX(when_run) AS last_run
            FROM commands 
            WHERE commands.id > :min_id AND {} AND {}
            GROUP BY commands.cmd
            ORDER BY {};",
            dir_clause, status_clause, order_clause,
        ),
    };

    let mut statement = history
        .connection
        .prepare(&query)
        .unwrap_or_else(|err| panic!("Mcfly-fzf error: Prepare to work ({})", err));
    let mut rows = statement
        .query(named_params! {":current_dir": current_dir, ":min_id": min_id})
        .unwrap_or_else(|err| panic!("Mcfly-fzf error: Query Map to work ({})", err));

    let mut stdout = std::io::stdout();

    if header {
        stdout
            .write_fmt(format_args!(
                "F1 Sort ({}) | F2 Dir ({}) | F3 Status {} | Ctrl-R Strict Ordering {}",
                match options.sort_order {
                    ResultSort::LastRun => "Time",
                    ResultSort::Rank => "Rank",
                },
                match options.current_dir_only {
                    true => "Cur",
                    false => "Any",
                },
                match options.exit_code {
                    ExitCode::Any => "(Any) ",
                    ExitCode::Fail => "(Fail)",
                    ExitCode::Success => "(Okay)",
                },
                if options.strict_ordering {"(On) "} else {"(Off)"}
            ))
            .and_then(|_| stdout.write(if zero_separated { b"\0" } else { b"\n" }))
            .unwrap();
    }

    while let Ok(Some(row)) = rows.next() {
        let cmd: String = row.get(0).unwrap_or_else(|err| {
            panic!(
                "Mcfly-fzf error: unable to read database result column 'cmd': {}",
                err
            )
        });
        let last_run: i64 = row.get(1).unwrap_or_else(|err| {
            panic!(
                "Mcfly-fzf error: unable to read database result column 'last_run': {}",
                err
            )
        });

        let duration = format_time_since(last_run);

        let res = stdout
            .write_fmt(format_args!("{}\t{}", duration, cmd))
            .and_then(|_| stdout.write(if zero_separated { b"\0" } else { b"\n" }));
        if res.is_err() {
            break;
        }
    }
}

pub fn toggle(path: String, toggle: ToggleChoice) {
    let mut opts = read_dump_options(&path);

    // Toggle options
    match toggle {
        ToggleChoice::SortOrder => {
            opts.sort_order = match opts.sort_order {
                ResultSort::LastRun => ResultSort::Rank,
                ResultSort::Rank => ResultSort::LastRun,
            }
        }
        ToggleChoice::CurrentDirOnly => opts.current_dir_only = !opts.current_dir_only,
        ToggleChoice::ExitCode => {
            opts.exit_code = match opts.exit_code {
                ExitCode::Success => ExitCode::Fail,
                ExitCode::Fail => ExitCode::Any,
                ExitCode::Any => ExitCode::Success,
            }
        }
        ToggleChoice::StrictOrdering => opts.strict_ordering = !opts.strict_ordering,
    }

    write_dump_options(&path, &opts)
}

pub fn print_toggle(path: String, toggle: ToggleChoice) {
    let opts = read_dump_options(&path);
    match toggle {
        ToggleChoice::StrictOrdering => {
            if opts.strict_ordering {
                println!("+s");
            } else {
                println!("");
            }
        }
        _ => {
            eprintln!("Only viewing of strict-ordering toggle is supported.")
        }
    }
}

/// Read dump option from a json file, or default options in case of a non-existent file
fn read_dump_options(path: &String) -> DumpOptions {
    if Path::new(&path).exists() {
        // Read input
        let input = File::open(path)
            .unwrap_or_else(|_| panic!("Mcfly-fzf error: unable to open toggle file: {}", &path));

        if input.metadata().map(|m| m.len() == 0).unwrap_or(false) {
            DumpOptions::default()
        } else {
            serde_json::from_reader(BufReader::new(input)).unwrap_or_else(|err| {
                panic!("Mcfly-fzf error: Could not parse toggle file: {}", err)
            })
        }
    } else {
        DumpOptions::default()
    }
}

/// Write dump options to a json file
fn write_dump_options(path: &String, opts: &DumpOptions) {
    let output = File::create(path)
        .unwrap_or_else(|err| panic!("Mcfly-fzf error: Could not write to toggle file: {}", err));
    serde_json::to_writer_pretty(BufWriter::new(&output), &opts)
        .unwrap_or_else(|err| panic!("Mcfly-fzf error: Could not write toggle file: {}", err));
}

fn format_time_since(secs: i64) -> String {
    humantime::format_duration(
        Duration::minutes(
            Utc::now()
                .signed_duration_since(Utc.timestamp_opt(secs, 0).unwrap())
                .num_minutes(),
        )
        .to_std()
        .unwrap(),
    )
    .to_string()
    .split(' ')
    .take(2)
    .map(|s| {
        s.replace("years", "y")
            .replace("year", "y")
            .replace("months", "mo")
            .replace("month", "mo")
            .replace("days", "d")
            .replace("day", "d")
            .replace("hours", "h")
            .replace("hour", "h")
            .replace("minutes", "m")
            .replace("minute", "m")
            .replace("0s", "< 1m")
    })
    .collect::<Vec<String>>()
    .join(" ")
}
