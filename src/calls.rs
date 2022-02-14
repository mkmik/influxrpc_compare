use std::collections::BTreeMap;

use crate::{call::Call, entry::Entry};

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
                    let Entry {
                        timestamp,
                        call_id,
                        sequence_id_within_call,
                        event_type,
                        logger,
                        payload_truncated,
                        peer,
                        payload,
                    } = entry.into();

                    assert!(!payload_truncated, "truncated payloads not handled yet");

                    let builder = builders
                        .entry(call_id)
                        .or_insert_with(|| Call::new(call_id))
                        .timestamp(timestamp);

                    // };

                    builders
                });

        let calls = builders.into_values().collect();

        Self { calls }
    }
}
