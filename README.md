# Nagger
This repository contains all of the custom written code lints for the Rust programming language.

## Repository structure
`ci` - Various CI related files.  
`existing_lints` - Contains implemented or to-be-implemented lints.
`lint_template` - The bare minimum template to speed up the creation of new lints.
`scripts` - Scripts to help get a new lint up and running.

## Requirements
To be able to build the lints, the following packages need to be installed using `cargo`.
```
cargo install cargo-dylint dylint-link
```

Additionally, on linux `pkg-config` and `libssl-dev` packages should be installed.

## Usage
#### Building / Testing
Navigate to the `existing_lints` folder and run `cargo build` or `cargo test` to build or test respectively. 

#### Running the linter
To run the linter, first export an environmental variable `DYLINT_LIBRARY_PATH` that points to where the libraries are built and then run `cargo dylint --workspace --all`
##### Example
```
cd nagger_repository 
export DYLINT_LIBRARY_PATH=$PWD/existing_lints/target/debug
cd some_project
cargo dylint --workspace --all
```

## Creating your own lints
To create your own lints, copy one of the existing lints and edit it.

Don't forget to add the newly created project to the workspace in `existing_lints/Cargo.toml`.

## Resources on writing lints
Some dylint examples - https://github.com/trailofbits/dylint/tree/master/examples  
Examples on testing lints - https://github.com/trailofbits/dylint/tree/master/utils/testing  
Writeup on adding new lints to clippy (same ideas apply to dylint) - https://github.com/rust-lang/rust-clippy/blob/master/doc/adding_lints.md  
Online tool that generates code to detect a specific expression - https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=9a12cb60e5c6ad4e3003ac6d5e63cf55  
Some guidelines on diagnostics - https://rustc-dev-guide.rust-lang.org/diagnostics.html
