extern crate getopts;
use getopts::{optflag,getopts,OptGroup,usage};
use std::os;

use std::num::FromPrimitive;
use std::num::Int;
use std::char;
use std::old_io;
use std::old_io::{IoResult, IoError, BufferedReader, File, stdio};

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
    print!("     {}\t", linenum);
    print_unnumbered_buf(out_buf, options);
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
    let mut t = 0;
    let mut buf: Vec<u8> = vec![];
    for b in out_buf.iter() {
        if t > 0 {
            buf.push(*b);
            t = t - 1;
            if t == 0 {
                print_u8_buf(&mut buf, options);
                buf.clear();
            }
        }
        else {
            if *b >= 240 {
                //4th byte
                t = 3;
                buf.push(*b);
            }
            else if *b >= 224 {
                t = 2;
                buf.push(*b);
            }
            else if *b >= 192 {
                t = 1;
                buf.push(*b);
            }
            else {
                print_byte(b, options);
            }
        }
    }
}

fn print_u8_buf(buf: &mut Vec<u8>, options: &getopts::Matches) {
    match convert_buf_to_codepoint(buf) {
        Ok(codepoint) => {
            match char::from_u32(codepoint) {
                Some(c) => print!("{}", c),
                None => {
                    for b in buf.iter() {
                        print_byte(b, options);
                    }
                }

            }
        },
        Err(_) => {
            for b in buf.iter() {
                print_byte(b, options);
            }
        }
    }
}

fn convert_buf_to_codepoint(buf: &mut Vec<u8>) -> Result<u32, String> {
    let mut s = 0u32;
    let mut l = buf.len();
    let orig_l = l;
    for b in buf.iter() {
        if l == 4 {
            s = s + ((*b as u32 | 240) - 240) << 19;
        }
        else if l == 3 {
            if orig_l == 4 && *b > 191 {
                return Err("Couldn't parse".to_string());
            }
            else {
                s = s + ((*b as u32 | 224) - 224) << 12;
            }
        }
        else if l == 2 {
            if (orig_l == 4 || orig_l == 3) && *b > 191 {
                return Err("Couldn't parse".to_string());
            }
            else {
                s = s + (((*b as u32 | 192) - 192) << 6);
            }
        }
        else if l == 1 {
            if orig_l > 1 {
                if *b > 191 {
                    return Err("Couldn't parse".to_string());
                }
                else {
                    s = s + ((*b as u32 | 128) - 128);
                }
            }
            else {
                s = s + ((*b as u32 | 192) - 192);
            }
        }
        l = l - 1;
    }
    Ok(s)
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
    use super::convert_buf_to_codepoint;

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

    #[test]
    fn assert_buf_to_codepoint_24() {
        let mut b = vec![24u8];
        assert_eq!(convert_buf_to_codepoint(&mut b).unwrap(), 24u32);
    }

    #[test]
    fn assert_buf_to_codepoint_36() {
        let mut b = vec![164u8];
        assert_eq!(convert_buf_to_codepoint(&mut b).unwrap(), 36);
    }

    #[test]
    fn assert_buf_to_codepoint_162() {
        // ¢
        // C2 A2
        // 11000010 10100010
        // 00010100010
        let mut b = vec![194u8, 162];
        assert_eq!(convert_buf_to_codepoint(&mut b).unwrap(), 162);
    }

    #[test]
    fn assert_buf_to_codepoint_8364() {
        // €
        // E2 82 AC
        // 11100010 10000010 10101100
        // 0010000010101100
        let mut b = vec![226u8, 130, 172];
        assert_eq!(convert_buf_to_codepoint(&mut b).unwrap(), 8364);
    }

    #[test]
    fn assert_buf_to_codepoint_294() {
        // c4 a6
        // 11000100 10100110
        // 00100100110
        let mut b = vec![196u8, 166];
        assert_eq!(convert_buf_to_codepoint(&mut b).unwrap(), 294);
    }

    #[test]
    fn assert_buf_to_codepoint_66376() {
        // F0 90 8D 88
        // 11110000 10010000 10001101 10001000
        // 00001 00000011 01001000
        let mut b = vec![240u8, 144, 141, 136];
        assert_eq!(convert_buf_to_codepoint(&mut b).unwrap(), 66376);
    }

    #[test]
    fn assert_buf_to_codepoint_192193_err() {
        let mut b = vec![192u8, 193];
        assert!(convert_buf_to_codepoint(&mut b).is_err());
    }
    
}
