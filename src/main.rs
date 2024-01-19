mod problems;

use problems::p18110::_solve;

// Copy-Paste below.

use std::io::{self, Read, Write};

fn main() {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());

    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let ans = _solve(input);
    writeln!(output, "{}", ans).unwrap();
}
