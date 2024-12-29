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
        "file_version": "4.5.6"
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
