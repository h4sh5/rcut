use std::io;
use std::io::prelude::*;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct RcutOpt {
    /// Activate verbose mode
    // short and long flags (-v, --verbose) will be deduced from the field's name
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(short = "d", long = "delimeter", default_value = " ")]
    delim: String,

    #[structopt(short = "f", long = "field", default_value = "1")]
    field: i32,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf> // makes it optional

}

    

fn main() {
    let opt = RcutOpt::from_args();
    println!("{:?}", opt);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}