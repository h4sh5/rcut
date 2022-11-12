# rcut

a rust implementation of cut, mostly done to learn rust

the main difference is that the delimeter `-d` supports strings, not just single characters. 

## compile

install rust, then run `cargo build` and find the `rcut` binary somewhere in `target/`

to build with cosmopolitan libc (to make an actual portable executable that can run on both linux, windows, mac and BSDs):

- install cosmopolitan into libcosmo

```
mkdir libcosmo
cd libcosmo
wget https://justine.lol/cosmopolitan/cosmopolitan.zip
unzip cosmopolitan.zip
ls -al
# should have cosmopolitan.a, ape.lds etc.
cd ../
```

- install rust toolchains

```
# on Debian / Fedora, do this
rustup toolchain install nightly-x86_64-unknown-linux-gnu
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
# on Alpine Linux / any with musl instead of glibc, you may need to do
rustup toolchain install nightly-x86_64-unknown-linux-musl
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-musl
```

- run `./cosmo_build.sh`

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