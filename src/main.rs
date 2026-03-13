mod cli;
mod reducer;

use std::fs::File;
use std::io;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;

use cli::Args;
use reducer::fold;

fn main() -> Result<()> {
    let args = Args::parse();

    let ignore_prefix = args
        .ignore_prefix
        .as_deref()
        .map(|pattern| Regex::new(pattern).context("Invalid --ignore-prefix regex"))
        .transpose()?;

    let results = match &args.file {
        Some(path) => {
            let file = File::open(path).with_context(|| format!("Failed to open '{path}'"))?;
            fold(file, ignore_prefix.as_ref())?
        }
        None => fold(io::stdin(), ignore_prefix.as_ref())?,
    };

    let results = match args.top {
        Some(n) => &results[..n.min(results.len())],
        None => &results[..],
    };

    for (_key, entry) in results {
        println!("[{}x] {}", entry.count, entry.first_seen);
    }

    Ok(())
}
