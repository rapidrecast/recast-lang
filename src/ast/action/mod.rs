//! Actions available in the AST.

use crate::ast::protocol::RapidRecastProtocolType;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// Actions that one can make on any event
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastAction<'a> {
    /// An action that is related to authentication
    AuthBasedAction(AuthBasedAction<'a>),
    /// An action that is based on logic
    LogicBasedAction(LogicBasedAction<'a>),
}

/// Actions that resolve to a logic related change
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum LogicBasedAction<'a> {
    /// If-condition-style blocks
    ConditionBlock {
        /// The condition type associated with the if block
        condition: ConditionStatement<'a>,
        /// The code that is executed if the statement is true
        if_true: Box<RapidRecastAction<'a>>,
        /// THe code that is executed if the statement is false
        if_false: Option<Box<RapidRecastAction<'a>>>,
    },
}

/// A value that can be used in statements
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RecastValue<'a> {
    /// A pre-declared variable reference
    Param(Cow<'a, str>),
    /// A string literal
    String(Cow<'a, str>),
    /// A number literal
    Number(f64),
}

/// A condition statement
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ConditionStatement<'a> {
    /// Left == Right
    Equals(RecastValue<'a>, RecastValue<'a>),
    /// Left != Right
    NotEquals(RecastValue<'a>, RecastValue<'a>),
    /// Left > Right
    GreaterThan(RecastValue<'a>, RecastValue<'a>),
    /// Left < Right
    LessThan(RecastValue<'a>, RecastValue<'a>),
    /// Left >= Right
    GreaterThanOrEqual(RecastValue<'a>, RecastValue<'a>),
    /// Left <= Right
    LessThanOrEqual(RecastValue<'a>, RecastValue<'a>),
    /// Left && Right
    And(RecastValue<'a>, RecastValue<'a>),
    /// Left || Right
    Or(RecastValue<'a>, RecastValue<'a>),
    /// !Value
    Not(RecastValue<'a>),
}

/// Actions that resolve to an authentication related change
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum AuthBasedAction<'a> {
    /// Creates a new user with the specified password
    CreateUser {
        /// The user or role
        subject: UserIdentifier<'a>,
        /// The password for the user
        password: Option<Cow<'a, str>>,
    },
    /// Adds non-system metadata that will be accessible to the system for the user
    AddMetadataToUser {
        /// The user or role
        subject: UserIdentifier<'a>,
        /// Metadata to be added to a subject (user or role)
        metadata: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    },
    /// Adds a policy to the system
    GrantPermissions {
        /// The user or role
        subject: UserIdentifier<'a>,
        /// Policies to be added to a subject (user or role)
        policy: Vec<RapidRecastRbacPolicy<'a>>,
    },
}

/// Convenience struct user data for actions
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct UserIdentifier<'a> {
    /// The namespace the user belongs to
    pub namespace: Cow<'a, str>,
    /// The username within that namespace
    pub username: Cow<'a, str>,
}

/// A way of declaring a RapidRecast RBAC policy
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct RapidRecastRbacPolicy<'a> {
    /// The subject of the policy
    pub subject: RapidRecastRbacSubject<'a>,
    /// The object of the policy
    pub object: RapidRecastRbacObject<'a>,
    /// The action of the policy
    pub action: RapidRecastRbacAction,
}

/// The subjects available in RapidRecast RBAC
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastRbacSubject<'a> {
    /// The system administrator
    Admin,
    /// An anonymous user
    Anon,
    /// A specific user on the given namespace
    /// (namespace, user)
    UserOrRole(Cow<'a, str>, Cow<'a, str>),
}

/// The objects available in RapidRecast RBAC
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastRbacObject<'a> {
    /// A Namespace within RapidRecast
    Namespace(NamespaceObject<'a>),
    /// A Protocol provided by within RapidRecast
    Protocol(RapidRecastProtocolType),
    /// A Topic within RapidRecast
    Topic(TopicObject<'a>),
    /// A client available to RapidRecast
    Client(ClientObject),
    /// A RapidRecast Definition Language object
    Model(ModelObject),
}

/// A Namespace within RapidRecast
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum NamespaceObject<'a> {
    /// A namespace that exists
    ExistingNamespace(Cow<'a, str>),
    /// A namespace that does not exist
    NonExistingNamespace(Cow<'a, str>),
}

/// A topic within RapidRecast
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TopicObject<'a> {
    /// A topic that does not exist
    NonExistingTopic(Cow<'a, str>),
    /// A topic that exists
    NamespaceTopic(Cow<'a, str>, Cow<'a, str>),
}

/// The clients available to the system. Cached or otherwise to-be created.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ClientObject {
    /// An http 1 client
    Http1,
    /// An http 2 client
    Http2,
    /// An http 3 client
    Http3,
    /// A kafka client
    Kafka,
    /// An mqtt client
    Mqtt,
}

/// A reference to a RapidRecast Definition Language object
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ModelObject {}

/// The actions available in RapidRecast RBAC
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastRbacAction {
    /// Allows the creation of a resource
    Create,
    /// Allows reading from the resource
    Read,
    /// Allows updating the resource (metadata)
    Update,
    /// Allows deleting the resource
    Delete,
    /// Allows writing to the resource
    Write,
    /// Allows listing the resource (reading metadata)
    List,
    /// Allows renaming the resource
    Rename,
}
