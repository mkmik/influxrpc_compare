//! Dumps grpc logs somewhere

use std::fmt;

pub struct Dump{}

impl Dump {
    pub fn new() -> Self {
        Self {}
    }
}

impl std::fmt::Display for Dump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "dumping!")
    }
}
