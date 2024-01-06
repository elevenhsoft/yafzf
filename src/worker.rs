use std::sync::{Arc, Mutex};

use colored::Colorize;
use rayon::prelude::*;

use crate::common::matcher;
use crate::haystack::Haystack;

#[derive(Clone)]
pub struct Worker {
    stack: Haystack,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            stack: Haystack::new(),
        }
    }

    pub fn fill_stack(mut self, stack: Haystack) -> Self {
        self.stack = stack;
        self
    }

    pub fn run(self, query: String) {
        self.stack.paths.par_iter().for_each(|item| {
            let query_clone = query.clone();
            let once = Arc::new(Mutex::new(0));
            matcher(item.to_string(), &query_clone)
                .par_iter()
                .for_each(|line| {
                    if *once.lock().unwrap() < 1 {
                        println!("{}", item.as_str().purple());
                    }
                    println!("{}", line);

                    *once.lock().unwrap() += 1;
                })
        });
    }
}
