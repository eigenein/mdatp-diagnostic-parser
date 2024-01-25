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
