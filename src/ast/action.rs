//! Actions available in the AST.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;

/// Actions that one can make on any event
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastAction<'a> {
    AuthBasedAction(AuthBasedAction<'a>),
    LogicBasedAction(LogicBasedAction<'a>),
}

/// Actions that resolve to a logic related change
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum LogicBasedAction<'a> {
    /// If-condition-style blocks
    ConditionBlock {
        condition: ConditionStatement<'a>,
        if_true: Box<RapidRecastAction<'a>>,
        if_false: Option<Box<RapidRecastAction<'a>>>,
    },
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RecastValue<'a> {
    Param(Cow<'a, str>),
    String(Cow<'a, str>),
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
        subject: UserIdentifier<'a>,
        password: Option<Cow<'a, str>>,
    },
    /// Adds non-system metadata that will be accessible to the system for the user
    AddMetadataToUser {
        metadata: BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    },
    /// Adds a policy to the system
    GivePermission {
        subject: UserIdentifier<'a>,
        policy: Vec<Policy>,
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
pub struct Policy {
    /// The subject of the policy
    pub subject: RapidRecastRbacSubject,
    /// The object of the policy
    pub object: RapidRecastRbacObject,
    /// The action of the policy
    pub action: RapidRecastRbacAction,
}

/// The subjects available in RapidRecast RBAC
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastRbacSubject {}

/// The objects available in RapidRecast RBAC
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastRbacObject {}

/// The actions available in RapidRecast RBAC
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum RapidRecastRbacAction {}
