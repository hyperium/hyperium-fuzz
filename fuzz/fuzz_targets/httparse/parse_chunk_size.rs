#![no_main]

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::fuzz_target;

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = std::hint::black_box(httparse::parse_chunk_size(data));
});
