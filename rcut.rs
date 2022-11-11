use std::io;
// use std::io::prelude::*;


use std::path::PathBuf;
use structopt::StructOpt;

use std::fs::File;
use std::io::{BufRead};
use std::path::Path;

static EOLMARKER: &str  = "";

#[derive(Debug, StructOpt)]
#[structopt(name = "rcut", about = "cut written in rust, supporting string delimeters (not just a single char)")]
struct RcutOpt {
    /// Activate verbose mode
    // short and long flags (-v, --verbose) will be deduced from the field's name
    #[structopt(short, long)]
    verbose: bool,

    ///Show count number (one-indexed) with each field. -f will be ignored
    #[structopt(short = "n", long = "--show-count")]
    numbercount: bool,

    #[structopt(short = "d", long = "delimeter string", default_value = " ")]
    delim: String,

    #[structopt(short = "f", long = "fields", default_value = "1")]
    fields: String,

    // the /// comments will be used as annotation in the help output

    /// Input file (default will be stdin)
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
        return String::from(EOLMARKER);
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

        // do the if statement up here so that it doesn't have compare every time
        // pro is faster speed, con is a dup code
        if opt.numbercount == true {
            for line in stdin.lock().lines() {
                // println!("{}", line.unwrap());
                let l = &line.unwrap();
                let mut c = 1;
                loop {
                    // default output separator is space
                    let t = get_nth_token(l, delim.to_string(), c);
                    if t == EOLMARKER {
                        break;
                    }
                    // check that line has ended
                    print!("[{}]{} ", c, t);
                    c += 1;
                }
                println!();
                
            }
        } else {
            for line in stdin.lock().lines() {
                // println!("{}", line.unwrap());
                let l = &line.unwrap();
                for f in &fields {
                    // default output separator is space
                    print!("{} ", get_nth_token(l, delim.to_string(), *f));
                }
                println!();
                
            }

        }
        
    } else {
        if let Ok(lines) = read_lines(&opt.input.unwrap().into_os_string()) {
            // Consumes the iterator, returns an (Optional) String
            if opt.numbercount == true {
                for line in lines {
                    let mut c = 1;
                    loop {
                        // default output separator is space
                        if let Ok(l) = &line {
                            let t = get_nth_token(&l, delim.to_string(), c);
                            if t == EOLMARKER {
                                break;
                            }
                            // check that line has ended
                            print!("[{}]{} ", c, t);
                            c += 1;
                        }
                    }
                    println!();
                }
            } else {
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
    
     


    
}