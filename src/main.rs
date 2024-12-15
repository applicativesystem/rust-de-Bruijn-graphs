mod args;
use args::DEBruijnArgs;
use clap::Parser;
use std::cmp::Ord;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-16

rust-debruijns-algorithm: implementation of debruijns algorithm and
graph traversal in RUST. It will also take a preformatted kmer table
and will do the string traversal and joining.

* */

fn main() {
    let args = DEBruijnArgs::parse();
    let debruijn_longread_output = debruijn_longread(&args.debruijn_arg, args.kmer_arg).unwrap();
    println!("The bwt has been written: {}", debruijn_longread_output);
}

fn debruijn_longread(path: &str, kmer: usize) -> Result<String, Box<dyn Error>> {
    Ok("The debruijn graph construction has finished".to_string())
}
