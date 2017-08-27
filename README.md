cargo-docserve
=========

[![](https://img.shields.io/crates/v/cargo-docserve.svg)](https://crates.io/crates/cargo-docserve)

Add a `docserve` command to `cargo`.

`cargo-docserve` builds the documentation for your crate and its dependencies,
then runs a tiny static webserver in order to serve the HTML. This is useful
when developing on a remote machine where you don't have a web browser (and
thus can't just `cargo doc --open`).


### Installation

`cargo install cargo-docserve`


### Usage

`cargo docserve -b 0.0.0.0:8888`
