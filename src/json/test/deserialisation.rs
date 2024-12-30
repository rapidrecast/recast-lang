use crate::ast::protocol::{HttpMethod, HttpStatement, RapidProtocolDefinition};
use crate::ast::RapidAstStatement::ProtocolDefinition;
use crate::ast::{RapidRecastDefinition, Version};
use crate::json::JsonRRDL;
use crate::ParseRRDL;
use std::borrow::Cow;

#[test]
pub fn bare_minimum() {
    let input = r#"
    {
        "id": "unique-schema-id-123",
        "language_version": "1.2.3",
        "file_version": "4.5.6",
        "ast": []
    }
    "#;

    let parser = JsonRRDL {};

    let res = parser.parse_rrdl(input).unwrap();
    assert_eq!(
        res,
        RapidRecastDefinition {
            id: Cow::Owned("unique-schema-id-123".to_string()),
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
            name: None,
            description: None,
            ast: vec![],
        }
    )
}

#[test]
pub fn protocol_definition() {
    let input = r#"
    {
        "id": "unique-schema-id-123",
        "language_version": "1.2.3",
        "file_version": "4.5.6",
        "ast": [
            {
                "ProtocolDefinition": {
                    "HttpProtocolDefinition": {
                        "sequence": 0,
                        "paths": ["/user"],
                        "methods": ["POST"],
                        "actions": []
                    }
                }
            }
        ]
    }
    "#;

    let parser = JsonRRDL {};

    let res = parser.parse_rrdl(input).unwrap();
    assert_eq!(
        res,
        RapidRecastDefinition {
            id: Cow::Owned("unique-schema-id-123".to_string()),
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
            name: None,
            description: None,
            ast: vec![ProtocolDefinition(
                RapidProtocolDefinition::HttpProtocolDefinition(HttpStatement {
                    sequence: 0,
                    paths: vec![Cow::Borrowed("/user")],
                    methods: vec![HttpMethod::POST],
                    actions: vec![],
                })
            ),],
        }
    );
}
