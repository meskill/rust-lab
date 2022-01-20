use std::env;
use std::process;

use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(x) = run(&args) {
        eprintln!("{}", x);
        process::exit(1);
    }
}
