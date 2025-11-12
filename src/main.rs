use std::env;
use std::io::stdout;
use unit_converter::run;

fn main() {
    // discard program name and run
    let args: Vec<String> = env::args().skip(1).collect();
    run(args, &mut stdout());
}