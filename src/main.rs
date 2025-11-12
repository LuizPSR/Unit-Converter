use std::env;
use unit_converter::run;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    run(args);
}