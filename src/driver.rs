//! Multiple implementations of driver/glue functions using different concurrency patterns

use crate::{csv_writer::CsvWriter, finder::Finder};
use std::thread;

/// Uses a single thread to read, process and write.
pub fn single_thread(input: String) {
    let mut csv = CsvWriter::default();

    for line in input.lines() {
        let finder = Finder::new();
        let result = finder.find(line);

        csv.add_record(result.to_csv());
    }
}

/// Uses a fork-join pattern to process domains.
///
/// # Resources
///
/// This will spawn an OS thread for each line in
/// the input which can get very expensive and maybe
/// exaust system resources. Be mindful with the amount
/// of work that is passed to `stdin`.
pub fn fork_join(input: String) {
    let mut csv = CsvWriter::default();
    let mut handles = Vec::new();

    for line in input.lines().map(|l| l.to_string()) {
        let handle = thread::spawn(move || {
            let finder = Finder::new();
            finder.find(line)
        });

        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap();
        csv.add_record(result.to_csv());
    }
}
