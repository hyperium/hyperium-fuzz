#![no_main]

/// configure the maximum number of frames that are placed in a stream.
// const MAX_FRAMES: usize = 4096;

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::fuzz_target;

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

use bytes::BytesMut;
use futures::{ready, Stream, StreamExt};
use http::{Response, StatusCode};
use lazy_static::lazy_static;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use hyperium_fuzz_utils::http2::*;
// use hyperium_fuzz_utils::mockio::client::*;
use h2_support::mock;
use hyperium_fuzz_utils::setup_tracing;

fuzz_target!(|frames: Vec<(Http2FrameE, Option<u32>)>| {
    #[cfg(feature = "enable_tracing")]
    {
        lazy_static! {
            static ref O: Option<()> = setup_tracing();
        }
        assert!(O.is_some());
    }

    if frames.len() == 0 {
        return;
    }
    // setting up a tokio runtime takes a bit, so we do it lazily once. should be fine to reuse it
    // across fuzzcases.
    lazy_static! {
        // default multi-threaded runtime
        static ref RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        // single-threaded runtime on the current thread
        // static ref RT: tokio::runtime::Runtime =  tokio::runtime::Builder::new_current_thread().build().unwrap();
    }

    let (io, mut handle) = mock::new();

    let client = async move {
        let mut ctxt = Http2WireContext::new();
        let mut data = BytesMut::with_capacity(1024);

        #[cfg(feature = "enable_tracing")]
        tracing::warn!("mock io start");

        handle.write_preface().await;
        // handle.send_bytes(HTTP2_PREFACE).await;

        if !matches!(frames[0].0, Http2FrameE::Settings(_)) {
            put_settings_frame(&[], &mut data, 0);
            handle.send_bytes(&data).await;

            // tokio::task::yield_now().await;
        }

        for ((frame, stream_id), i) in frames.into_iter().zip(1..) {
            #[cfg(feature = "enable_tracing")]
            tracing::debug!("got frame #{}: {:?}", i, frame);

            data.clear();
            frame.put(&mut data, stream_id.unwrap_or(1), &mut ctxt);
            handle.send_bytes(&data).await;

            tokio::task::yield_now().await;
        }

        #[cfg(feature = "enable_tracing")]
        tracing::warn!("mock io end");

    };
    let server = async move {
        let mut i = 0usize;
        let mut h2 = h2::server::handshake(io).await.unwrap();
        #[cfg(feature = "enable_tracing")]
        tracing::debug!("harness handshake complete");

        while let Some(request) = h2.accept().await {
            i = i.wrapping_add(1);
            #[cfg(feature = "enable_tracing")]
            tracing::debug!("harness attempts accept #{}", i);

            match request {
                Ok((request, mut respond)) => {

                    #[cfg(feature = "enable_tracing")]
                    tracing::info!("Accepted request #{}: {:?}", i, request);

                    // call a bunch of methods on the request struct
                    let _ = std::hint::black_box(request.uri());
                    let _ = std::hint::black_box(request.method());
                    let v = std::hint::black_box(request.version());
                    debug_assert_eq!(v, http::Version::HTTP_2);
                    let _ = std::hint::black_box(request.headers());
                    let _ = std::hint::black_box(request.extensions());
                    let _ = std::hint::black_box(request.body());

                    // Build a response with no body...
                    let response = Response::builder().status(StatusCode::OK).body(()).unwrap();
                    // ...and send the response back to the client
                    respond.send_response(response, true).unwrap();
                }
                Err(e) => {
                    #[cfg(feature = "enable_tracing")]
                    tracing::debug!("harness encountered error on accept #{}: {:?}", i, e);

                    // break out of the while loop to stop accepting new requests
                    break;
                }
            }
        }
    };
    let _res = RT.block_on(futures::future::join(server, client));
});
