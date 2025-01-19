use simple_json_parser::{parse_with_exit_signal, JSONKey, ParseOptions, RootJSONValue};

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
        &ParseOptions {
            allow_comments: true,
            ..Default::default()
        },
    );
    let without_comments = parse_with_exit_signal(
        source,
        |_keys, _value| {
            eprintln!("{:?}", (_keys, _value));
            false
        },
        &ParseOptions {
            allow_comments: false,
            ..Default::default()
        },
    );

    assert!(with_comments.is_ok());
    assert!(without_comments.is_err());
}
