use clap::Parser;
use clap_stdin::FileOrStdin;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// File to parse or `-` for the standard input.
    pub path: FileOrStdin,

    /// Aggregation.
    #[clap(short, long, value_enum, default_value_t)]
    pub aggregate_by: AggregateBy,

    /// Sorting.
    #[clap(short, long, value_enum, default_value_t)]
    pub sort_by: SortBy,

    /// Print at maximum this number of rows.
    #[clap(short, long)]
    pub limit: Option<usize>,

    /// Print in ascending order (by default, print in descending order).
    #[clap(long)]
    pub ascending: bool,

    /// List only active processes (as reported by MDATP).
    #[clap(long)]
    pub only_active: bool,

    /// List only running processes.
    #[clap(long)]
    pub only_running: bool,
}

#[derive(Copy, Clone, Default, clap::ValueEnum)]
pub enum AggregateBy {
    /// Do not aggregate.
    None,

    /// By process name.
    #[default]
    Name,

    /// By executable path.
    Path,
}

#[derive(Copy, Clone, Default, clap::ValueEnum)]
pub enum SortBy {
    /// Do not sort.
    None,

    /// By total files scanned.
    #[default]
    TotalFilesScanned,

    /// By maximum file scanning time.
    MaxFileScanTime,

    /// By total scanning time.
    TotalScanTime,
}
