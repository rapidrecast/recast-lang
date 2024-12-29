use crate::json::JsonRRDL;
use crate::test::bare_minimum_schema;
use crate::SaveRRDL;
use serde_json::Value;

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
