use crate::ast::{RapidRecastDefinition, Version};
use crate::json::JsonRRDL;
use crate::test::bare_minimum_schema;
use crate::{ParseRRDL, SaveRRDL};
use serde_json::Value;
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
pub fn test_basic_save() {
    let definition = bare_minimum_schema();
    let res = JsonRRDL {}.save_rrdl(&definition);
    let res_str = String::from_utf8(res.into_inner()).unwrap();
    let res: Value = serde_json::from_str(&res_str).unwrap();
    let expected_str = r#"{
        "id":"unique-schema-id-123",
        "language_version":"1.2.3",
        "file_version":"4.5.6",
        "name":"some name",
        "description":"some description",
        "ast":[
            {"ProtocolDefinition":{"protocol":"HTTP","sequence":0}}
        ]}"#;
    let expected: Value = serde_json::from_str(expected_str).unwrap();
    assert!(expected.is_object());
    assert!(res.is_object());
    assert_eq!(res, expected);
}
