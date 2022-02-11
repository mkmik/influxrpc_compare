use std::{path::PathBuf, collections::VecDeque};


#[derive(Debug)]
/// Recursively and incrementally walks a directory structure. Is
/// likely to get confused if the directories change during iteration
pub struct RecursiveDirectoryIterator {
    worklist :VecDeque<PathBuf>
}

impl RecursiveDirectoryIterator {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let mut worklist = VecDeque::new();
        worklist.push_back(path.into());
        Self {
            worklist
        }
    }
}

impl Iterator for RecursiveDirectoryIterator {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.worklist.pop_front() {
            let metadata = path.metadata().expect("reading metadata");

            if metadata.is_file() {
                return Some(path)
            } else if metadata.is_dir() {
                // read entries
                for entry in std::fs::read_dir(path).expect("reading directory") {
                    let entry = entry.expect("error reading directory entry");
                    self.worklist.push_back(entry.path())
                }
            }
        }

        // Nothing left in worklist
        None
    }
}
