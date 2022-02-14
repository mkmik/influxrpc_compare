mod influxrpc;


/// All the GRPC methods this code knows how to decode to native form
#[derive(Debug, Clone)]
pub enum Method {

    /// a gRPC Method we don't (yet) know how to decode
    Unknown{
        /// The name of the gRPC method
        method_name: String,
        /// The raw data of the
        data: Vec<u8>,
    }
}



impl Method {
    pub fn new(method_name: impl Into<String>, data: Vec<u8>) -> Self {
        let method_name = method_name.into();

        println!("Attempting to decode {} from {} bytes", method_name, data.len());

        // TODO handle method names specially

        // fallback to Unknown

        Self::Unknown {
            method_name,
            data
        }
    }
}
