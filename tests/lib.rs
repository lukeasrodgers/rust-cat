// Copyright (c) Jordi Boggiano
// With modifications by Luke Rodgers
// https://github.com/uutils/coreutils/blob/2f0d8c89c94ee4b6c1d911c48d6e56f002e2def7/test/cat.rs

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![allow(unstable)]

use std::old_io::process::Command;
use std::str;

static PROGNAME: &'static str = "./target/rust-cat";

#[test]
fn test_output_multi_files_print_all_chars() {
    let po = match Command::new(PROGNAME)
                                .arg("tests/fixtures/alpha.txt")
                                .arg("tests/fixtures/256.txt")
                                .arg("-vt")
                                .arg("-n").output() {

        Ok(p) => p,
        Err(err) => panic!("{}", err),
    };

    let out = str::from_utf8(po.output.as_slice()).unwrap();
    assert_eq!(out,
               "     1\tabcde$\n     2\tfghij$\n     3\tklmno$\n     4\tpqrst$\n     5\tuvwxyz$\n     6\t^@^A^B^C^D^E^F^G^H^I$\n     7\t^K^L^M^N^O^P^Q^R^S^T^U^V^W^X^Y^Z^[^\\^]^^^_ !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~^?M-^@M-^AM-^BM-^CM-^DM-^EM-^FM-^GM-^HM-^IM-^JM-^KM-^LM-^MM-^NM-^OM-^PM-^QM-^RM-^SM-^TM-^UM-^VM-^WM-^XM-^YM-^ZM-^[M-^\\M-^]M-^^M-^_M- M-!M-\"M-#M-$M-%M-&M-\'M-(M-)M-*M-+M-,M--M-.M-/M-0M-1M-2M-3M-4M-5M-6M-7M-8M-9M-:M-;M-<M-=M->M-?M-@M-AM-BM-CM-DM-EM-FM-GM-HM-IM-JM-KM-LM-MM-NM-OM-PM-QM-RM-SM-TM-UM-VM-WM-XM-YM-ZM-[M-\\M-]M-^M-_M-`M-aM-bM-cM-dM-eM-fM-gM-hM-iM-jM-kM-lM-mM-nM-oM-pM-qM-rM-sM-tM-uM-vM-wM-xM-yM-zM-{M-|M-}M-~M-^?");
}

#[test]
fn test_output_files_print_all_chars() {
    let po = match Command::new(PROGNAME)
                                .arg("tests/fixtures/alpha.txt")
                                .arg("-vt")
                                .arg("-n").output() {

        Ok(p) => p,
        Err(err) => panic!("{}", err),
    };

    let out = str::from_utf8(po.output.as_slice()).unwrap();
    assert_eq!(out,
               "     1\tabcde\n     2\tfghij\n     3\tklmno\n     4\tpqrst\n     5\tuvwxyz\n");
}

#[test]
fn test_stdin_squeeze() {
    let mut process= Command::new(PROGNAME).arg("-vt").spawn().unwrap();

    process.stdin.take().unwrap().write(b"\x00\x01\x02").unwrap();
    let po = process.wait_with_output().unwrap();
    let out = str::from_utf8(po.output.as_slice()).unwrap();

    assert_eq!(out, "^@^A^B");
}

#[test]
fn test_stdin_number_non_blank() {
    let mut process = Command::new(PROGNAME).arg("-b").spawn().unwrap();

    process.stdin.take().unwrap().write(b"\na\nb\n\n\nc").unwrap();
    let po = process.wait_with_output().unwrap();
    let out =  str::from_utf8(po.output.as_slice()).unwrap();

    assert_eq!(out, "\n     1\ta\n     2\tb\n\n\n     3\tc");
}
