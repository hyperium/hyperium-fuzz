#![no_main]

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::fuzz_target;

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

fuzz_target!(|data_: &[u8]| {
    let decoder = h2::fuzz_bridge::fuzz_logic::fuzz_hpack(data_);
    let _ = std::hint::black_box(decoder);
});
