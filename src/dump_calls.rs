//! Dumps summarized gRPC calls (summarized entries)

use std::{
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

use crate::{
    calls::Calls,
    entries::Entries,
    error::{Error, Result},
    path::LogIterator,
};

pub struct DumpCalls {
    start_path: PathBuf,
}

impl DumpCalls {
    /// Dumps the raw entry contents in all files in the specified path (and its children)
    pub fn new(start_path: impl Into<PathBuf>) -> Self {
        let start_path = start_path.into();

        Self { start_path }
    }

    pub fn process(&mut self) -> Result<Calls> {
        println!(
            "Attempt to process gRPC frames from all .txt files starting at {:?}",
            self.start_path
        );

        let paths = LogIterator::new(self.start_path.clone());
        let mut call_res = Calls::default();
        for path in paths {
            let calls = self.process_path(&path)?;
            call_res.extend_from_other(calls);
        }

        Ok(call_res)
    }

    fn process_path(&self, p: &Path) -> Result<Calls> {
        println!("Processing {:?}", p);

        let entries = match Entries::try_new(p) {
            Ok(entries) => entries,
            Err(e) => return Err(format!("Error reading {:?}: {}", p, e).into()),
        };

        let start = Instant::now();

        // split them into Ok and Errors
        let (ok_entries, err_entries): (Vec<_>, Vec<_>) = entries
            .map(|result| match result {
                Ok(entry) => (Some(entry), None),
                Err(msg) => (None, Some(msg)),
            })
            .unzip();

        let ok_entries: Vec<_> = ok_entries.into_iter().flatten().collect();

        println!(
            "Read {} ok entries and {} err entries in {:?}",
            ok_entries.len(),
            err_entries.into_iter().flatten().count(),
            Instant::now() - start
        );

        // collect into calls
        let calls: Calls = ok_entries.into_iter().collect();
        println!("Found {} calls", calls.len());

        Ok(calls)
    }

    pub fn write_calls_pretty<W>(&self, calls: Calls, out: &mut W) -> Result<()>
    where
        W: Write,
    {
        for call in calls.iter() {
            writeln!(out, "{}", call)?;
        }

        // full debug dump
        writeln!(out, "\n\n***************** DEBUG *************\n\n")?;

        for call in calls.iter() {
            if call
                .status_code
                .as_ref()
                .map(|status_code| *status_code != 0)
                .unwrap_or(false)
            {
                writeln!(out, "Error call: {}", call)?;
            }

            let rpc_method = call
                .method_name
                .as_ref()
                .map(|method_name| method_name != "/influxdata.platform.storage.Storage/Offsets")
                .unwrap_or(false);

            if rpc_method {
                writeln!(out, "Non storage offset call:\n  {}", call)?;
                writeln!(out, "  request: {:?}", call.request)?;
                writeln!(out, "  response: {:?}", call.response)?;
            }
        }

        Ok(())
    }

    pub fn write_calls_binary(&self, calls: Calls, path: &str) -> Result<()> {
        use std::fs::File;
        let contents = File::create(path).map_err(|e| Error::from(e.to_string()))?;
        bincode::serialize_into(contents, &calls).map_err(|e| Error::from(e.to_string()))
    }
}
