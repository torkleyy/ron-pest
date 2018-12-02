#!/usr/bin/env run-cargo-script

//! ```cargo
//! [dependencies]
//! quote = "*"
//! pest_generator = "*"
//! ```

#[macro_use]
extern crate quote;
extern crate pest_generator;

use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

fn main() {
    let pest = Path::new(
        "grammar.pest"
    );
    let rs = Path::new(
        "src/grammar.rs"
    );

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct RonParser;
        };
        derive_parser(pest, false)
    };

    let mut file = File::create(rs).unwrap();

    writeln!(file, "pub struct RonParser;\n{}", derived,).unwrap();
}
