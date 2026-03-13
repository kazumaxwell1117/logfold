use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

use anyhow::Result;
use regex::Regex;

pub struct LogEntry {
    pub count: usize,
    pub first_seen: String,
}

/// Read lines from `reader`, group by normalized key, and return sorted results.
///
/// Normalization strips the `ignore_prefix` regex match from the start of each line.
/// Results are sorted by count descending, then alphabetically by normalized key.
pub fn fold<R: Read>(
    reader: R,
    ignore_prefix: Option<&Regex>,
) -> Result<Vec<(String, LogEntry)>> {
    let buf = BufReader::new(reader);
    let mut map: HashMap<String, LogEntry> = HashMap::new();

    for line in buf.lines() {
        let line = line?;

        let key = match ignore_prefix {
            Some(re) => re.replacen(&line, 1, "").into_owned(),
            None => line.clone(),
        };

        let entry = map.entry(key).or_insert(LogEntry {
            count: 0,
            first_seen: line,
        });
        entry.count += 1;
    }

    let mut results: Vec<(String, LogEntry)> = map.into_iter().collect();

    // Sort: count descending, then normalized key ascending
    results.sort_unstable_by(|a, b| b.1.count.cmp(&a.1.count).then_with(|| a.0.cmp(&b.0)));

    Ok(results)
}
