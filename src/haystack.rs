use anyhow::Result;
use ignore::{WalkBuilder, WalkParallel, WalkState};
use rayon::prelude::*;
use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct HaystackBuilder {
    walker: WalkParallel,
}

impl HaystackBuilder {
    pub fn new(path: PathBuf) -> Self {
        let walker = WalkBuilder::new(path)
            .standard_filters(true)
            .build_parallel();

        Self { walker }
    }

    pub fn build(self) -> Result<Haystack> {
        let vec = Arc::new(Mutex::new(Vec::new()));

        self.walker.run(|| {
            let vec_copy = Arc::clone(&vec);
            Box::new(move |result| -> WalkState {
                if let Ok(path) = result {
                    match path.path().to_str() {
                        Some(string) => {
                            vec_copy.lock().unwrap().push(string.to_string());
                            WalkState::Continue
                        }
                        None => WalkState::Skip,
                    };
                    WalkState::Continue
                } else {
                    WalkState::Skip
                }
            })
        });

        vec.lock().unwrap().par_sort();
        let stack: HashSet<String> = vec.lock().unwrap().iter().cloned().collect();
        let haystack = Haystack::new();

        Ok(haystack.pusher(stack))
    }
}

#[derive(Debug, Clone)]
pub struct Haystack {
    pub paths: HashSet<String>,
}

impl Haystack {
    pub fn new() -> Self {
        Self {
            paths: HashSet::new(),
        }
    }

    pub fn pusher(mut self, stack: HashSet<String>) -> Self {
        self.paths = stack;
        self
    }
}

impl Default for Haystack {
    fn default() -> Self {
        Self::new()
    }
}
