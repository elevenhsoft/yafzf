mod cli;
pub mod common;
mod haystack;
mod worker;

use anyhow::{Error, Result};
use clap::Parser;
use std::{env::current_dir, time::Instant};

use crate::{
    cli::Cli,
    haystack::{Haystack, HaystackBuilder},
    worker::Worker,
};

fn main() -> Result<()> {
    let start_at = Instant::now();

    let args = Cli::parse();
    let query = args.query;
    let mut stack: Result<Haystack, Error> = Ok(Haystack::new());

    if let Ok(cwd) = current_dir() {
        stack = HaystackBuilder::new(cwd).build();
    };

    let mut worker = Worker::new();
    let mut files = 0;

    if let Ok(stack) = stack {
        files = stack.paths.len();
        worker = worker.fill_stack(stack);
        worker.run(query);
    }

    let elapsed = start_at.elapsed();
    println!("Found {} files in {}ms", files, elapsed.as_millis());

    Ok(())
}
