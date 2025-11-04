use std::io;

use data_eng_interview::driver;

fn main() {
    driver::single_thread(io::stdin().lock());
}
