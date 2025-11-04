//! Multiple implementations of driver/glue functions using different concurrency patterns

use crate::{csv_writer::CsvWriter, finder::Finder, worker::Pool};
use std::{io::BufRead, sync::mpsc, thread};

/// Uses a single thread to read, process and write.
pub fn single_thread(reader: impl BufRead) {
    let mut csv = CsvWriter::default();

    for line in reader.lines() {
        if let Ok(url) = line {
            let result = Finder::new().find(url);

            csv.add_record(result.to_csv());
        }
    }
}

pub fn worker_pool(size: usize, reader: impl BufRead) {
    let (results_tx, results_rx) = mpsc::channel();
    let pool = Pool::new(size, results_tx);
    let mut csv = CsvWriter::default();

    for line in reader.lines() {
        if let Ok(url) = line {
            pool.send(url);
        }
    }

    pool.close();

    for site in results_rx {
        csv.add_record(site.to_csv());
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
pub fn fork_join(reader: impl BufRead) {
    let mut csv = CsvWriter::default();
    let mut handles = Vec::new();

    for line in reader.lines() {
        if let Ok(url) = line {
            let handle = thread::spawn(move || {
                let finder = Finder::new();
                finder.find(url)
            });

            handles.push(handle);
        }
    }

    for handle in handles {
        let result = handle.join().unwrap();
        csv.add_record(result.to_csv());
    }
}
