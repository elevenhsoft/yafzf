use colored::Colorize;
use std::thread;

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
        let handles: Vec<_> = self
            .stack
            .into_iter()
            .map(|item| {
                let query_clone = query.clone();
                thread::spawn(move || {
                    let mut once = 0;
                    matcher(item.to_string(), &query_clone)
                        .iter()
                        .for_each(|line| {
                            if once < 1 {
                                println!("{}", item.as_str().purple());
                            }
                            println!("{}", line);

                            once += 1;
                        })
                })
            })
            .collect();

        handles
            .into_iter()
            .for_each(|handle| handle.join().unwrap())
    }
}
