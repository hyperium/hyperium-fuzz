#![no_main]

/// configure the maximum number of frames that are placed in a stream.
// const MAX_FRAMES: usize = 4096;

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::fuzz_target;

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

use bytes::BytesMut;
use http::{Response, StatusCode};
use lazy_static::lazy_static;

use hyperium_fuzz_utils::http2::*;
use hyperium_fuzz_utils::mockio::client::*;

async fn run(script: &[u8]) -> Result<(), h2::Error> {
    let mut i = 0usize;
    let io = MockIo::from(script);
    let mut h2 = h2::server::handshake(io).await?;
    while let Some(request) = h2.accept().await {
        i = i.wrapping_add(1);
        match request {
            Ok((request, mut respond)) => {
                tracing::info!("Accepted request #{}: {:?}", i, request);

                let _ = std::hint::black_box(request.uri());
                let _ = std::hint::black_box(request.method());
                let v = std::hint::black_box(request.version());
                debug_assert_eq!(v, http::Version::HTTP_2);
                let _ = std::hint::black_box(request.headers());
                let _ = std::hint::black_box(request.extensions());
                let _ = std::hint::black_box(request.body());

                // Build a response with no body
                let response = Response::builder().status(StatusCode::OK).body(()).unwrap();

                // Send the response back to the client
                respond.send_response(response, true).unwrap();
            }
            Err(e) => {
                // hmmm
                tracing::debug!("harness encountered error on accept #{}: {:?}", i, e);
                return Err(e);
            }
        }
    }
    Ok(())
}

fn setup_tracing() -> Option<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    Some(())
}

fuzz_target!(|requests: Vec<Http2Request>| {
    lazy_static! {
        static ref O: Option<()> = setup_tracing();
    }
    assert!(O.is_some());

    if requests.len() == 0 {
        return;
    }
    lazy_static! {
        static ref RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    }
    let mut ctxt = Http2WireContext::new();
    let mut data = BytesMut::with_capacity(1024);
    data.extend_from_slice(HTTP2_PREFACE);
    put_settings_frame(&[], &mut data, 0);

    let mut stream_id = 1;
    for req in requests.into_iter() {
        tracing::debug!("got request: {:?}", req);
        // req.put(&mut data, stream_id, &mut ctxt);
        http2_request_put(req, &mut data, stream_id, &mut ctxt);
        stream_id += 2;
    }

    let _res = RT.block_on(run(&data));
});
