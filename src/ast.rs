//! This module contains the AST definitions for the RapidRecast language.

use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;

/// RapidModelDefinition represents a single "Cell" of configuration.
/// You can think of it as a single file, or a single request.
/// Even though it has a file version, the file version can be updated - file versions are not permanent.
/// So you can upload file version 1.0.0, then 1.1.0, then update file version 1.0.0.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RapidRecastDefinition<'a> {
    /// The unique identifier of the RapidRecastDefinition.
    pub id: Cow<'a, str>,
    /// The version of the language used to define the file.
    pub language_version: Version,
    /// The version of the RapidRecastDefinition.
    pub file_version: Version,
    /// The name of the RapidRecastDefinition.
    pub name: Option<Cow<'a, str>>,
    /// The description of the RapidRecastDefinition.
    pub description: Option<Cow<'a, str>>,
    /// The AST of the RapidRecastDefinition.
    /// TODO this should be something like a Cow but working for vec
    /// So not Cow::borrow(str) -> ::clone() -> Cow::Owned(String)
    /// But actually Cow::borrow([]Vals) -> ::push() -> Cow::Owned(Vec<Vals>)
    pub ast: Vec<RapidAstStatement>,
}

/// A semantic version, referenced across the RapidRecastDefinition.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Version {
    /// The major version.
    pub major: u64,
    /// The minor version.
    pub minor: u64,
    /// The patch version.
    pub patch: u64,
}

/// A parent element of the language
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum RapidAstStatement {
    /// A Model Definition
    ModelDefinition(RapidModelDefinition),
    /// A Protocol Definition
    ProtocolDefinition(RapidProtocolDefinition),
    /// A Topic Definition
    TopicDefinition(RapidTopicDefinition),
    /// A Cron Definition
    CronDefinition(RapidCronDefinition),
}

/// A Model Definition
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RapidModelDefinition {}

/// A Protocol Definition
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RapidProtocolDefinition {}

/// A Topic Definition
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RapidTopicDefinition {}

/// A Cron Definition
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RapidCronDefinition {}
