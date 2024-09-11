# Compact Genome

[![Version](https://img.shields.io/crates/v/compact-genome.svg?style=flat-square)](https://crates.io/crates/compact-genome)
[![Downloads](https://img.shields.io/crates/d/compact-genome.svg?style=flat-square)](https://crates.io/crates/compact-genome)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/compact-genome)

A Rust crate to represent a genome string in memory.

The crate defines trait abstractions over a genome string, and provides different compact implementations.
At the moment, a bitpacked representation along with a basic ASCII representation of the base characters is supported.
