# A simple helper for filtering output of the freeRADIUS debug output

## Installation

`cargo install radius-debug`

## Usage

`radiusd -X | radius-debug -t auth -p username`

## Help

```
FreeRADIUS-debug-helper 0.1.0
Filters out FreeRADIUS debug

USAGE:
    radius-debug [OPTIONS] --type <packet type>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --type <packet type>    Filter packet types. [possible values: auth, acct]
    -p, --pattern <pattern>     Search for pattern in the packets
```
