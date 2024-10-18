extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;
use std::env;
use std::io::{self, BufReader, BufWriter, Read};


fn main()-> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() !=2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = format!("{}.gz", input_file);

    let input = File::open(input_file)?;
    let reader = BufReader::new(input);

    let output = File::create(output_file)?;
    let writer = BufWriter::new(output);

    let mut encoder = GzEncoder::new(writer, Compression::default());

    io::copy(&mut reader.take(u64::MAX), &mut encoder)?;

    encoder.finish()?;

    println!("File compressed successfully!");
    Ok(())
}
