//! Multiple implementations of driver/glue functions using different concurrency patterns

use crate::{csv_writer::CsvWriter, finder::Finder};
use std::io::{self, Read};

/// Uses a single thread to read, process and write.
pub fn single_thread() {
    let mut buf = String::new();
    let mut csv = CsvWriter::default();

    io::stdin().read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        let finder = Finder::new();
        let result = finder.find(line);

        csv.add_record(result.to_csv());
    }
}
