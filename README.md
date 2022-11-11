# rcut

a rust implementation of cut, mostly done to learn rust

the main difference is that the delimeter `-d` supports strings, not just single characters. 

## compile

install rust, then run `cargo build` and find the `rcut` binary somewhere in `target/`

## usage

usage is pretty much the same as `cut`, and the only currently supported flags are `-d` and `-f` (as well as the new `-n`)

example usage:

`rcut -d , -f 1 test.csv`

getting the field/column number of each field (useful for finding the field number of data in files with many columns)

It will be displayed next to the field like `[1]fieldone [2]fieldtwo [3]abc`

`rcut -d, -n test.csv`

multiple fields (output default sep'd by space)

`rcut -d , -f 1,2,3 test.csv`

using a string as delim

`rcut -d "----" -f 2 something.txt`

it also reads stdin when you don't give it a file

`cat /etc/passwd | rcut -d : -f 2`