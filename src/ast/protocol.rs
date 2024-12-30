//! Protocol Definition related AST
use crate::ast::action::RapidRecastAction;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A Protocol Definition
/// The type of protocol used for the protocol definition
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidProtocolDefinition<'a> {
    /// Indicates that we are modifying an HTTP protocol
    HttpProtocolDefinition(HttpStatement<'a>),
}

/// Allows for specifying parts of an HTTP statement
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct HttpStatement<'a> {
    /// Protocol sequence, where the order is determined from CLI order
    /// ex. `rapidrecast --http host1:123,host1:234,host2:345`
    /// That declares 3 protocols, so `sequence=2` means binding `host2:345`
    pub sequence: u8,
    /// The paths to match on the HTTP protocol
    pub paths: Vec<Cow<'a, str>>,
    /// The methods of access for this rule. GET, POST, DELETE, UPDATE
    pub methods: Vec<RapidRecastHttpMethod>,
    /// Actions that take effect once the protocol is triggered
    pub actions: Vec<RapidRecastAction<'a>>,
}

/// Http Methods supported by RapidRecast
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastHttpMethod {
    /// Get Method
    GET,
    /// Post Method
    POST,
    /// Delete Method
    DELETE,
    /// Update Method
    UPDATE,
    /// Patch Method
    PATCH,
    /// Put Method
    PUT,
    /// Options Method
    OPTIONS,
    /// Head Method
    HEAD,
    /// Connect Method
    CONNECT,
    /// Trace Method
    TRACE,
}

/// The protocols available in RapidRecast
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastProtocolType {
    /// HTTP 1,2, and 3 protocols
    HTTP,
    /// WebSockets on HTTP 1, 2, and 3 - it is a different model of communication from HTTP 2/3 streams
    WebSocket,
    /// Kafka protocol
    Kafka,
    /// RabbitMQ protocol
    RabbitMQ,
    /// gRPC protocol
    Grpc,
}
