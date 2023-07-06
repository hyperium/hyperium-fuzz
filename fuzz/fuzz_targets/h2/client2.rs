#![no_main]

/// configure the maximum number of frames that are placed in a stream.
// const MAX_FRAMES: usize = 4096;

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::fuzz_target;

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

use arbitrary::Arbitrary;
use bytes::BytesMut;
use lazy_static::lazy_static;
use bytes::Bytes;

use hyperium_fuzz_utils::http2::*;
use h2_support::mock;

const MAX_REQUESTS: u16 = 4092;

#[derive(Debug, Arbitrary)]
struct Header {
    name: Vec<u8>,
    value: Vec<u8>,
}

#[derive(Debug, Arbitrary)]
struct RequestSpec {
    uri: Vec<u8>,
    headers: Vec<Header>,
    body: Option<Vec<u8>>,
    method: HttpMethod,
}

pub type ServerResponseFrames = Vec<(Http2FrameE, Option<u32>)>;

#[derive(Debug, Arbitrary)]
struct FuzzedInput {
    requests: Vec<RequestSpec>,
    responses: ServerResponseFrames,
}

impl FuzzedInput {
    fn parts(&self) -> (&Vec<RequestSpec>, &ServerResponseFrames) {
        (&self.requests, &self.responses)
    }
}

fuzz_target!(|fuzzed: FuzzedInput| {
    #[cfg(feature = "enable_tracing")]
    {
        lazy_static! {
            static ref O: Option<()> = hyperium_fuzz_utils::setup_tracing();
        }
        assert!(O.is_some());
    }

    let (reqs, frames) = fuzzed.parts();

    if reqs.is_empty() {
        return;
    }
    if frames.is_empty() {
        return;
    }
    lazy_static! {
        static ref RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        // static ref RT: tokio::runtime::Runtime =  tokio::runtime::Builder::new_current_thread().build().unwrap();
    }

    let (io, mut handle) = mock::new();

    let fuzzer = async move {
        let mut ctxt = Http2WireContext::new();
        let mut data = BytesMut::with_capacity(1024);

        tracing::info!("mock io start");

        if !matches!(frames[0].0, Http2FrameE::Settings(_)) {
            put_settings_frame(&[], &mut data, 0);
            handle.send_bytes(&data).await;

            // tokio::task::yield_now().await;
        }

        for ((frame, stream_id), i) in frames.into_iter().zip(1..) {
            tracing::debug!("got frame #{}: {:?}", i, frame);
            data.clear();
            frame.put(&mut data, stream_id.unwrap_or(1), &mut ctxt);
            handle.send_bytes(&data).await;

            tokio::task::yield_now().await;
        }
        tracing::info!("mock io end");

        drop(handle);
    };

    let target = async move {
        if let Ok((requester, connection)) = h2::client::handshake(io).await {
            tracing::debug!("client harness completed handshake");
            RT.spawn(async move {
                let _ = connection.await;
            });

            match requester.ready().await {
                Ok(mut requester) => {
                    for (i, req) in reqs.into_iter().enumerate() {
                        if i >= (MAX_REQUESTS as usize) {
                            break;
                        }
                        let mut builder = http::Request::builder();
                        builder = builder.method(req.method.as_bytes());
                        builder = builder.uri(&req.uri[..]);
                        for header in req.headers.iter() {
                            builder = builder.header(&header.name[..], &header.value[..]);
                        }
                        let request = builder.body(());
                        if request.is_err() {
                            tracing::warn!("request nr {} was invalid - stopping", i);
                            break;
                        }
                        let request = request.unwrap();

                        match requester.send_request(request, true) {
                            Ok((response, mut sendstream)) => {
                                if let Some(body) = req.body.as_ref() {
                                    let b = Bytes::copy_from_slice(body);
                                    match sendstream.send_data(b, true) {
                                        Ok(_) => {
                                            tracing::debug!("sending request data success");
                                        }
                                        Err(_e) => {
                                            tracing::debug!("sending request data error");
                                        }
                                    }
                                }
                                match response.await {
                                    Ok(response) => {
                                        let (head, mut body) = response.into_parts();
                                        tracing::debug!("Received response: {:?}", head);

                                        let mut flow_control = body.flow_control().clone();

                                        tracing::debug!("awaiting body data");
                                        tokio::task::yield_now().await;
                                        while let Some(chunk) = body.data().await {
                                            match chunk {
                                                Ok(chunk) => {
                                                    tracing::debug!(
                                                        "received chunk of len {}",
                                                        chunk.len()
                                                    );

                                                    // Let the server send more data.
                                                    let _ =
                                                        flow_control.release_capacity(chunk.len());
                                                }
                                                Err(e) => {
                                                    tracing::debug!(
                                                        "error {:?} while receiving chunk",
                                                        e
                                                    );
                                                    break;
                                                }
                                            }

                                            tokio::task::yield_now().await;
                                        }
                                    }
                                    Err(e) => {
                                        tracing::debug!("error {:?} while awaiting response", e);
                                    }
                                }

                                // if requester not ready; break the loop
                                if let Ok(r) = requester.ready().await {
                                    requester = r;
                                } else {
                                    break;
                                };
                            }
                            Err(e) => {
                                tracing::debug!("{:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::debug!("error {:?} while awaiting requester.ready", e);
                }
            }
        }
    };

    let _res = RT.block_on(futures::future::join(fuzzer, target));
});
