extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::os;

use std::io;
use std::io::{IoResult, IoError};
use std::io::BufferedReader;
use std::io::File;

pub fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

pub fn cat(v: &Vec<String>, options: &getopts::Matches) {
    if v.is_empty() {
        cat_stdin(options);
    }
    else {
        // read files
        cat_files(v, options);
    }
}

fn cat_files(v: &Vec<String>, options: &getopts::Matches) {
    let mut printempty: bool = false;
    for filename in v.iter() {
        let path = Path::new(filename);
        let mut file = BufferedReader::new(File::open(&path));
        for line in file.lines() {
            printempty = handle_line(line, &mut printempty, options);
        }
    }
}

fn cat_stdin(options: &getopts::Matches) {
    let mut printempty: bool = false;
    for line in io::stdin().lock().lines() {
        printempty = handle_line(line, &mut printempty, options);
    }
}

fn handle_line<'a>(
    line: IoResult<String>,
    printempty: &'a mut bool,
    options: &getopts::Matches) -> bool {
    let linestr = line.unwrap();
    if options.opt_present("s") {
        if !is_empty(&linestr) {
            if *printempty == true {
                println!("");
            }
            print!("{}", linestr);
        }
        else {
            *printempty = true;
        }
    }
    else {
        print!("{}", linestr);
    }
    *printempty
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
}
