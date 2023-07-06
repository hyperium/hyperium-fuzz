#![no_main]

use arbitrary::Arbitrary;
use h2_support::prelude::*;
use lazy_static::lazy_static;

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::fuzz_target;

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

#[derive(Debug, Arbitrary)]
struct HttpSpec {
    uri: Vec<u8>,
    headers: Vec<Header>,
}

#[derive(Debug, Arbitrary)]
struct Header {
    name: Vec<u8>,
    value: Vec<u8>,
}

async fn fuzz_entry(inp: HttpSpec) {
    let mut builder = Request::builder();
    builder = builder.uri(&inp.uri[..]);

    for header in inp.headers.iter() {
        builder = builder.header(&header.name[..], &header.value[..]);
    }

    if let Ok(req) = builder.body(()) {
        let (io, mut _srv) = mock::new();
        let (mut client, _h2) = client::Builder::new()
            .handshake::<_, Bytes>(io)
            .await
            .unwrap();

        // this could still trigger a user error:
        // - if the uri isn't absolute
        // - if the header name isn't allowed in http2 (like connection)
        let _ = std::hint::black_box(client.send_request(req, true));
    }
}

fuzz_target!(|inp: HttpSpec| {
    lazy_static! {
        static ref RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    }
    RT.block_on(fuzz_entry(inp));
});
