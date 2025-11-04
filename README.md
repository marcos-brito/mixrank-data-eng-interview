# Added dependencies

- `reqwest`: Http clients and requests
- `scraper`: Querying HTML documents
- `log`: Logging API (no actual implementation)
- `criterion`: Benchmarking. (dev only).
- `tokio`: Async run time. (async branch only).

# Nix package

I made a few changes to the Nix package so it runs Rust instead:

- Pin a more recent tarball
- Update the checksum so it matches the new tarball
- Add rustc, cargo and openssl
- Set the path for shared libraries to fix linking error
- Remove python deps

It should run fine, but the code might take a few seconds to compile.

# Running

From the project directory, you can run the program either inside
a nix-shell or a regular shell (if the Rust toolchain is installed).

To run and see the output in the terminal:

```sh
cat website.csv | cargo run --release
```

To save the output to a file:

```sh
cat website.csv | cargo run --release > out.csv
```

The results will be written to `out.csv`.

# Code improvements

These are a few updates that would make the code better, but were not implemented:

- Serialization
- Async runtime
- Better API for selectors
- Look for logos in other sources. Not only HTML
- Output SVGs. Not only URLs

# Benches

I wrote a few benchmarks to test different concurrency patterns. They take
quite some time to run even with only 10 samples, so I wouldn't recommend.

If you want to run them, this is how:

```sh
cargo bench
```

It should just work, but it will take some time.

## Results

These were the results after running it on my machine with a reduced input:

```sh
drivers/single_thread   time:   [3.3322 s 3.8067 s 4.2840 s]
drivers/fork_join       time:   [1.1770 s 2.2269 s 3.4842 s]
drivers/worker_pool     time:   [754.90 ms 839.49 ms 926.70 ms]
pool_size/16 workers    time:   [962.06 ms 1.3301 s 1.7923 s]
pool_size/32 workers    time:   [1.1214 s 1.7618 s 2.5195 s]
pool_size/64 workers    time:   [1.0775 s 1.3365 s 1.6495 s]
```

I also ran the benches with all the lines from `websites.csv`:

```sh
drivers/worker_pool     time:   [47.239 s 48.126 s 49.208 s]
pool_size/16 workers    time:   [88.261 s 88.916 s 89.849 s]
pool_size/32 workers    time:   [46.502 s 47.028 s 47.550 s]
pool_size/64 workers    time:   [31.595 s 32.490 s 33.461 s]
```

- `single_thread` takes forever
- `fork_join` runs out of file descriptors instantly

# Objectives

- Write a program that will crawl a list of website and output their logo URLs.
- The program should read domain names on `STDIN` and write a CSV of domain and logo URL to `STDOUT`.
- A `websites.csv` list is included as a sample to crawl.
- You can't always get it right, but try to keep precision and recall as high as you can. Be prepared to explain ways you can improve. Bonus points if you can measure.
- Be prepared to discuss the bottlenecks as you scale up to millions of websites. You don't need to implement all the optimizations, but be able to talk about the next steps to scale it for production.
- Favicons aren't an adequate substitute for a logo, but if you choose, it's also valuable to extract as an additional field.
- Spare your time on implementing features that would be time consuming, but make a note of them so we can discuss the ideas.
- Please keep 3rd party dependencies to a minimum, unless you feel there's an essential reason to add a dependency.
- We use [Nix](https://nixos.org/nix/) for package management. If you add your dependencies to `default.nix`, then it's easy for us to run your code. Install nix and launch the environment with `nix-shell` (works on Linux, macOS, and most unixes).
