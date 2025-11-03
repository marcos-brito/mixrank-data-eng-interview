use std::io::{self, Read};

use data_eng_interview::driver;

fn main() {
    let mut buf = String::new();

    io::stdin()
        .read_to_string(&mut buf)
        .expect("can't read from stdin");

    driver::single_thread(buf);
}
