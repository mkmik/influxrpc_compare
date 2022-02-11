use bytes::Bytes;

// Bring in the compiled protobuf
include!(concat!(env!("OUT_DIR"), "/grpc.binarylog.v1.rs"));

pub fn decode_log_entry(bytes: Bytes) -> Result<GrpcLogEntry, String> {
    use prost::Message;
    GrpcLogEntry::decode(bytes).map_err(|e| format!("Protobuf error decoding GrpcLogEntry: {}", e))
}

/// Convert the prost timestamp to chrono
use chrono::{DateTime, Duration, TimeZone, Utc};
pub fn to_chrono_timestamp(pb_timestamp: prost_types::Timestamp) -> DateTime<Utc> {
    Utc.timestamp(
        pb_timestamp.seconds,
        pb_timestamp.nanos.try_into().expect("nanos were negative"),
    )
}

/// Convert the prost duraton to chrono
pub fn to_chrono_duration(duration: prost_types::Duration) -> Duration {
    let nanos = (duration.seconds as i64)
        .checked_mul(1_000_000_000)
        .expect("duration seconds overflowed")
        .checked_add(duration.nanos as i64)
        .expect("duration nanos overflowed");

    Duration::nanoseconds(nanos)
}
