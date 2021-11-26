# rcut

a rust implementation of cut, mostly done to learn rust

## compile

install rust, then run `cargo build` and find the `rcut` binary somewhere in `target/`

## usage

usage is pretty much the same as `cut`, and the only currently supported flags are `-d` and `-f`

example usage:
`rcut -d , -f 1 test.csv`

or 
`cat /etc/passwd | rcut -d : -f 2`