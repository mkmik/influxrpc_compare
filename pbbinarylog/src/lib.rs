use bytes::Bytes;

// Bring in the compiled protobuf
include!(concat!(env!("OUT_DIR"), "/grpc.binarylog.v1.rs"));

pub fn decode_log_entry(bytes: Bytes) -> Result<GrpcLogEntry, String> {
    use prost::Message;
    GrpcLogEntry::decode(bytes).map_err(|e| format!("Protobuf error decoding GrpcLogEntry: {}", e))
}

/// Convert the prost timestamp to chrono
use chrono::{DateTime, TimeZone, Utc};
pub fn to_chrono(pb_timestamp: Option<prost_types::Timestamp>) -> DateTime<Utc> {

    let pb_timestamp = pb_timestamp.expect("timestamp not present");

    Utc.timestamp(
        pb_timestamp.seconds,
        pb_timestamp.nanos.try_into().expect("nanos were negative"),
    )
}
