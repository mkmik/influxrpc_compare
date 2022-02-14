use std::{collections::HashMap, fmt::Debug};

use chrono::{DateTime, Duration, Utc};
use pbbinarylog::{to_chrono_duration, to_chrono_timestamp};

/// Native rust version of decoded [pbbinarylog::GrpcLogEntry] to make
/// it easier to work (all types are explicit here, not just in IDEs!)
#[derive(Debug, Clone)]
pub struct Entry {
    pub timestamp: Option<DateTime<Utc>>,
    pub call_id: u64,
    pub sequence_id_within_call: u64,
    pub event_type: EventType,
    pub logger: Logger,
    pub payload_truncated: bool,
    // Host address
    pub peer: Option<String>,
    // The contents of this entry
    pub payload: Payload,
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

#[derive(Debug, Clone)]
pub enum Logger {
    Unknown,
    Client,
    Server,
}

#[derive(Debug, Clone)]
pub enum Payload {
    ClientHeader(ClientHeader),
    ServerHeader(ServerHeader),
    Message(Message),
    Trailer(Trailer),
}

#[derive(Debug, Clone)]
pub struct ClientHeader {
    pub metadata: HashMap<String, String>,
    pub method_name: String,
    /// hostname of the client making the request
    pub authority: String,
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct ServerHeader {
    pub metadata: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Message {
    pub length: u32,
    // raw encoded data (needs to be decoded)
    pub data: Vec<u8>,
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("length", &self.length)
            .field("data", &format!("<..{} bytes..>", self.data.len()))
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Trailer {
    pub metadata: HashMap<String, String>,
    pub status_code: u32,
    pub status_message: String,
    pub status_details: Vec<u8>,
}

impl Entry {
    pub fn new(inner: pbbinarylog::GrpcLogEntry) -> Self {
        //println!("Converting {:#?}\n", inner);

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

        let peer = peer.map(|address| format!("{}:{}", address.address, address.ip_port));

        Self {
            timestamp: timestamp.map(to_chrono_timestamp),
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

// Begin prost conversion goo

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

impl From<pbbinarylog::grpc_log_entry::Logger> for Logger {
    fn from(logger: pbbinarylog::grpc_log_entry::Logger) -> Logger {
        match logger {
            pbbinarylog::grpc_log_entry::Logger::Unknown => Logger::Unknown,
            pbbinarylog::grpc_log_entry::Logger::Client => Logger::Client,
            pbbinarylog::grpc_log_entry::Logger::Server => Logger::Server,
        }
    }
}

impl From<pbbinarylog::grpc_log_entry::Payload> for Payload {
    fn from(payload: pbbinarylog::grpc_log_entry::Payload) -> Self {
        match payload {
            pbbinarylog::grpc_log_entry::Payload::ClientHeader(p) => {
                Payload::ClientHeader(p.into())
            }
            pbbinarylog::grpc_log_entry::Payload::ServerHeader(p) => {
                Payload::ServerHeader(p.into())
            }
            pbbinarylog::grpc_log_entry::Payload::Message(p) => Payload::Message(p.into()),
            pbbinarylog::grpc_log_entry::Payload::Trailer(p) => Payload::Trailer(p.into()),
        }
    }
}

impl From<pbbinarylog::ClientHeader> for ClientHeader {
    fn from(header: pbbinarylog::ClientHeader) -> Self {
        let pbbinarylog::ClientHeader {
            metadata,
            method_name,
            authority,
            timeout,
        } = header;

        ClientHeader {
            metadata: to_hashmap(metadata),
            method_name,
            authority,
            timeout: timeout.map(to_chrono_duration),
        }
    }
}

impl From<pbbinarylog::ServerHeader> for ServerHeader {
    fn from(header: pbbinarylog::ServerHeader) -> Self {
        let pbbinarylog::ServerHeader { metadata } = header;
        ServerHeader {
            metadata: to_hashmap(metadata),
        }
    }
}

impl From<pbbinarylog::Message> for Message {
    fn from(message: pbbinarylog::Message) -> Self {
        let pbbinarylog::Message { length, data } = message;

        Message { length, data }
    }
}

impl From<pbbinarylog::Trailer> for Trailer {
    fn from(trailer: pbbinarylog::Trailer) -> Self {
        let pbbinarylog::Trailer {
            metadata,
            status_code,
            status_message,
            status_details,
        } = trailer;
        Trailer {
            metadata: to_hashmap(metadata),
            status_code,
            status_message,
            status_details,
        }
    }
}

fn to_hashmap(metadata: Option<pbbinarylog::Metadata>) -> HashMap<String, String> {
    metadata
        .map(|metadata| {
            metadata
                .entry
                .into_iter()
                .map(|entry| {
                    (
                        entry.key,
                        String::from_utf8(entry.value).expect("non utf8 header value"),
                    )
                })
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_else(HashMap::new)
}
