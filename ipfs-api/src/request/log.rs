// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;
use serde::ser::{Serialize, Serializer};
use std::borrow::Cow;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LoggingLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl Serialize for LoggingLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            &LoggingLevel::Debug => "debug",
            &LoggingLevel::Info => "info",
            &LoggingLevel::Warning => "warning",
            &LoggingLevel::Error => "error",
            &LoggingLevel::Critical => "critical",
        };

        serializer.serialize_str(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Logger<'a> {
    All,
    Specific(Cow<'a, str>),
}

impl<'a> Serialize for Logger<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            &Logger::All => "*",
            &Logger::Specific(ref logger) => logger.as_ref(),
        };

        serializer.serialize_str(s)
    }
}


#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogLevel<'a> {
    #[serde(rename = "arg")]
    pub logger: Logger<'a>,

    #[serde(rename = "arg")]
    pub level: LoggingLevel,
}

impl<'a> ApiRequest for LogLevel<'a> {
    const path: &'static str = "/log/level";
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LogLs;

impl_skip_serialize!(LogLs);

impl ApiRequest for LogLs {
    const path: &'static str = "/log/ls";
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LogTail;

impl_skip_serialize!(LogTail);

impl ApiRequest for LogTail {
    const path: &'static str = "/log/tail";
}
