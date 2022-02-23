use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::methods::{Method, MethodType};

/// Represents a logical gRPC call extracted from a chain of Entrys
///
///
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Call {
    pub id: u64,

    /// gRPR method name
    pub method_name: Option<String>,

    /// decoded gRPC request message (depends on method_name)
    pub request: Option<Method>,

    /// decoded gRPC response message (depends on method_name)
    pub response: Option<Method>,

    /// first observed timestamp of this call
    pub start_time: Option<DateTime<Utc>>,

    /// last observed timestamp of this call
    pub end_time: Option<DateTime<Utc>>,

    /// Other end of the request
    pub peer: Option<String>,

    /// authority (client dns name)
    pub authority: Option<String>,

    /// Headers sent from Client
    pub client_headers: HashMap<String, String>,

    /// Headers sent from Server
    pub server_headers: HashMap<String, String>,

    /// Response code
    pub status_code: Option<u32>,

    /// Response message
    pub status_message: Option<String>,

    /// Response details (decoded as string)
    pub status_details: Option<String>,

    /// Trailer metadata
    pub status_metadata: HashMap<String, String>,
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call(id={:6}) [", self.id)?;

        if let Some(start_time) = &self.start_time {
            write!(f, "{}-", start_time)?;
        } else {
            write!(f, "-")?;
        };

        if let Some(end_time) = &self.end_time {
            write!(f, "{}", end_time)?;
        } else {
            write!(f, "??")?;
        };

        write!(f, "]")?;

        if let Some(method_name) = &self.method_name {
            write!(f, " {}", method_name)?;
        }

        write!(
            f,
            " {} --> {}",
            self.authority.as_deref().unwrap_or("<UNKNOWN>"),
            self.peer.as_deref().unwrap_or("<UNKNOWN>")
        )?;

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
                .or(Some(timestamp));

            self.end_time = self
                .end_time
                .take()
                .map(|ts| ts.max(timestamp))
                .or(Some(timestamp));
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
                .or(Some(peer));
        }

        self
    }

    pub fn with_method_name(&mut self, method_name: String) -> &mut Self {
        assert!(
            self.method_name.is_none(),
            "Already have method name: {:?}",
            self.method_name
        );
        self.method_name = Some(method_name);
        self
    }

    pub fn with_request_data(&mut self, method_data: Vec<u8>) -> &mut Self {
        assert!(
            self.request.is_none(),
            "Already have request: {:?}",
            self.request
        );

        let method = if let Some(method_name) = &self.method_name {
            Method::new(method_name, method_data, MethodType::Request)
        } else {
            // could be smarter here and postpone decoding if method_name hasn't been seen yet
            panic!("Got method data before method_name, so don't know how to decode it");
        };

        self.request = Some(method);
        self
    }

    pub fn with_response_data(&mut self, method_data: Vec<u8>) -> &mut Self {
        assert!(
            self.response.is_none(),
            "Already have response: {:?}",
            self.response
        );

        let method = if let Some(method_name) = &self.method_name {
            Method::new(method_name, method_data, MethodType::Response)
        } else {
            // could be smarter here and postpone decoding if method_name hasn't been seen yet
            panic!("Got method data before method_name, so don't know how to decode it");
        };

        self.response = Some(method);
        self
    }

    pub fn with_authority(&mut self, authority: String) -> &mut Self {
        assert!(
            self.authority.is_none(),
            "Already have authority name: {:?}",
            self.authority
        );
        self.authority = Some(authority);
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
