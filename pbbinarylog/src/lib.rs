use bytes::Bytes;

// Bring in the compiled protobuf
include!(concat!(env!("OUT_DIR"), "/grpc.binarylog.v1.rs"));

pub fn decode_log_entry(bytes: Bytes) -> Result<GrpcLogEntry, String> {
    use prost::Message;
    GrpcLogEntry::decode(bytes).map_err(|e| format!("Protobuf error decoding GrpcLogEntry: {}", e))
}
