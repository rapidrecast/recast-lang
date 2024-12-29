use crate::ast::{
    ProtocolType, RapidAstStatement, RapidProtocolDefinition, RapidRecastDefinition, Version,
};
use std::borrow::Cow;

pub fn bare_minimum_schema() -> RapidRecastDefinition<'static> {
    RapidRecastDefinition {
        id: Cow::Borrowed("unique-schema-id-123"),
        language_version: Version {
            major: 1,
            minor: 2,
            patch: 3,
        },
        file_version: Version {
            major: 4,
            minor: 5,
            patch: 6,
        },
        name: Some(Cow::Borrowed("some name")),
        description: Some(Cow::Borrowed("some description")),
        ast: vec![RapidAstStatement::ProtocolDefinition(
            RapidProtocolDefinition {
                protocol: ProtocolType::HTTP,
                sequence: 0,
            },
        )],
    }
}
