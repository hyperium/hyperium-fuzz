#![no_main]

const GRAMMAR_NEW_PROB: f64 = 0.1;
const GRAMMAR_DEFAULT_DEPTH: usize = 256;

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::{fuzz_mutator, fuzz_target};

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

use hyper::Uri;
use std::hint::black_box;

#[cfg(all(feature = "use_libfuzzer", feature = "use_grammar"))]
fuzz_mutator!(|data: &mut [u8], size: usize, max_size: usize, seed: u32| {
    use hyperium_fuzz_utils::f0_url_generator::GrammarGenerator;
    use rand::prelude::*;

    let seed = seed as u128;
    let seed = (seed << 96) | (seed << 64) | (seed << 32) | seed;
    let mut rng = rand_pcg::Pcg64Mcg::new(seed);

    let gen_new = data.len() == 0 || rng.gen_bool(GRAMMAR_NEW_PROB);
    if gen_new {
        let new_data = GrammarGenerator::generate_new(Some(GRAMMAR_DEFAULT_DEPTH), &mut rng);
        let new_len = std::cmp::min(max_size, new_data.len());
        data[0..new_len].copy_from_slice(&new_data[0..new_len]);
        new_len
    } else {
        libfuzzer_sys::fuzzer_mutate(data, size, max_size)
    }
});

fn parse_uri(data: &[u8]) {
    // construct a Bytes type to call the from_shared method
    let bytes = bytes::Bytes::copy_from_slice(data);
    match http::Uri::from_maybe_shared(bytes) {
        Ok(uri) => {
            // call all kinds of accessor methods on the Uri type
            let _ = black_box(uri.path());
            let _ = black_box(uri.query());
            let _ = black_box(uri.host());
            let _ = black_box(uri.scheme_str());
            let _ = black_box(uri.scheme());
            let _ = black_box(uri.port());
            let _ = black_box(uri.port_u16());
            let _ = black_box(uri.authority());

            // try to rebuild the Uri again - this should work.
            let mut builder = Uri::builder();
            if let Some(scheme) = uri.scheme() {
                builder = builder.scheme(scheme.clone());
            }
            if let Some(authority) = uri.authority() {
                builder = builder.authority(authority.clone());
            }
            if let Some(pnq) = uri.path_and_query() {
                builder = builder.path_and_query(pnq.clone());
            }
            match builder.build() {
                Ok(new_uri) => {
                    let new_uri = black_box(new_uri);
                    assert_eq!(new_uri, uri);
                }
                Err(e) => {
                    let _ = black_box(e);
                }
            }
        }
        Err(e) => {
            let _ = black_box(e);
        }
    }
}

fuzz_target!(|data: &[u8]| { parse_uri(data) });
