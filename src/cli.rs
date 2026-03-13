use clap::Parser;

/// Collapse duplicate log messages in large log files while ignoring variable prefixes.
///
/// Examples:
///   logfold app.log
///   logfold app.log --ignore-prefix "^[0-9-: ]+"
///   logfold app.log --ignore-prefix "^[0-9-: ]+" --top 10
///   cat app.log | logfold
#[derive(Parser, Debug)]
#[command(name = "logfold", version, about, long_about = None)]
pub struct Args {
    /// Input log file (reads from stdin if not provided)
    pub file: Option<String>,

    /// Ignore a matching prefix when grouping lines (only affects grouping, not output)
    #[arg(long, value_name = "REGEX")]
    pub ignore_prefix: Option<String>,

    /// Show only the top N results
    #[arg(long, value_name = "N")]
    pub top: Option<usize>,
}
