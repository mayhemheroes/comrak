#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    _ = comrak::markdown_to_html(data, &comrak::ComrakOptions::default());
});
