extern crate clap;
extern crate hound;
extern crate filter_lib;

use clap::{App, Arg};
use hound::{WavReader, WavWriter};
use filter_lib::FilterTwoPole;

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

    let mut reader = WavReader::open(infile).unwrap();
    let input_spec = reader.spec();

    let mut writer = WavWriter::create(outfile, input_spec).unwrap();

    // work out scaling to convert samples to f32 range -1.0 to 1.0
    let fp_outscale = ((1 << (input_spec.bits_per_sample - 1)) - 1) as f32;
    let fp_inscale = 1.0 / fp_outscale;

    // create filter
    let mut filter = FilterTwoPole::lowpass(500.0, input_spec.sample_rate);

    // process
    for maybe_sample in reader.samples::<i32>() {
        let sample = (maybe_sample.unwrap() as f32) * fp_inscale;

        let filtered = filter.process_sample(sample);

        // apply clipping
        let clipped = filtered.min(1.0).max(-1.0);

        writer.write_sample((clipped * fp_outscale) as i32).unwrap();
    }

    writer.finalize().unwrap();

}
