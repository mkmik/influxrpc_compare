use std::{collections::HashMap, fmt::Display};

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

    /// Other end of the request
    peer: Option<String>,

    /// Headers sent from Client
    client_headers: HashMap<String, String>,

    /// Headers sent from Server
    server_headers: HashMap<String, String>,

    /// Response code
    status_code: Option<u32>,

    /// Response message
    status_message: Option<String>,

    /// Response details (decoded as string)
    status_details: Option<String>,

    /// Trailer metadata
    status_metadata: HashMap<String, String>,
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call(id={:6}) [", self.id)?;

        if let Some(start_time) = &self.start_time {
            write!(f, "{}-", start_time)?;
        } else {
            write!(f, "{}-", "")?;
        };

        if let Some(end_time) = &self.end_time {
            write!(f, "{}]", end_time)?;
        } else {
            write!(f, "{}]]", "??")?;
        };

        write!(f, "]")?;

        if let Some(method_name) = &self.method_name {
            write!(f, " {}", method_name)?;
        }

        Ok(())
    }
}

impl Call {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Note that a timestamp occured as part of this call
    pub fn with_timestamp(&mut self, timestamp: Option<DateTime<Utc>>) -> &mut Self {
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

    pub fn with_peer(&mut self, peer: Option<String>) -> &mut Self {
        if let Some(peer) = peer {
            self.peer = self
                .peer
                .take()
                .map(|existing_peer| {
                    assert_eq!(existing_peer, peer, "Unexpectedly different peer in log");
                    existing_peer
                })
                .or_else(|| Some(peer));
        }

        self
    }

    pub fn with_method_name(&mut self, method_name: String) -> &mut Self {
        self.method_name = self
            .method_name
            .take()
            .map(|existing_method_name| {
                assert_eq!(
                    existing_method_name, method_name,
                    "Unexpectedly different method_name in log"
                );
                existing_method_name
            })
            .or_else(|| Some(method_name));

        self
    }

    pub fn with_client_headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        for (name, value) in headers {
            let existing_header = self.client_headers.insert(name, value);
            assert!(
                existing_header.is_none(),
                "duplicated client header: {:?}",
                existing_header
            );
        }
        self
    }

    pub fn with_server_headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        for (name, value) in headers {
            let existing_header = self.server_headers.insert(name, value);
            assert!(
                existing_header.is_none(),
                "duplicated server header: {:?}",
                existing_header
            );
        }
        self
    }

    pub fn with_status_metadata(&mut self, metadata: HashMap<String, String>) -> &mut Self {
        for (name, value) in metadata {
            let existing_header = self.status_metadata.insert(name, value);
            assert!(
                existing_header.is_none(),
                "duplicated status metadata header: {:?}",
                existing_header
            );
        }
        self
    }

    pub fn with_status_code(&mut self, status_code: u32) -> &mut Self {
        assert!(
            self.status_code.is_none(),
            "Already have status code: {:?}",
            self.status_code
        );
        self.status_code = Some(status_code);
        self
    }

    pub fn with_status_message(&mut self, status_message: String) -> &mut Self {
        assert!(
            self.status_message.is_none(),
            "Already have status message: {:?}",
            self.status_message
        );
        self.status_message = Some(status_message);
        self
    }

    pub fn with_status_details(&mut self, status_details: impl Into<String>) -> &mut Self {
        let status_details = status_details.into();
        assert!(
            self.status_details.is_none(),
            "Already have status details: {:?}",
            self.status_details
        );
        self.status_details = Some(status_details);
        self
    }
}
