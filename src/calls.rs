use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    call::Call,
    entry::{ClientHeader, Entry, EventType, Logger, Message, Payload, ServerHeader, Trailer},
};

/// Group `Entries` into logical gRPC calls
///
/// To use:
/// ```
/// let calls: Calls = Entries::try_from(file)?;
/// for call in calls {
///   // do awesome stuff
/// }
/// ```
#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Calls {
    /// Calls that are build from the overall records
    calls: Vec<Call>,
}

impl Calls {
    pub fn len(&self) -> usize {
        self.calls.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Call> {
        self.calls.iter()
    }

    // appends `other` into a new [`Calls`].
    pub fn extend_from_other(&mut self, other: Self) {
        self.calls.extend(other.calls.into_iter());
    }
}

impl<A: Into<Entry>> FromIterator<A> for Calls {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let builders =
            iter.into_iter()
                .fold(BTreeMap::<u64, Call>::new(), |mut builders, entry| {
                    let entry = entry.into();
                    //println!("Processing Entry: {:?}", entry);
                    let Entry {
                        timestamp,
                        call_id,
                        sequence_id_within_call: _,
                        event_type,
                        logger,
                        payload_truncated,
                        peer,
                        payload,
                    } = entry;

                    assert!(!payload_truncated, "truncated payloads not handled yet");
                    assert!(
                        matches!(logger, Logger::Client),
                        "Only handling client logging now"
                    );

                    let call = builders
                        .entry(call_id)
                        .or_insert_with(|| Call::new(call_id))
                        .with_timestamp(timestamp)
                        .with_peer(peer);

                    match payload {
                        Payload::ClientHeader(client_header) => {
                            let ClientHeader {
                                metadata,
                                method_name,
                                authority,
                                timeout: _,
                            } = client_header;
                            call.with_method_name(method_name)
                                .with_authority(authority)
                                .with_client_headers(metadata)
                        }
                        Payload::ServerHeader(server_header) => {
                            let ServerHeader { metadata } = server_header;

                            call.with_server_headers(metadata)
                        }
                        Payload::Message(message) => {
                            let Message { length, data } = message;
                            assert_eq!(length as usize, data.len(), "mismatched data length");
                            match event_type {
                                EventType::ClientMessage => call.with_request_data(data),
                                EventType::ServerMessage => call.with_response_data(data),
                                _ => panic!("Unexpected payload in event type {:?}", event_type),
                            }
                        }
                        Payload::Trailer(trailer) => {
                            let Trailer {
                                metadata,
                                status_code,
                                status_message,
                                status_details,
                            } = trailer;
                            let status_details =
                                String::from_utf8(status_details).expect("details not string");
                            call.with_status_metadata(metadata)
                                .with_status_code(status_code)
                                .with_status_message(status_message)
                                .with_status_details(status_details)
                        }
                    };

                    //println!("Call after build: {:?}", call);
                    builders
                });

        let calls = builders.into_values().collect();

        Self { calls }
    }
}
