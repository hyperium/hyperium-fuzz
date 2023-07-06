#![no_main]

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::fuzz_target;

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

use httpdate::{fmt_http_date, parse_http_date};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(d) = parse_http_date(s) {
            let o = fmt_http_date(d);
            assert!(!o.is_empty());
            assert_eq!(parse_http_date(&o).expect("formatting to round trip"), d);
        }
    }
});
