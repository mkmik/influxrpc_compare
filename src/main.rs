mod call;
mod calls;
mod dump_calls;
mod dump_entries;
mod entries;
mod entry;
mod error;
mod methods;
mod path;

use std::{io::stdout, path::PathBuf, str::FromStr};

use clap::Parser;

/// Command line program for working with binary gRPC [logs] that
/// contain requests / responses in the influxdb storage gRPC format.
/// These files are typically named something like grpcgo_binarylog_2709101216.txt
///
/// # Example (dump raw log entries in all .txt files found in):
/// influxrpc_compare dump-entries --path  /path/to/dumps
///
/// # Example (dump reconstructed calls from logs in all .txt files found in):
/// influxrpc_compare dump-calls --path  /path/to/dumps
///
/// # Reference
///
/// [logs]: https://github.com/grpc/proposal/blob/master/A16-binary-logging.md
/// [protobuf]: https://github.com/grpc/grpc-proto/blob/master/grpc/binlog/v1/binarylog.proto
#[derive(Parser, Debug)]
#[clap(author, version, about)]
enum InfluxRpcCompare {
    /// Dump raw log entry files
    DumpEntries(DumpEntries),
    /// Dump gRPC calls (reconstructed from log entry files)
    DumpCalls(DumpCalls),
}

#[derive(Parser, Debug)]
struct DumpEntries {
    #[clap(long, parse(from_os_str))]
    /// Search path for grpc log files
    path: PathBuf,
}

#[derive(Parser, Debug)]
struct DumpCalls {
    #[clap(long = "in", parse(from_os_str))]
    /// Search path for grpc log files
    input_path: PathBuf,

    #[clap(long = "out", parse(from_os_str))]
    /// optional output path for binary formatted Calls
    output_path: Option<PathBuf>,

    #[clap(long)]
    /// Format to emit processed gRPC calls
    format: CallFormat,

    #[clap(long, default_value = "")]
    /// optional filter on org_id
    org_filter: String,
}

#[derive(Debug)]
enum CallFormat {
    Pretty,
    Binary,
}

impl FromStr for CallFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pretty" => Ok(Self::Pretty),
            "bin" => Ok(Self::Binary),
            _ => Err("supported formats: {pretty, bin}".to_string()),
        }
    }
}

fn main() {
    let args = InfluxRpcCompare::parse();

    match args {
        InfluxRpcCompare::DumpEntries(dump) => {
            dump_entries::DumpEntries::new(dump.path)
                .dump(&mut stdout())
                .expect("Error dumping entries");
        }
        InfluxRpcCompare::DumpCalls(dump) => {
            if matches!(dump.format, CallFormat::Binary) && dump.output_path.is_none() {
                eprintln!("output path required");
                return;
            }

            let mut dc = dump_calls::DumpCalls::new(dump.input_path);
            let mut calls = match dc.process() {
                Ok(calls) => calls,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            // Filter offset calls out
            calls.filter_offset_calls();
            println!("Filtered offset calls. {:?} calls remaining", calls.len());

            // Filter by org_id
            if !dump.org_filter.is_empty() {
                let org_filter = dump.org_filter.as_str();
                calls.filter_by_org_id(org_filter);
                println!(
                    "Filtered calls not for org id {}. {:?} calls remaining",
                    org_filter,
                    calls.len()
                );
            }

            let res = match dump.format {
                CallFormat::Pretty => dc.write_calls_pretty(calls, &mut stdout()),
                CallFormat::Binary => dc.write_calls_binary(
                    calls,
                    dump.output_path
                        .unwrap()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .as_str(),
                ),
            };

            match res {
                Ok(_) => println!("Completed successfully"),
                Err(e) => eprintln!("{}", e),
            }
        }
    };
}
