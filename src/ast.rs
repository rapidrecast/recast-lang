//! This module contains the AST definitions for the RapidRecast language.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

/// RapidModelDefinition represents a single "Cell" of configuration.
/// You can think of it as a single file, or a single request.
/// Even though it has a file version, the file version can be updated - file versions are not permanent.
/// So you can upload file version 1.0.0, then 1.1.0, then update file version 1.0.0.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
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
#[derive(PartialEq, Debug, Clone)]
pub struct Version {
    /// The major version.
    pub major: u64,
    /// The minor version.
    pub minor: u64,
    /// The patch version.
    pub patch: u64,
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split('.');

        let major = parts.next().ok_or_else(|| serde::de::Error::custom("Missing major version"))?.parse::<u64>().map_err(|_| serde::de::Error::custom("Invalid major version"))?;
        let minor = parts.next().ok_or_else(|| serde::de::Error::custom("Missing minor version"))?.parse::<u64>().map_err(|_| serde::de::Error::custom("Invalid minor version"))?;
        let patch = parts.next().ok_or_else(|| serde::de::Error::custom("Missing patch version"))?.parse::<u64>().map_err(|_| serde::de::Error::custom("Invalid patch version"))?;

        Ok(Version { major, minor, patch })
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}.{}.{}", self.major, self.minor, self.patch))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A parent element of the language
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
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
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct RapidModelDefinition {}

/// A Protocol Definition
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct RapidProtocolDefinition {
    /// The type of protocol used for the protocol definition
    pub protocol: ProtocolType,
    /// Protocol sequence, where the order is determined from CLI order
    /// ex. `rapidrecast --http host1:123,host1:234,host2:345`
    /// That declares 3 protocols, so `sequence=2` means binding `host2:345`
    pub sequence: u8,
}

/// The protocols available in RapidRecast
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ProtocolType {
    /// HTTP 1,2, and 3 protocols
    HTTP,
    /// WebSockets on HTTP 1, 2, and 3 - it is a different model of communication from HTTP 2/3 streams
    WebSocket,
    /// Kafka protocol
    Kafka,
    /// RabbitMQ protocol
    RabbitMQ,
}

/// A Topic Definition
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct RapidTopicDefinition {}

/// A Cron Definition
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct RapidCronDefinition {}
