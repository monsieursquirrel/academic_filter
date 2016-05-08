extern crate clap;
extern crate hound;

use clap::{App, Arg};

fn main() {
    let args = App::new("Academic filter")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use. Must be a wav file.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file to use.")
            .required(true)
            .index(2))
        .get_matches();

    let infile = args.value_of("INPUT").unwrap();
    let outfile = args.value_of("OUTPUT").unwrap();
}
