//! This module contains the AST definitions for the RapidRecast language.

use std::borrow::Cow;

/// RapidModelDefinition represents a single "Cell" of configuration.
/// You can think of it as a single file, or a single request.
/// Even though it has a version, it can be updated - versions are not permanent.
/// So you can upload 1.0.0, then 1.1.0, then update 1.0.0.
pub struct RapidRecastDefinition<'a> {
    /// The unique identifier of the RapidRecastDefinition.
    pub id: Cow<'a, str>,
    /// The version of the RapidRecastDefinition.
    pub version: Version,
    /// The name of the RapidRecastDefinition.
    pub name: Option<Cow<'a, str>>,
    /// The description of the RapidRecastDefinition.
    pub description: Option<Cow<'a, str>>,
    /// The AST of the RapidRecastDefinition.
    pub ast: Vec<RapidAstStatement>,
}

/// A semantic version, referenced across the RapidRecastDefinition.
pub struct Version {
    /// The major version.
    pub major: u64,
    /// The minor version.
    pub minor: u64,
    /// The patch version.
    pub patch: u64,
}

/// A parent element of the language
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
pub struct RapidModelDefinition {}

/// A Protocol Definition
pub struct RapidProtocolDefinition {}

/// A Topic Definition
pub struct RapidTopicDefinition {}

/// A Cron Definition
pub struct RapidCronDefinition {}
