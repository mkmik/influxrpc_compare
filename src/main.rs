mod dump;
mod error;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
enum InfluxRpcCompare {
    Dump(Dump),
}

#[derive(Parser, Debug)]
struct Dump {
}

fn main() {
    let args = InfluxRpcCompare::parse();

    match args {
        InfluxRpcCompare::Dump(dump) => println!("{}", dump::Dump::new())
    };


    println!("Done");
 }
