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
