# tztail

[![Build Status](https://travis-ci.org/thecasualcoder/tztail.svg?branch=master)](https://travis-ci.org/thecasualcoder/tztail)
[![crates.io](https://img.shields.io/crates/v/tztail.svg)](https://crates.io/crates/tztail)

tztail (TimeZoneTAIL) allows you to view logs in the timezone you want. Its tail with timezone.

## Install

_Using Homebrew_

```bash
brew tap thecasualcoder/stable
brew install tztail
```

_Using Cargo_

```bash
cargo install tztail
```

## Usage

```bash
$ tztail --help
tztail (TimeZoneTAIL) allows you to view logs in the timezone you want

USAGE:
    tztail [FILE]

OPTIONS:
    -t, --timezone <TIMEZONE>    Sets the timezone in which output should be printed. (Default: local timezone)
    -f, --follow                 Follow the file indefinitely as changes are added. (Default: Off)
        --format <FORMAT>        Custom format for parsing dates. (Default: autodetected patterns)
    -h, --help                   Prints help information
    -V, --version                Prints version information

ARGS:
    <FILE>    File to tail. STDIN by default
```

## Features

- Supports few standard formats with which auto detection is done when parsing logs.
- Supports specifying custom format for parsing in case it is a non-standard format. See [here](https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html#specifiers) for formats.
- Autodetect source timezone if present in logs. Example (`2014-11-28T12:00:09+0100` is CET)
- Output logs to local timezone by default

## Demo

![demo](/demo/tztail.gif)

## Autodetectable formats

Most used autodetectable formats

| Name             | Example                         |
| ---------------- | ------------------------------- |
| RFC2822          | Fri, 28 Nov 2014 12:00:09 +0000 |
| RFC3339          | 2014-11-28T12:00:09+0000        |
| Nginx Log format | 04/Nov/2018:12:13:49 +0000      |

## Usecase

This tool can be used to convert timestamps in a log to any desired timezone while tailing logs. Eg. In case your logs are in UTC and you want to view it in a different timezone say. Asia/Kolkata (IST), pipe the logs through `tztail`.

```bash
## Example usage
$ cat somelog # A log in UTC
2018-11-03 19:47:20.279044 I mvcc: finished scheduled compaction at 104794 (took 748.443µs)
2018-11-03 19:52:20.282913 I mvcc: store.index: compact 105127

$ cat somelog | tztail --timezone Asia/Kolkata # Timestamps converted to IST
2018-11-04 01:17:20.279044 I mvcc: finished scheduled compaction at 104794 (took 748.443µs)
2018-11-04 01:22:20.282913 I mvcc: store.index: compact 105127
```

It allows to specify a custom format as well.

```bash
## Example usage
$ cat somelog # A log in non-standard format
2018-11-03 20:07:20 mvcc: store.index: compact 106120
2018-11-03 20:07:20 mvcc: finished scheduled compaction at 106120 (took 933.25µs)

$ cat somelog | tztail -t Asia/Kolkata --format "%Y-%m-%d %H:%M:%S"
2018-11-04 01:37:20 mvcc: store.index: compact 106120
2018-11-04 01:37:20 mvcc: finished scheduled compaction at 106120 (took 933.25µs)
```

## Building from source

Checkout the code and build locally. Needs rust compiler 1.30 or above.

```bash
$ git clone https://github.com/thecasualcoder/tztail
$ cd tztail

# To build binary locally
$ cargo build --release

# To install binary locally in Cargo bin path
$ cargo install

# To run tests
$ cargo test
```
