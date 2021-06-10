#![no_main]
use libfuzzer_sys::fuzz_target;
use naga::front::glsl::Options;
use naga::front::glsl::parse_str;

fuzz_target!(|data: String| {
    // Ensure the parser can handle potentially malformed strings without crashing.
    let options = Options::simple(true, false, false);
    let _result = parse_str(&data, &options);
});
