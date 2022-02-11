//! Dumps grpc logs somewhere

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::{entries::Entries, error::Result, path::RecursiveDirectoryIterator};

pub struct Dump {
    start_path: PathBuf,
}

impl Dump {
    /// Dumps contents in all files in the specified path (and its children)
    pub fn new(start_path: impl Into<PathBuf>) -> Self {
        let start_path = start_path.into();

        Self { start_path }
    }

    pub fn dump<W: Write>(&mut self, mut out: W) -> Result<()> {
        writeln!(
            out,
            "Attempt to dump gRPC frames from all .txt files starting at {:?}",
            self.start_path
        )?;

        let paths = RecursiveDirectoryIterator::new(self.start_path.clone());

        for p in paths {
            self.dump_path(&mut out, &p)?;
        }
        Ok(())
    }

    pub fn dump_path<W: Write>(&self, mut out: W, p: &Path) -> Result<()> {
        //println!("path: {:?}", p);

        // skip anything without extension
        let extension = if let Some(extension) = p.extension() {
            extension.to_string_lossy()
        } else {
            return Ok(());
        };

        if extension != "txt" {
            return Ok(());
        }
        println!("Attempting to dump {:?}", p);
        let entries = match Entries::try_new(p) {
            Ok(entries) => entries,
            Err(e) => {
                println!("Error reading {:?}: {}", p, e);
                return Ok(());
            }
        };

        entries.take(10).for_each(|entry| {
            println!("Decoded entry: {:?}", entry);
        });

        Ok(())
    }
}
