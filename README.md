# Certstream-rs

**Certstream-rs** is a library for interacting with the [certstream network](https://certstream.calidog.io/) to monitor an aggregated feed from a collection of [Certificate Transparency Logs](https://www.certificate-transparency.org/known-logs).

## Installation

To add the library's Git repository to a Cargo project, add this to your Cargo.toml

```INI
[dependencies]
certstream = { git = "https://github.com/bradfordboyle/certstream-rs" }
```

And add `extern crate certstream;` to your project.

## Usage

The examples (located in `examples/`) can be run with:

```sh
cargo run --example client
```
