use std::io;

use data_eng_interview::driver;

fn main() {
    driver::worker_pool(32, io::stdin().lock());
}
