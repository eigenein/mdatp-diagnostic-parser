mod cli;
mod statistics;

use std::{cmp::Ordering, ops::AddAssign};

use anyhow::{Context, Result};
use clap::Parser;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use itertools::Itertools;
use psutil::process::Process;

use crate::{
    cli::{AggregateBy, Cli, SortBy},
    statistics::{Counter, Statistics},
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let contents = cli.path.contents().context("failed to read the input file contents")?;
    let statistics: Statistics =
        serde_json::from_str(&contents).context("failed to parse the input file")?;

    let mut rows: Vec<AggregatedRow> = statistics
        .counters
        .into_iter()
        .filter(|counter| !cli.only_active || counter.is_active)
        .filter_map(|mut counter| {
            if counter.name.is_none() {
                // The diagnostic tool failed to report the process name (how ironic). Try to fix it for them.
                counter.name =
                    Process::new(counter.process_id).ok().and_then(|process| process.name().ok());
            }
            ((!cli.only_running)
                || Process::new(counter.process_id)
                    .ok()
                    .map(|process| process.is_running())
                    .unwrap_or(true))
            .then_some(counter)
        })
        .into_grouping_map_by(group_function(cli.aggregate_by))
        .aggregate(|acc, _key, counter| {
            let row = AggregatedRow::from(counter);
            match acc {
                Some(mut acc) => {
                    acc += row;
                    Some(acc)
                }
                None => Some(row),
            }
        })
        .into_values()
        .collect();

    rows.sort_by(sort_function(cli.sort_by));

    if cli.ascending {
        rows.reverse();
    }

    if let Some(limit) = cli.limit {
        rows.truncate(limit);
    }

    print_table(rows);
    Ok(())
}

fn group_function(aggregate_by: AggregateBy) -> impl Fn(&Counter) -> AggregationKey {
    match aggregate_by {
        AggregateBy::None => |_: &Counter| AggregationKey::None,
        AggregateBy::Name => |counter: &Counter| AggregationKey::Name(counter.name.clone()),
        AggregateBy::Path => |counter: &Counter| AggregationKey::Path(counter.path.clone()),
    }
}

fn sort_function(sort_by: SortBy) -> impl Fn(&AggregatedRow, &AggregatedRow) -> Ordering {
    match sort_by {
        SortBy::None => |_lhs: &AggregatedRow, _rhs: &AggregatedRow| Ordering::Equal,
        SortBy::MaxFileScanTime => |lhs: &AggregatedRow, rhs: &AggregatedRow| {
            rhs.max_file_scan_time.cmp(&lhs.max_file_scan_time)
        },
        SortBy::TotalFilesScanned => |lhs: &AggregatedRow, rhs: &AggregatedRow| {
            rhs.n_total_files_scanned.cmp(&lhs.n_total_files_scanned)
        },
        SortBy::TotalScanTime => {
            |lhs: &AggregatedRow, rhs: &AggregatedRow| rhs.total_scan_time.cmp(&lhs.total_scan_time)
        }
    }
}

fn print_table(rows: impl IntoIterator<Item = AggregatedRow>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "PIDs",
            "Names",
            "Paths",
            "Files scanned",
            "Total scan time",
            "Maximum file scan time",
        ]);
    for row in rows {
        let process_ids = format!("{}", row.process_ids.into_iter().format(", "));
        let names = format!("{}", row.names.into_iter().format(", "));
        let paths = format!("{}", row.paths.into_iter().format("\n"));
        table.add_row(vec![
            process_ids,
            names,
            paths,
            row.n_total_files_scanned.to_string(),
            row.total_scan_time.to_string(),
            row.max_file_scan_time.to_string(),
        ]);
    }
    println!("{table}");
}

#[derive(Hash, Eq, PartialEq)]
enum AggregationKey {
    None,
    Name(Option<String>),
    Path(Option<String>),
}

struct AggregatedRow {
    process_ids: Vec<u32>,
    names: Vec<String>,
    paths: Vec<String>,
    max_file_scan_time: u64,
    total_scan_time: u64,
    n_total_files_scanned: u64,
}

impl From<Counter> for AggregatedRow {
    fn from(counter: Counter) -> Self {
        Self {
            process_ids: vec![counter.process_id],
            names: counter.name.into_iter().collect(),
            paths: counter.path.into_iter().collect(),
            max_file_scan_time: counter.max_file_scan_time,
            total_scan_time: counter.total_scan_time,
            n_total_files_scanned: counter.n_total_files_scanned,
        }
    }
}

impl AddAssign<Self> for AggregatedRow {
    fn add_assign(&mut self, rhs: Self) {
        self.process_ids.extend(rhs.process_ids);
        self.paths.extend(rhs.paths);
        self.max_file_scan_time = rhs.max_file_scan_time.max(self.max_file_scan_time);
        self.total_scan_time += rhs.total_scan_time;
        self.n_total_files_scanned += rhs.n_total_files_scanned;
    }
}
