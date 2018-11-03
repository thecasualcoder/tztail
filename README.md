# tztail

tztail (TimeZoneTAIL) allows you to view logs in the timezone you want.

> This project is work in progress

## Usage

```bash
tztail 0.1.0
View logs in your desired timezone

USAGE:
    tztail --timezone <TIMEZONE>

OPTIONS:
    -t, --timezone <TIMEZONE>    Sets the timezone in which output should be printed
    -h, --help                   Prints help information
    -V, --version                Prints version information


## Example usage
$ tail somelog | tztail --timezone Asia/Kolkata
```

It reads from _STDIN_ as of now.

## Roadmap

- Support all standard datetime formats.
- Allow custom datetime format.
- Allow specifying source timezone (currently supports only UTC).
- Auto-detect source timezone if possible.
- Add option to read from file.
- Performance optimizations