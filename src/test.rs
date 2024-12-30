use crate::ast::{
    HttpMethod, HttpStatement, RapidAstStatement, RapidProtocolDefinition, RapidRecastDefinition,
    Version,
};
use std::borrow::Cow;

/// An AST that contains only the bare minimum info
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
        ast: vec![],
    }
}

/// A fixture to provide a simple ast with a protocol definition
pub fn schema_with_proto_definition() -> RapidRecastDefinition<'static> {
    let mut schema = bare_minimum_schema();
    schema.ast.push(RapidAstStatement::ProtocolDefinition(
        RapidProtocolDefinition::HttpProtocolDefinition(HttpStatement {
            sequence: 0,
            paths: vec![Cow::Borrowed("/")],
            methods: vec![HttpMethod::GET],
        }),
    ));
    schema
}
