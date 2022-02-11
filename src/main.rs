mod dump;
mod entries;
mod entry;
mod error;
mod path;

use std::{io::stdout, path::PathBuf};

use clap::Parser;

/// Command line program for working with binary gRPC [logs] that
/// contain requests / responses in the influxdb storage gRPC format.
/// These files are typically named something like grpcgo_binarylog_2709101216.txt
///
/// # Example (dump contents of logs in a directory):
///
/// influxrpc_compare dump --path  /path/to/dumps
///
/// # Reference
///
/// [logs]: https://github.com/grpc/proposal/blob/master/A16-binary-logging.md
/// [protobuf]: https://github.com/grpc/grpc-proto/blob/master/grpc/binlog/v1/binarylog.proto
#[derive(Parser, Debug)]
#[clap(author, version, about)]
enum InfluxRpcCompare {
    Dump(Dump),
}

#[derive(Parser, Debug)]
struct Dump {
    #[clap(long, parse(from_os_str))]
    /// Search path for grpc log files
    path: PathBuf,
}

fn main() {
    let args = InfluxRpcCompare::parse();

    match args {
        InfluxRpcCompare::Dump(dump) => {
            dump::Dump::new(dump.path)
                .dump(&mut stdout())
                .expect("Error dumping");
        }
    };

    println!("Done");
}
