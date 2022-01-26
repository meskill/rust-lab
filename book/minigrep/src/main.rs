use std::env;
use std::process;

use minigrep::run;

fn main() {
    let args = env::args();

    if let Err(x) = run(args) {
        eprintln!("{}", x);
        process::exit(1);
    }
}
