extern crate getopts;
extern crate "rust-cat" as rustcat;

use getopts::{optflag,getopts,OptGroup,usage};
use std::os;

use rustcat::{print_usage,cat};

fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = &[
        optflag("b", "number non-blank lines", "Number the non-blank output lines, starting at 1."),
        optflag("s", "squeeze", "Squeeze multiple adjacent empty lines, causing the output to be single spaced."),
        optflag("b", "number non-blank", "Number the non-blank output lines, starting at 1."),
        optflag("n", "number output", "Number the output lines, starting at 1."),
        optflag("v", "display non-printing characters", "Display non-printing characters so they are visible.  Control characters print as `^X' for control-X; the delete character (octal 0177) prints as `^?'.  Non-ASCII characters (with the high bit set) are printed as `M-' (for meta) followed by the character for the low 7 bits."),
        optflag("t", "display non-printing characters and tabs", "Display non-printing characters (see the -v option), and display tab characters as `^I'."),
        optflag("u", "disable buffering", "Disable output buffering."),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => {
            print_usage(program.as_slice(), opts);
            panic!(f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let input_files = matches.free.clone();
    cat(&input_files, &matches)
}
