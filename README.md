# tztail

tztail (TimeZoneTAIL) allows you to view logs in the timezone you want.

> This project is work in progress

## Usecase

This tool can be used to convert timestamps in a log to any desired timezone while tailing logs. Eg. In case your logs are in UTC and you want to view it in a different timezone say. Asia/Kolkata (IST), pipe the logs through `tztail`.

```bash
## Example usage
$ cat somelog # A log in UTC
2014-11-28T12:00:09+00:00 I | some log
$ tail somelog | tztail --timezone Asia/Kolkata
2014-11-28T17:30:09+05:30 I | some log # timestamps alone converted to IST
```

## Usage

```bash
tztail 0.1.0
tztail (TimeZoneTAIL) allows you to view logs in the timezone you want

USAGE:
    tztail --timezone <TIMEZONE>

OPTIONS:
    -t, --timezone <TIMEZONE>    Sets the timezone in which output should be printed
    -h, --help                   Prints help information
    -V, --version                Prints version information
```

It reads from _STDIN_ as of now.

## Supported formats

| Name                  | Example                         |
| --------------------- | ------------------------------- |
| RFC2822               | Fri, 28 Nov 2014 12:00:09 +0000 |
| RFC3339               | 2014-11-28T12:00:09+00:00       |
| Nginx Log format      | 04/Nov/2018:12:13:49            |
| %Y-%m-%d %H:%M:%S.%6f | 2018-11-03 12:19:36.361297      |

## Roadmap

* [ ] Support all standard datetime formats.
* [ ] Allow custom datetime format.
* [ ] Allow specifying source timezone (currently supports only UTC).
* [ ] Auto-detect source timezone if possible.
* [ ] Add option to read from file.
* [ ] Performance optimizations
