use ignore::{Walk, WalkBuilder};
use std::{collections::HashSet, path::PathBuf};

pub struct HaystackBuilder {
    walker: Walk,
}

impl HaystackBuilder {
    pub fn new(path: PathBuf) -> Self {
        let walker = WalkBuilder::new(path).build();

        Self { walker }
    }

    pub fn build(self) -> Haystack {
        let mut vec = Vec::new();

        self.walker.for_each(|result| {
            if let Ok(entry) = result {
                if let Some(path) = entry.path().to_str() {
                    vec.push(path.to_string());
                }
            }
        });

        let stack: HashSet<String> = vec.into_iter().collect();
        let haystack = Haystack::new();

        haystack.pusher(stack)
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

impl<'a> IntoIterator for &'a Haystack {
    type Item = String;
    type IntoIter = std::collections::hash_set::IntoIter<String>;

    fn into_iter(self) -> std::collections::hash_set::IntoIter<String> {
        let paths = self.paths.clone();
        paths.into_iter()
    }
}
