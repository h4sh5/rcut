use std::io;
// use std::io::prelude::*;


use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::{BufRead};
use std::path::Path;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct RcutOpt {
    /// Activate verbose mode
    // short and long flags (-v, --verbose) will be deduced from the field's name
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(short = "d", long = "delimeter", default_value = " ")]
    delim: String,

    #[structopt(short = "f", long = "fields", default_value = "1")]
    fields: String,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf> // makes it optional

}

//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>   
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_nth_token(line: &String, delim: String, field: usize) -> String {
    let token_opt =  line.split(&delim).nth(field - 1); //unwrap().to_string();
    if token_opt == None {
        return String::from("");
    }
    return token_opt.unwrap().to_string();

}

fn main() {
    let opt = RcutOpt::from_args();
    
    if opt.verbose {
        println!("{:?}", opt);
        // println!("delim: {}", opt.delim);
    }

    let mut fields = Vec::new();
    for d in opt.fields.split(',') {
        fields.push(d.parse::<usize>().unwrap());
    }

    let delim = &opt.delim;


    if opt.input == None {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            // println!("{}", line.unwrap());
            let l = &line.unwrap();
            for f in &fields {
                // default output separator is space
                print!("{} ", get_nth_token(l, delim.to_string(), *f));
            }
            println!();
            
        }
    } else {
        if let Ok(lines) = read_lines(&opt.input.unwrap().into_os_string()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            // println!("{:?}", line);
            
            for f in &fields {
                if let Ok(l) = &line {
                    print!("{} ", get_nth_token(&l, delim.to_string(), *f));
                }
            }
            println!();
        }
        }
       
    }
    
     


    
}