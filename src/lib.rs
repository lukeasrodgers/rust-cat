extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::os;

use std::io;
use std::io::BufferedReader;
use std::io::File;

pub fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

pub fn cat(v: &Vec<String>, options: &getopts::Matches) {
    if v.is_empty() {
        cat_stdin();
    }
    else {
        // read files
        cat_files(v);
    }
}

fn cat_files(v: &Vec<String>) {
    for filename in v.iter() {
        let path = Path::new(filename);
        let mut file = BufferedReader::new(File::open(&path));
        for line in file.lines() {
            print!("{}", line.unwrap());
        }
    }
}

fn cat_stdin() {
    for line in io::stdin().lock().lines() {
        print!("{}", line.unwrap());
    }
}
