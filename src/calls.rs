use crate::{call::Call, entry::Entry, error::Result};

/// Group `Entries` into logical gRPC calls
///
/// To use:
/// ```
/// let calls: Calls = Entries::try_from(file)?;
/// for call in calls {
///   // do awesome stuff
/// }
/// ```
pub struct Calls {
    /// Calls that are build from the overall records
    calls: Vec<Call>,

    /// Records which could not be handled
    errors: Vec<String>,
}

impl<A: Into<Result<Entry>>> FromIterator<A> for Calls {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        todo!()
    }
}
