use std::error::Error;

use bytes::{Bytes, BytesMut};
use h2::server::{self, SendResponse};
use h2::RecvStream;
use http::Request;
use tokio::net::{TcpListener, TcpStream};

const BIND_TO: &'static str = "127.0.0.1:5928";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let listener = TcpListener::bind(BIND_TO).await?;
    tracing::info!("listening on {:?}", listener.local_addr());

    let _client_thread = std::thread::spawn(client);

    loop {
        tracing::info!("awaiting call to accept");
        if let Ok((socket, _peer_addr)) = listener.accept().await {
            tokio::spawn(async move {
                if let Err(e) = serve(socket).await {
                    tracing::info!("  -> err={:?}", e);
                }
            });
        }
    }

    // client_thread.join().unwrap();
}

async fn serve(socket: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("H2 server attempting handshake");
    let mut connection = server::handshake(socket).await?;
    tracing::info!("H2 server connection bound");

    while let Some(result) = connection.accept().await {
        match result {
            Ok((request, respond)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_request(request, respond).await {
                        tracing::info!("error while handling request: {}", e);
                    }
                });
            }
            Err(e) => {
                tracing::info!("accept() returned error {:?}", e);
                return Err(Box::new(e));
            }
        }
    }

    tracing::info!("~~~~~~~~~~~ H2 connection CLOSE !!!!!! ~~~~~~~~~~~");
    Ok(())
}

async fn handle_request(
    mut request: Request<RecvStream>,
    mut respond: SendResponse<Bytes>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("h2 server got request: {:?}", request);

    for (k, v) in request.headers().iter() {
        tracing::info!("h2 server received header: {:?} => {:?}", k, v);
    }

    let body = request.body_mut();
    while let Some(data) = body.data().await {
        let data = data?;
        tracing::info!("<<<< recv {:?}", data);
        let _ = body.flow_control().release_capacity(data.len());
    }

    let response = http::Response::new(());
    let mut send = respond.send_response(response, false)?;
    tracing::info!(">>>> send");
    send.send_data(Bytes::from_static(b"hello "), false)?;
    send.send_data(Bytes::from_static(b"world\n"), true)?;

    Ok(())
}

fn client() {
    use bytes::BufMut;
    use hyperium_fuzz_utils::http2::*;
    use std::io::prelude::*;

    let mut ctxt = Http2WireContext::new();
    let mut raw_bytes = BytesMut::new();
    tracing::info!("[client] sending preface");
    raw_bytes.put(HTTP2_PREFACE);

    let frame = Http2FrameE::Settings(Some(vec![]));
    tracing::info!("[client] sending frame: {:?}", frame);
    frame.put(&mut raw_bytes, 0, &mut ctxt);
    tracing::info!("raw: {:?}", raw_bytes);

    let mut first_req = BytesMut::new();
    let frame = Http2FrameE::Headers(HeadersFrame {
        pad_len: None,
        stream_dep: None,
        weight: None,
        headers: vec![],
        exclusive: false,
        end_stream: true,
        end_headers_anyway: false,
        split_into: None,
        required_headers: Some(RequiredHttpHeaders::Request(RequiredHttpRequestHeaders {
            method: HttpMethod::Get,
            scheme: Scheme::Http,
            path: StringOrBytes::String("/what".to_string()),
            authority: AuthorityE::Structured(Authority {
                userinfo: None,
                port: None,
                host: PercentEncodedString::from_str("example.com"),
            }),
        })),
    });
    tracing::info!("[client] sending frame: {:?}", frame);
    frame.put(&mut first_req, 1, &mut ctxt);
    raw_bytes.extend_from_slice(&first_req[..]);
    tracing::info!("raw: {:?}", raw_bytes);

    let mut second_req = BytesMut::new();
    let frame = Http2FrameE::Headers(HeadersFrame {
        pad_len: None,
        stream_dep: None,
        weight: None,
        headers: vec![
            (
                StringOrBytes::from_bytes(b":method"),
                StringOrBytes::from_bytes(b"GET"),
            ),
            (
                StringOrBytes::from_bytes(b":scheme"),
                StringOrBytes::from_bytes(b"http"),
            ),
            (
                StringOrBytes::from_bytes(b":path"),
                StringOrBytes::from_bytes(b"/what"),
            ),
            (
                StringOrBytes::from_bytes(b":authority"),
                StringOrBytes::from_bytes(b"example.com"),
            ),
        ],
        exclusive: false,
        end_stream: true,
        end_headers_anyway: false,
        split_into: None,
        required_headers: None,
    });
    tracing::info!("sending frame: {:?}", frame);
    frame.put(&mut second_req, 1, &mut ctxt);
    assert_eq!(first_req, second_req);
    frame.put(&mut raw_bytes, 3, &mut ctxt);
    tracing::info!("raw: {:?}", raw_bytes);

    let frame = Http2FrameE::Headers(HeadersFrame {
        pad_len: None,
        stream_dep: None,
        weight: None,
        headers: vec![
            (
                StringOrBytes::from_str("x-whatever"),
                StringOrBytes::from_bytes(b"x-whenever"),
            ),
            (
                StringOrBytes::from_str("content-length"),
                StringOrBytes::from_bytes(b"8"),
            ),
        ],
        // headers: vec![],
        exclusive: false,
        end_stream: false,
        end_headers_anyway: false,
        split_into: None,
        required_headers: Some(RequiredHttpHeaders::Request(RequiredHttpRequestHeaders {
            method: HttpMethod::Post,
            scheme: Scheme::Http,
            path: StringOrBytes::String("/what".to_string()),
            authority: AuthorityE::Structured(Authority {
                userinfo: None,
                port: None,
                host: PercentEncodedString::from_str("example.com"),
            }),
        })),
    });
    tracing::info!("[client] sending frame: {:?}", frame);
    frame.put(&mut raw_bytes, 5, &mut ctxt);

    let frame = Http2FrameE::Data(DataFrame {
        pad_len: None,
        end_stream: true,
        data: vec![0u8, 1, 2, 3, 4, 5, 6, 7],
    });
    tracing::info!("[client] sending frame: {:?}", frame);
    frame.put(&mut raw_bytes, 5, &mut ctxt);

    let frame = Http2FrameE::Headers(HeadersFrame {
        pad_len: None,
        stream_dep: None,
        weight: None,
        required_headers: Some(RequiredHttpHeaders::Response(RequiredHttpResponseHeaders {
            status: 128,
        })),
        headers: vec![],
        exclusive: false,
        end_stream: false,
        end_headers_anyway: false,
        split_into: None,
    });
    tracing::info!("[client] sending frame: {:?}", frame);
    frame.put(&mut raw_bytes, 7, &mut ctxt);

    let mut stream = std::net::TcpStream::connect(BIND_TO).unwrap();
    stream.write(&raw_bytes).unwrap();

    let mut buf = Vec::with_capacity(1024);
    buf.resize(1024, 0);
    let mut nothing_count = 0;
    while let Ok(received) = stream.read(&mut buf) {
        tracing::info!("client received: {:?}", &buf[..received]);
        if received == 0 {
            nothing_count += 1;
        }
        if nothing_count > 16 {
            tracing::warn!("read returned nothing too often - exiting");
            std::process::exit(0);
        }
    }
}
