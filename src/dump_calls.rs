//! Dumps summarized gRPC calls (summarized entries)

use std::{
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

use crate::{calls::Calls, entries::Entries, error::Result, path::LogIterator};

pub struct DumpCalls {
    start_path: PathBuf,
}

impl DumpCalls {
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

        let paths = LogIterator::new(self.start_path.clone());

        for p in paths {
            self.dump_path(out, &p)?;
        }
        Ok(())
    }

    pub fn dump_path<W: Write>(&self, out: &mut W, p: &Path) -> Result<()> {
        //println!("path: {:?}", p);
        println!("Attempting to dump {:?}", p);

        let entries = match Entries::try_new(p) {
            Ok(entries) => entries,
            Err(e) => {
                writeln!(out, "Error reading {:?}: {}", p, e)?;
                return Ok(());
            }
        };

        let start = Instant::now();

        // split them into Ok and Errors
        let (ok_entries, err_entries): (Vec<_>, Vec<_>) = entries
            .map(|result| match result {
                Ok(entry) => (Some(entry), None),
                Err(msg) => (None, Some(msg)),
            })
            .unzip();

        let ok_entries: Vec<_> = ok_entries.into_iter().filter_map(|s| s).collect();
        let err_entries: Vec<_> = err_entries.into_iter().filter_map(|s| s).collect();

        write!(
            out,
            "Read {} ok entries and {} err entries in {:?}",
            ok_entries.len(),
            err_entries.len(),
            Instant::now() - start
        )?;

        // collect into calls
        let calls: Calls = ok_entries.into_iter().collect();
        writeln!(out, "Found {} calls", calls.len())?;

        for call in calls.iter() {
            writeln!(out, "{}", call)?;
        }

        // full debug dump
        writeln!(out, "\n\n{:#?}", calls)?;

        Ok(())
    }
}
