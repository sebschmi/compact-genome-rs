# Compact Genome

[![](http://meritbadge.herokuapp.com/compact-genome)](https://crates.io/crates/compact-genome)
[![](https://docs.rs/compact-genome/badge.svg)](https://docs.rs/compact-genome)
![](https://github.com/sebschmi/compact-genome-rs/workflows/Tests%20and%20Lints/badge.svg?branch=main)

A Rust crate to represent a genome string in memory.

The crate defines trait abstractions over a genome string, and provides different compact implementations.
At the moment, a bitpacked representation along with a basic ASCII representation of the base characters is supported.
