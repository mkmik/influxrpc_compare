use std::collections::BTreeMap;

use crate::{
    call::Call,
    entry::{Entry, Logger, Payload},
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
#[derive(Debug, Clone)]
pub struct Calls {
    /// Calls that are build from the overall records
    calls: Vec<Call>,
}

impl Calls {
    pub fn len(&self) -> usize {
        self.calls.len()
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
                        event_type: _,
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

                    let call = match payload {
                        Payload::ClientHeader(client_headers) => call
                            .with_method_name(client_headers.method_name)
                            .with_client_headers(client_headers.metadata),
                        Payload::ServerHeader(server_headers) => {
                            call.with_server_headers(server_headers.metadata)
                        }
                        Payload::Message(_msg) => call,
                        Payload::Trailer(_trailer) => call,
                    };

                    //println!("Call after build: {:?}", call);
                    builders
                });

        let calls = builders.into_values().collect();

        Self { calls }
    }
}
