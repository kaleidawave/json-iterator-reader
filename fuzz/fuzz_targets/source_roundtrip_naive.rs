#![no_main]

use simple_json_parser::parse;
use libfuzzer_sys::{fuzz_target, Corpus};
use std::str;

/// `do_fuzz` will take an arbitrary string, parse once and see if it returned a valid source
fn do_fuzz(data: &str) -> Corpus {
	let input = data.trim_start();

    let result =
        simple_json_parser::parse(
            &input,
            |_keys, _value| {},
        );

    match result {
        Ok(_value) => {
            Corpus::Keep
        }
        Err(_err) => {
            Corpus::Reject
        }
    }
}

fuzz_target!(|data: &str| {
	do_fuzz(data);
});