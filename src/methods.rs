use bytes::Bytes;
use generated_types::influxdata::platform::storage::{
    CapabilitiesResponse, OffsetsResponse, ReadResponse, ReadWindowAggregateRequest,
    StringValuesResponse, TagKeysRequest, TagValuesRequest,
};

#[derive(Debug, Clone, Copy)]
pub enum MethodType {
    Request,
    Response,
}

/// All the GRPC methods this code knows how to decode to native form
#[derive(Debug, Clone)]
pub enum Method {
    /// `/influxdata.platform.storage.Storage/Offsets`
    /// No special decoding (yet)
    StorageOffsetsRequest(Bytes),
    StorageOffsetsResponse(OffsetsResponse),

    /// `/influxdata.platform.storage.Storage/TagKeys`
    TagKeysRequest(TagKeysRequest),

    /// /influxdata.platform.storage.Storage/TagValues
    TagValuesRequest(TagValuesRequest),

    /// Request: `/influxdata.platform.storage.Storage/Capabilities`
    CapabilitiesRequest(),
    /// Response: `/influxdata.platform.storage.Storage/Capabilities`
    CapabilitiesResponse(CapabilitiesResponse),

    /// Request `/influxdata.platform.storage.Storage/ReadWindowAggregate`
    ReadWindowAggregateRequest(ReadWindowAggregateRequest),

    /// Response for:
    /// `/influxdata.platform.storage.Storage/ReadWindowAggregate`
    ReadResponse(ReadResponse),

    /// Response for:
    /// * `/influxdata.platform.storage.Storage/TagKeys`
    /// * `/influxdata.platform.storage.Storage/TagValues`
    StringValuesResponse(StringValuesResponse),

    /// a gRPC Method we don't (yet) know how to decode
    Unknown {
        /// The name of the gRPC method
        method_name: String,
        /// The raw data that went in
        bytes: Bytes,
    },
}

impl Method {
    pub fn new(method_name: impl Into<String>, data: Vec<u8>, method_type: MethodType) -> Self {
        use prost::Message;
        use MethodType::*;
        let method_name = method_name.into();
        let bytes: Bytes = data.into();

        match (method_name.as_str(), method_type) {
            ("/influxdata.platform.storage.Storage/Offsets", Request) => {
                Self::StorageOffsetsRequest(bytes)
            }
            ("/influxdata.platform.storage.Storage/Offsets", Response) => {
                let msg = OffsetsResponse::decode(bytes).expect("Error decoding OffsetsResponse");
                Self::StorageOffsetsResponse(msg)
            }
            ("/influxdata.platform.storage.Storage/Capabilities", Request) => {
                assert!(
                    bytes.is_empty(),
                    "Unexpected request payload for storage/capabilities"
                );
                Self::CapabilitiesRequest()
            }
            ("/influxdata.platform.storage.Storage/Capabilities", Response) => {
                let msg = CapabilitiesResponse::decode(bytes)
                    .expect("Error decoding CapabilitiesResponse");
                Self::CapabilitiesResponse(msg)
            }
            ("/influxdata.platform.storage.Storage/TagKeys", Request) => {
                let msg = TagKeysRequest::decode(bytes).expect("Error decoding TagKeysRequest");
                Self::TagKeysRequest(msg)
            }
            ("/influxdata.platform.storage.Storage/TagValues", Request) => {
                let msg = TagValuesRequest::decode(bytes).expect("Error decoding TagValuesRequest");
                Self::TagValuesRequest(msg)
            }
            ("/influxdata.platform.storage.Storage/TagKeys", Response)
            | ("/influxdata.platform.storage.Storage/TagValues", Response) => {
                let msg = StringValuesResponse::decode(bytes)
                    .expect("Error decoding StringValuesResponse");
                Self::StringValuesResponse(msg)
            }
            ("/influxdata.platform.storage.Storage/ReadWindowAggregate", Request) => {
                let msg = ReadWindowAggregateRequest::decode(bytes)
                    .expect("Error decoding ReadWindowAggregateRequest");
                Self::ReadWindowAggregateRequest(msg)
            }
            ("/influxdata.platform.storage.Storage/ReadWindowAggregate", Response) => {
                let msg = ReadResponse::decode(bytes).expect("Error decoding ReadResponse");
                Self::ReadResponse(msg)
            }

            _ => {
                // fallback to unknown
                println!(
                    "Unknown how to decode {} from {} bytes",
                    method_name,
                    bytes.len()
                );
                Self::Unknown { method_name, bytes }
            }
        }
    }
}
