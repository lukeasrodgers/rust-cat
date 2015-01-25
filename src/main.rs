extern crate getopts;
extern crate "rust-cat" as rustcat;

use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::os;

use rustcat::{print_usage,cat};

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = &[
        optflag("b", "number non-blank lines", "Number the non-blank output lines, starting at 1."),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let input_files = matches.free.clone();
    cat(&input_files, &matches)
}
