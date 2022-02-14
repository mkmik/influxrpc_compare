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

impl <A: Into<Entry>> FromIterator<A> for Calls {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let calls = Vec::new();
        Self { calls }
    }
}
