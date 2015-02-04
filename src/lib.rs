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
    let mut linenum = 1u32;
    for filename in v.iter() {
        let path = Path::new(filename);
        match File::open(&path) {
            Ok(fh) => {
                let mut file = BufferedReader::new(fh);
                linenum = cat_file(file, options, linenum);
            },
            Err(f) => {
                println!("{}", f);
            }
        }
    }
}

fn cat_stdin(options: &getopts::Matches) {
    let mut linenum = 1u32;
    let mut printempty: bool = false;
    for line in old_io::stdin().lock().lines() {
        let (a, b) = handle_line(line.unwrap(), &mut printempty, linenum, options);
        printempty = a;
        linenum = b;
    }
}

fn cat_file<'a>(
    mut file: old_io::BufferedReader<std::old_io::fs::File>,
    options: &getopts::Matches,
    mut linenum: u32, 
    ) -> u32 {
    let mut printempty: bool = false;
    let mut read_buf = [0, 4096];
    let mut out_buf: Vec<u8> = vec![];
    loop {
        match file.read(&mut read_buf) {
            Ok(n) => {
                for c in read_buf[..n].iter() {
                    out_buf.push(*c);
                    if is_nl(c) {
                        let (a, b) = handle_buf(&out_buf, &mut printempty, linenum, options);
                        printempty = a;
                        linenum = b;
                        out_buf.clear();
                    }
                }
            },
            Err(f) => {
                let mut o = 0u32;
                if out_buf.len() > 0 {
                    handle_buf(&out_buf, &mut printempty, linenum, options);
                }
                break
            }
        }
    }
    linenum
}

fn handle_line<'a>(
    linestr: String,
    printempty: &'a mut bool,
    mut linenum: u32,
    options: &getopts::Matches) -> (bool, u32) {
    if options.opt_present("s") {
        if !is_empty(&linestr) {
            if *printempty == true {
                println!("");
            }
            linenum = print_string(&linestr, linenum, options);
        }
        else {
            *printempty = true;
        }
    }
    else {
        linenum = print_string(&linestr, linenum, options);
    }
    (*printempty, linenum)
}

fn handle_buf<'a>(
    out_buf: &Vec<u8>,
    printempty: &'a mut bool,
    mut linenum: u32,
    options: &getopts::Matches) -> (bool, u32) {
    if options.opt_present("s") {
        if !is_empty_out_buf(out_buf) {
            if *printempty == true {
                if options.opt_present("n") {
                    println!("     {}\t", linenum);
                    linenum = linenum + 1;
                }
                else {
                    println!("");
                }
            }
            linenum = print_buf(&out_buf, linenum, options);
        }
        else {
            *printempty = true;
        }
    }
    else {
        linenum = print_buf(&out_buf, linenum, options);
    }
    (*printempty, linenum)
}

fn print_string<'a>(s: &String, mut linenum: u32, options: &getopts::Matches) -> u32 {
    if options.opt_present("b") {
        if !is_empty(s) {
            print_numbered(s, linenum, options);
            linenum = linenum + 1;
        }
        else {
            print_unnumbered(s, options);
        }
    }
    else if options.opt_present("n") {
        print_numbered(s, linenum, options);
        linenum = linenum + 1;
    }
    else {
        print_unnumbered(s, options);
    }
    linenum
}

fn print_buf<'a>(out_buf: &Vec<u8>, mut linenum: u32, options: &getopts::Matches) -> u32 {
    if options.opt_present("b") {
        if !is_empty_out_buf(out_buf) {
            print_numbered_buf(out_buf, linenum, options);
            linenum = linenum + 1;
        }
        else {
            print_unnumbered_buf(out_buf, options);
        }
    }
    else if options.opt_present("n") {
        print_numbered_buf(out_buf, linenum, options);
        linenum = linenum + 1;
    }
    else {
        print_unnumbered_buf(out_buf, options);
    }
    linenum
}

fn print_numbered<'a>(s: &String, linenum: u32, options: &getopts::Matches) {
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

fn print_numbered_buf<'a>(out_buf: &Vec<u8>, linenum: u32, options: &getopts::Matches) {
    // if options.opt_present("v") || options.opt_present("t") {
        print!("     {}\t", linenum);
        for b in out_buf.iter() {
            print_byte(b, options);
        }
    // }
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

fn print_unnumbered_buf(out_buf: &Vec<u8>, options: &getopts::Matches) {
    // if options.opt_present("v") || options.opt_present("t") {
        for b in out_buf.iter() {
            print_byte(b, options);
        }
    // }
    // else {
        // print!("{}", s);
    // }
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

fn is_empty_out_buf(v: &Vec<u8>) -> bool {
    if v.len() > 1 {
        false
    }
    else if is_nl(&v[0]) {
        true
    }
    else {
        false
    }
}

fn is_nl(c: &u8) -> bool {
    *c == 10u8
}

fn print_byte(b: &u8, options: &getopts::Matches) {
    if (*b > 31 && *b < 127) || *b == 10 || *b == 9 {
        if options.opt_present("t") && *b == 9 {
            print_tab();
        }
        else {
            print!("{}", *b as char);
        }
    }
    else {
        if options.opt_present("v") || options.opt_present("t") {
            if *b < 32 {
                print!("^{}", (*b + 64) as char);
            }
            else if *b == 127 {
                // del char
                print!("^?");
            }
            else if *b > 127 {
                if *b == 255 {
                    print!("M-^?");
                }
                else {
                    let x = *b - 128;
                    if x < 32 {
                        print!("M-^{}", (x + 64) as char);
                    }
                    else {
                        print!("M-{}", (*b - 128) as char);
                    }
                }
            }
        }
    }
}

fn print_tab() {
    print!("^I");
}

#[cfg(test)]
mod tests {
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
