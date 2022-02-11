//! Dumps grpc binary log entries

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::{entries::Entries, error::Result, path::RecursiveDirectoryIterator};

pub struct DumpEntries {
    start_path: PathBuf,
}

impl DumpEntries {
    /// Dumps the raw entry contents in all files in the specified path (and its children)
    pub fn new(start_path: impl Into<PathBuf>) -> Self {
        let start_path = start_path.into();

        Self { start_path }
    }

    pub fn dump<W: Write>(&mut self, out: &mut W) -> Result<()> {
        writeln!(
            out,
            "Attempt to dump gRPC frames from all .txt files starting at {:?}",
            self.start_path
        )?;

        let paths = RecursiveDirectoryIterator::new(self.start_path.clone());

        for p in paths {
            self.dump_path(out, &p)?;
        }
        Ok(())
    }

    pub fn dump_path<W: Write>(&self, out: &mut W, p: &Path) -> Result<()> {
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
                writeln!(out, "Error reading {:?}: {}", p, e)?;
                return Ok(());
            }
        };

        let mut num_entries = 0;
        entries
            .enumerate()
            .try_for_each(|(i, entry)| {
                num_entries += 1;
                match entry {
                    Ok(entry) => {
                        writeln!(out, "{:#?}", entry)
                    },
                    Err(e) => writeln!(out, "ERROR decoding {}: {}", i, e)
                }
            })?;

        println!("Dumped {} entries", num_entries);
        Ok(())
    }
}
