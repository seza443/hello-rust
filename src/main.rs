// Example from https://www.rust-lang.org/learn/get-started
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello mate :) Here is a simple Rust example!";
    let width = out.len();

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}