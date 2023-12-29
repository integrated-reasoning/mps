# mps

A fast MPS parser written in Rust

[![ci](https://github.com/integrated-reasoning/mps/actions/workflows/ci.yml/badge.svg)](https://github.com/integrated-reasoning/mps/actions/workflows/ci.yml)
![docs.rs](https://img.shields.io/docsrs/mps)
[![dependency status](https://deps.rs/repo/github/integrated-reasoning/mps/status.svg)](https://deps.rs/repo/github/integrated-reasoning/mps)
[![license: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![codecov](https://codecov.io/github/integrated-reasoning/mps/graph/badge.svg?token=K0GLHFU1ZF)](https://codecov.io/github/integrated-reasoning/mps)
![Docker Image Size (tag)](https://img.shields.io/docker/image-size/integratedreasoning/mps/latest)
[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/integrated-reasoning/mps/badge)](https://flakehub.com/flake/integrated-reasoning/mps)
[![Minimum Stable Rust Version](https://img.shields.io/badge/Rust-1.71.1-blue?color=fc8d62&logo=rust)](https://blog.rust-lang.org/2023/08/03/Rust-1.71.1.html)

## About

`mps` is a parser for the Mathematical Programming System (MPS) file format, commonly used to represent optimization problems.

This crate provides both a library and a CLI for parsing MPS data. Key features include:

- **Configurable Parsing**:
  - Supported feature flags:
    - `trace` - Enhanced debugging and statistics via `nom_tracable` and `nom_locate`.
    - `proptest` - Property testing integrations.
    - `cli` - Command line interface.
- **Robustness**: Extensively tested against [Netlib LP test suite](http://www.netlib.org/lp/data/).
- **Performance**: Benchmarked using [Criterion.rs](https://github.com/bheisler/criterion.rs).

## Examples

**Library**

```rust
use mps::Parser;

let contents = "MPS data...";
match Parser::<f32>::parse(&contents) {
    Ok((_, model)) => { /* use MPS model */ },
    Err(e) => eprintln!("Parsing error: {}", e),
}
```

**CLI**

```bash
$ mps --input-path ./data/netlib/afiro
```

## Usage as a flake

Add `mps` to your `flake.nix`:

```nix
{
  inputs.mps.url = "https://flakehub.com/f/integrated-reasoning/mps/*.tar.gz";

  outputs = { self, mps }: {
    # Use in your outputs
  };
}

```

## Running with Docker

```bash
docker run -it integratedreasoning/mps:latest
```
