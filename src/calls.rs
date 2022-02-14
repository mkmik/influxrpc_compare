use std::collections::BTreeMap;

use crate::{call::Call, entry::{Entry, Logger}};

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
                println!("Processing Entry: {:?}", entry);
                let Entry {
                    timestamp,
                    call_id,
                    sequence_id_within_call,
                    event_type,
                    logger,
                    payload_truncated,
                    peer,
                    payload,
                } = entry;

                assert!(!payload_truncated, "truncated payloads not handled yet");
                assert!(matches!(logger, Logger::Client), "Only handling client logging now");

                let call = builders
                    .entry(call_id)
                    .or_insert_with(|| Call::new(call_id))
                    .timestamp(timestamp)
                    .peer(peer)
                    ;

                println!("Call after build: {:?}", call);

                // };

                builders
            });

        let calls = builders.into_values().collect();

        Self { calls }
    }
}
