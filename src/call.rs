use crate::error::Result;

/// Represents a logical gRPC call extracted from a chain of Entrys
///
///
pub struct Call {
    id: u64,
    /// source/target
    /// start and end timestamp
    /// headers
    /// gRPR method name
    method_name: String,
}

/// Builder for creating [Call]s
pub struct CallBuilder {
    id: u64,
}

impl CallBuilder {
    pub fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn try_build(self) -> Result<Call> {
        let Self { id } = self;

        Ok(Call {
            id,
            method_name: "UNKNOWN".to_string(),
        })
    }
}
