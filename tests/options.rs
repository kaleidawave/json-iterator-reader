use simple_json_parser::{parse_with_exit_signal, JSONKey, RootJSONValue};

#[test]
fn disable_comments() {
    let source = r#"{
        // some comment
        "hi": "Ben"
    }"#;

    let with_comments = parse_with_exit_signal(
        source,
        |keys, value| {
            if let &[JSONKey::Slice("hi")] = keys {
                assert_eq!(value, RootJSONValue::String("Ben"));
            } else {
                panic!()
            }
            false
        },
        true,
        true,
    );
    let without_comments = parse_with_exit_signal(
        source,
        |_keys, _value| {
            eprintln!("{:?}", (_keys, _value));
            false
        },
        true,
        false,
    );

    assert!(with_comments.is_ok());
    assert!(without_comments.is_err());
}
