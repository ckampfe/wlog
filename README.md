# wlog

Like `watch`, but with logging.
Useful to repeatedly run a command and see what changed in successive invocations.

---

## example

```
$ wlog -n 2 "wc -l src/main.rs"
16:52:56 [INFO]       93 src/main.rs

16:52:58 [INFO]       93 src/main.rs

16:53:00 [INFO]       93 src/main.rs

16:53:02 [INFO]       93 src/main.rs

16:53:04 [INFO]       93 src/main.rs

16:53:06 [INFO]       93 src/main.rs

16:53:08 [INFO]       93 src/main.rs
```

## use

```
$ wlog -h
wlog ff6ac2c

USAGE:
    wlog [FLAGS] [OPTIONS] <command>

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Suppress terminal output. Has no effect on file output
    -V, --version    Prints version information

OPTIONS:
    -f, --file <output>         Log output to a file
    -n, --interval <seconds>    Time in seconds between two ticks [default: 2]

ARGS:
    <command>    The command to repeat
```
