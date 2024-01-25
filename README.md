# `mdatp-diagnostic-parser`

Revamp of the official [`high_cpu_parser.py`](https://github.com/microsoft/mdatp-xplat/blob/master/linux/diagnostic/high_cpu_parser.py). This project is not affiliated with Microsoft.

## Installation

```shell
cargo install mdatp-diagnostic-parser
```

## Usage

```shell
mdatp diagnostic real-time-protection-statistics --output=json | mdatp-diagnostic-parser -
```

## Options

```text
Usage: mdatp-diagnostic-parser [OPTIONS] <PATH>

Arguments:
  <PATH>
          File to parse or `-` for the standard input

Options:
  -a, --aggregate-by <AGGREGATE_BY>
          Aggregation
          
          [default: name]

          Possible values:
          - none: Do not aggregate
          - name: By process name
          - path: By executable path

  -s, --sort-by <SORT_BY>
          Sorting
          
          [default: total-files-scanned]

          Possible values:
          - none:                Do not sort
          - total-files-scanned: By total files scanned
          - max-file-scan-time:  By maximum file scanning time
          - total-scan-time:     By total scanning time

  -l, --limit <LIMIT>
          Print at maximum this number of rows

      --ascending
          Print in ascending order (by default, print in descending order)

      --only-active
          List only active processes (as reported by MDATP)

      --only-running
          List only running processes

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
