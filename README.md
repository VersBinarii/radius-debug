# Hello FreeRADIUS user

Do you find yourself overwhelmed by the radius debug output?

Trying to keep up with with constantly escaping lines of text?

Unable to find the request you're looking for?

Tired of the accounting packets creating noise for your authentication packets?

Your nightmare is over!

Introducing the fantastic radius-debug. Its easy to use, its fast, it's written in Rust, it works!

Just run your radiusd -X and pipe it to radius-debug. That's right, you heard me right. Just a simple pipe like so:

``` shell
/usr/local/sbin/radiusd -X | radius-debug -t auth
```
and leave those pesky accounting packets behind.


What is is? You want specific pattern? Look no further...

``` shell
/usr/local/sbin/radiusd -X | radius-debug -t auth -p my_username
```
 and worry no more about missing your favourite Access-Accept.

##### Get it now from:

`https://github.com/VersBinarii/radius-debug`
or
`cargo install radius-debug`

##### Usage

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
