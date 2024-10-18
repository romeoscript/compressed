extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::Instant;
use std::io::copy;
use std::env::args


fn main() {
    if args().len() != 3 {
        eprint!()
        return;
    }
}
