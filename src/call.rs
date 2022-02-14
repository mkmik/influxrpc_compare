use chrono::{DateTime, Utc};

/// Represents a logical gRPC call extracted from a chain of Entrys
///
///
#[derive(Default, Debug, Clone)]
pub struct Call {
    id: u64,
    /// source/target
    /// start and end timestamp
    /// headers
    /// gRPR method name
    method_name: Option<String>,

    /// first observed timestamp of this call
    start_time: Option<DateTime<Utc>>,

    /// last observed timestamp of this call
    end_time: Option<DateTime<Utc>>,
}

impl Call {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Note that a timestamp occured as part of this call
    pub fn timestamp(&mut self, timestamp: Option<DateTime<Utc>>) -> &mut Self {
        if let Some(timestamp) = timestamp {
            self.start_time = self
                .start_time
                .take()
                .map(|ts| ts.min(timestamp))
                .or_else(|| Some(timestamp));

            self.end_time = self
                .end_time
                .take()
                .map(|ts| ts.max(timestamp))
                .or_else(|| Some(timestamp));
        }
        self
    }
}
