use std::path::Path;

use bytes::Bytes;

use crate::error::Result;

/// module for decoding GrpcLogEntry from files
///
/// To use:
/// ```
/// let entries = Entries::try_from(file)?;
/// for entry in entries {
///   // Do some awesome stuff
/// }
/// ```
pub struct Entries {
    records: LengthDelimitedRecords,
}

impl Entries {
    pub fn try_new(p: impl AsRef<Path>) -> Result<Self> {
        // attempt to read the file
        let contents = std::fs::read(p.as_ref())?;

        let bytes: Bytes = contents.into();
        println!("Read {} bytes from {:?}", bytes.len(), p.as_ref());

        Ok(Self {
            records: LengthDelimitedRecords::new(bytes),
        })
    }
}

impl Iterator for Entries {
    type Item = Result<pbbinarylog::GrpcLogEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.records.next().map(|bytes_result| {
            bytes_result
                .and_then(|bytes| pbbinarylog::decode_log_entry(bytes).map_err(|e| e.into()))
        })
    }
}

/// This thing iterates through a set of recordes with the structure
///
/// ```text
/// (i32 length)(... data ...)
/// (i32 length)(... data ...)
/// (i32 length)(... data ...)
/// ```
struct LengthDelimitedRecords {
    bytes: Bytes,
    current_offset: Option<usize>,
}

impl LengthDelimitedRecords {
    pub fn new(bytes: Bytes) -> Self {
        Self {
            bytes,
            current_offset: Some(0),
        }
    }
}

impl Iterator for LengthDelimitedRecords {
    type Item = Result<Bytes>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_offset = if let Some(c) = self.current_offset.take() {
            c
        } else {
            return None;
        };

        // try and read the next 4 bytes as a length
        let remain = self.bytes.len() - current_offset;
        //println!("current_offset is {}, {} bytes remain", current_offset, remain);

        if remain == 0 {
            // no more records!
            return None;
        }

        let len_bytes =
            if let Ok(len_bytes) = self.bytes[current_offset..current_offset + 4].try_into() {
                len_bytes
            } else {
                return Some(Err(format!(
                    "Can not read next length from offset {}, only {} bytes remain",
                    current_offset, remain
                )
                .into()));
            };

        // account for the length field
        let current_offset = current_offset + 4;

        let record_len = i32::from_be_bytes(len_bytes) as usize;

        if record_len > remain {
            return Some(Err(format!(
                "Reported record length of {} is larger than {} remaining bytes",
                record_len, remain
            )
            .into()));
        }

        let next_offset = current_offset + record_len;
        let record = self.bytes.slice(current_offset..next_offset);
        self.current_offset = Some(next_offset);
        //println!("Next offset: {:?}", self.current_offset);
        Some(Ok(record))
    }
}
