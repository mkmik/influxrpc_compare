// Include the `items` module, which is generated from items.proto.
pub mod binarylog {
    include!(concat!(env!("OUT_DIR"), "/grpc.binarylog.v1.rs"));
}
