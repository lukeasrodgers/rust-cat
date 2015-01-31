extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::os;

use std::old_io;
use std::old_io::{IoResult, IoError};
use std::old_io::BufferedReader;
use std::old_io::File;
use std::old_io::stdio;

pub fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

pub fn cat(v: &Vec<String>, options: &getopts::Matches) {
    if options.opt_present("u") {
        let writer = Box::new(stdio::stdout_raw());
        stdio::set_stdout(writer);
    }
    if v.is_empty() {
        cat_stdin(options);
    }
    else {
        // read files
        cat_files(v, options);
    }
}

fn cat_files(v: &Vec<String>, options: &getopts::Matches) {
    for filename in v.iter() {
        let path = Path::new(filename);
        match File::open(&path) {
            Ok(fh) => {
                let mut file = BufferedReader::new(fh);
                cat_file(file, options);
            },
            Err(f) => {
                println!("{}", f);
            }
        }
    }
}

fn cat_stdin(options: &getopts::Matches) {
    let mut printempty: bool = false;
    let mut linenum = 1u32;
    for line in old_io::stdin().lock().lines() {
        let (a, b) = handle_line(line, &mut printempty, &mut linenum, options);
        printempty = a;
        linenum = b;
    }
}

fn cat_file(mut file: old_io::BufferedReader<std::old_io::fs::File>, options: &getopts::Matches) {
    let mut printempty: bool = false;
    let mut linenum = 1u32;
    for line in file.lines() {
        let (a, b) = handle_line(line, &mut printempty, &mut linenum, options);
        printempty = a;
        linenum = b;
    }
}

fn handle_line<'a>(
    line: IoResult<String>,
    printempty: &'a mut bool,
    linenum: &'a mut u32,
    options: &getopts::Matches) -> (bool, u32) {
    let linestr = line.unwrap();
    if options.opt_present("s") {
        if !is_empty(&linestr) {
            if *printempty == true {
                println!("");
            }
            *linenum = *print_string(&linestr, linenum, options);
        }
        else {
            *printempty = true;
        }
    }
    else {
        *linenum = *print_string(&linestr, linenum, options);
    }
    (*printempty, *linenum)
}

fn print_string<'a>(s: &String, linenum: &'a mut u32, options: &getopts::Matches) -> &'a mut u32 {
    if options.opt_present("b") {
        if !is_empty(s) {
            print_numbered(s, linenum, options);
            *linenum = *linenum + 1;
        }
        else {
            print_unnumbered(s, options);
        }
    }
    else if options.opt_present("n") {
        print_numbered(s, linenum, options);
        *linenum = *linenum + 1;
    }
    else {
        print_unnumbered(s, options);
    }
    linenum
}

fn print_numbered<'a>(s: &String, linenum: &'a mut u32, options: &getopts::Matches) {
    if options.opt_present("v") || options.opt_present("t") {
        print!("     {}\t", linenum);
        for b in s.as_bytes().iter() {
            print_byte(b, options);
        }
    }
    else {
        print!("     {}\t{}", linenum, s);
    }
}

fn print_unnumbered(s: &String, options: &getopts::Matches) {
    if options.opt_present("v") || options.opt_present("t") {
        for b in s.as_bytes().iter() {
            print_byte(b, options);
        }
    }
    else {
        print!("{}", s);
    }
}

fn is_empty(line: &String) -> bool {
    let newline_str = "\n".to_string();
    let crlf_str = "\r\n".to_string();
    if line.is_empty() {
        true
    }
    else if line.eq(&newline_str) || line.eq(&crlf_str) {
        true
    }
    else {
        false
    }
}

fn print_byte(b: &u8, options: &getopts::Matches) {
    if *b > 31 || *b == 10 || *b == 9 {
        if options.opt_present("t") && *b == 9 {
            print!("^{}", (*b + 64) as char);
        }
        else {
            print!("{}", *b as char);
        }
    }
    else {
        print!("^{}", (*b + 64) as char);
    }
}

#[cfg(test)]
mod test {
    use super::is_empty;

    #[test]
    fn assert_is_empty() {
        let s = "".to_string();
        assert!(is_empty(&s));
    }
    #[test]
    fn assert_is_empty_newline() {
        let s = "\n".to_string();
        assert!(is_empty(&s));
    }

    #[test]
    fn assert_is_empty_crlf() {
        let s = "\r\n".to_string();
        assert!(is_empty(&s));
    }

    #[test]
    fn assert_is_empty_char() {
        let s = "d".to_string();
        assert!(!is_empty(&s));
    }

    #[test]
    fn assert_is_empty_ctrl_char() {
        let s = "".to_string();
        assert!(!is_empty(&s));
    }
}
