use std::collections::HashMap;

use chrono::{NaiveDateTime, ParseError};
use thiserror::Error;

use crate::parsers::in_release::in_release_builder::InReleaseBuilder;
use crate::parsers::in_release::in_release_states::{ParserState, State};

#[derive(Error, Debug)]
pub enum HeaderStateError {
    #[error("Unknown header key: {0}")]
    UnknownHeaderKey(String),
    #[error("Unable to parse date: {0}")]
    DateParsingError(#[source] ParseError, String),
    #[error("Unexpected header format: {0}")]
    UnexpectedHeaderFormat(String),
}

pub struct HeaderState {
    parameters: HashMap<String, String>,
    date: Option<NaiveDateTime>,
    architectures: Vec<String>,
    components: Vec<String>,
}

impl HeaderState {
    pub fn new() -> Self {
        return HeaderState {
            parameters: HashMap::new(),
            date: Option::None,
            architectures: Vec::new(),
            components: Vec::new(),
        };
    }
}

impl HeaderState {
    fn match_header_key(
        &mut self,
        header_key: &str,
        header_value: &str,
    ) -> Result<(), HeaderStateError> {
        match header_key {
            "Origin" => {
                self.parameters
                    .insert("origin".to_string(), header_value.to_string());
            }
            "Label" => {
                self.parameters
                    .insert("label".to_string(), header_value.to_string());
            }
            "Suite" => {
                self.parameters
                    .insert("suite".to_string(), header_value.to_string());
            }
            "Version" => {
                self.parameters
                    .insert("version".to_string(), header_value.to_string());
            }
            "Codename" => {
                self.parameters
                    .insert("codename".to_string(), header_value.to_string());
            }
            "Date" => {
                self.date = Option::Some(
                    NaiveDateTime::parse_from_str(header_value, "%d %b %Y %H:%M:%S UTC").map_err(
                        |e| HeaderStateError::DateParsingError(e, header_key.to_string()),
                    )?,
                );
            }
            "Architectures" => {
                let architectures_iter = header_value
                    .split(' ')
                    .map(|x| x.trim())
                    .map(|x| x.to_string());

                self.architectures.extend(architectures_iter);
            }
            "Components" => {
                let components_iter = header_value
                    .split(' ')
                    .map(|x| x.trim())
                    .map(|x| x.to_string());

                self.components.extend(components_iter);
            }
            "Description" => {
                self.parameters
                    .insert("description".to_string(), header_value.to_string());
            }
            _ => {
                // No matching header found
                return Result::Err(HeaderStateError::UnknownHeaderKey(header_key.to_string()));
            }
        }

        return Result::Ok(());
    }
}

impl ParserState for HeaderState {
    fn parse_line(&mut self, line: &str) -> Result<(), HeaderStateError> {
        let mut header_key: Option<&str> = Option::None;
        let mut header_value: Option<&str> = Option::None;

        for part in line.split(':') {
            if header_key.is_none() {
                header_key = Option::Some(part.trim());
            } else if header_value.is_none() {
                header_value = Option::Some(part.trim());
            } else {
                // Unexpected split size
                return Result::Err(HeaderStateError::UnexpectedHeaderFormat(line.to_string()));
            }
        }

        if header_key.is_none() && header_value.is_none() {
            return Result::Ok(());
        }

        if header_key.is_none() || header_value.is_none() {
            // Not all values were set
            return Result::Err(HeaderStateError::UnexpectedHeaderFormat(line.to_string()));
        }

        self.match_header_key(header_key.unwrap(), header_value.unwrap())?;

        return Result::Ok(());
    }

    fn is_my_state(&self, line: &str) -> bool {
        return line.contains("BEGIN PGP SIGNED MESSAGE");
    }

    fn get_state(&self) -> State {
        return State::Header;
    }

    fn dump_data(&mut self, target: &mut InReleaseBuilder) {
        // TODO: Dump date
        target.architectures.append(&mut self.architectures);
        target.components.append(&mut self.components);
        target.string_parameters.extend(self.parameters.drain());
        target.date = self.date;
    }
}
