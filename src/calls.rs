use std::collections::BTreeMap;

use crate::{call::{Call, CallBuilder}, entry::Entry};

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

impl <A: Into<Entry>> FromIterator<A> for Calls {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {

        let builders = iter
            .into_iter()
            .fold(BTreeMap::<u64, CallBuilder>::new(), |mut builders, entry| {
                let entry = entry.into();
                let builder = builders.entry(entry.call_id)
                    .or_insert_with(|| CallBuilder::new(entry.call_id));

                // update based on the type of entry
                // match entry {

                // };

                builders
            });

        let calls = builders.into_values().map(|b| b.build()).collect();

        Self {
            calls
        }
    }
}
