use std::collections::HashMap;
use chrono::NaiveDateTime;
use crate::parsers::{InRelease, InReleaseFile, Signature};

pub struct InReleaseBuilder {
    pub string_parameters: HashMap<String, String>,
    pub signature: Option<Signature>,
    pub acquire_by_hash: Option<bool>,
    pub architectures: Vec<String>,
    pub components: Vec<String>,
    pub date: Option<NaiveDateTime>,
    pub md5_sum_files: Vec<InReleaseFile>,
    pub sha1_files: Vec<InReleaseFile>,
    pub sha256_files: Vec<InReleaseFile>,
    pub sha512_files: Vec<InReleaseFile>,
}

impl InReleaseBuilder {
    pub fn new() -> Self {
        return InReleaseBuilder {
            string_parameters: HashMap::new(),
            signature: Option::None,
            acquire_by_hash: Option::None,
            architectures: Vec::new(),
            components: Vec::new(),
            date: Option::None,
            md5_sum_files: Vec::new(),
            sha1_files: Vec::new(),
            sha256_files: Vec::new(),
            sha512_files: Vec::new(),
        };
    }
}

impl InReleaseBuilder {
    pub fn build(&mut self) -> Result<InRelease, ()> {
        return Result::Err(());
    }
}
