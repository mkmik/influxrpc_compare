use std::{collections::HashMap, time::Duration};

use chrono::{DateTime, Utc};
use pbbinarylog::to_chrono;

/// Native rust version of decoded [pbbinarylog::GrpcLogEntry] to make
/// it easier to work (all types are explicit here, not just in IDEs!)
#[derive(Debug, Clone)]
pub struct Entry {
    timestamp: DateTime<Utc>,
    call_id: u64,
    sequence_id_within_call: u64,
    event_type: EventType,
    logger: Logger,
    payload_truncated: bool,
    // Host address
    peer: String,
    // The contents of this entry
    payload: Payload,
}

impl Entry {
    // TODO make some real timestamps
    pub fn new(inner: pbbinarylog::GrpcLogEntry) -> Self {
        println!("Converting {:#?}\n", inner);

        let event_type = inner.r#type();
        let logger = inner.logger();

        let pbbinarylog::GrpcLogEntry {
            timestamp,
            call_id,
            sequence_id_within_call,
            r#type: _type,
            logger: _logger,
            payload_truncated,
            peer,
            payload,
        } = inner;

        let peer = peer
            .map(|address| format!("{}:{}", address.address, address.ip_port))
            .unwrap_or_else(|| "UNKNOWN".to_string());

        Self {
            timestamp: to_chrono(timestamp),
            call_id,
            sequence_id_within_call,
            event_type: event_type.into(),
            logger: logger.into(),
            payload_truncated,
            peer,
            payload: payload.expect("Missing payload").into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EventType {
    Unknown,
    ClientHeader,
    ServerHeader,
    ClientMessage,
    ServerMessage,
    ClientHalfClose,
    ServerTrailer,
    Cancel,
}

impl From<pbbinarylog::grpc_log_entry::EventType> for EventType {
    fn from(event_type: pbbinarylog::grpc_log_entry::EventType) -> Self {
        match event_type {
            pbbinarylog::grpc_log_entry::EventType::Unknown => EventType::Unknown,
            pbbinarylog::grpc_log_entry::EventType::ClientHeader => EventType::ClientHeader,
            pbbinarylog::grpc_log_entry::EventType::ServerHeader => EventType::ServerHeader,
            pbbinarylog::grpc_log_entry::EventType::ClientMessage => EventType::ClientMessage,
            pbbinarylog::grpc_log_entry::EventType::ServerMessage => EventType::ServerMessage,
            pbbinarylog::grpc_log_entry::EventType::ClientHalfClose => EventType::ClientHalfClose,
            pbbinarylog::grpc_log_entry::EventType::ServerTrailer => EventType::ServerTrailer,
            pbbinarylog::grpc_log_entry::EventType::Cancel => EventType::Cancel,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Logger {
    Unknown,
    Client,
    Server,
}

impl From<pbbinarylog::grpc_log_entry::Logger> for Logger {
    fn from(logger: pbbinarylog::grpc_log_entry::Logger) -> Logger {
        match logger {
            pbbinarylog::grpc_log_entry::Logger::Unknown => Logger::Unknown,
            pbbinarylog::grpc_log_entry::Logger::Client => Logger::Client,
            pbbinarylog::grpc_log_entry::Logger::Server => Logger::Server,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Payload {
    ClientHeader(ClientHeader),
    ServerHeader(ServerHeader),
    Message(Message),
    Trailer(Trailer),
}

impl From<pbbinarylog::grpc_log_entry::Payload> for Payload {
    fn from(payload: pbbinarylog::grpc_log_entry::Payload) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ClientHeader {
    pub metadata: HashMap<String, String>,
    pub method_name: String,
    pub authority: String,
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct ServerHeader {
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub length: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Trailer {
    pub metadata: HashMap<String, String>,
    pub status_code: u32,
    pub status_message: String,
    pub status_details: Vec<u8>,
}
