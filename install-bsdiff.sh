#!/usr/bin/expect -f

set timeout -1

spawn cargo php install --release

expect "Are you sure you want to install the extension *n\]"

send -- "yes\r"

expect eof
