#![no_main]

#[cfg(feature = "use_libfuzzer")]
use libfuzzer_sys::fuzz_target;

#[cfg(feature = "use_libafl")]
use cargo_libafl_helper::fuzz_target;

use arbitrary::{Arbitrary, Unstructured};
use http::Request;
use http::Response;
use http::StatusCode;

#[derive(Debug, Arbitrary)]
enum ReqOrResp {
    Req,
    Resp,
}

fn arbitrary_version(u: &mut Unstructured) -> arbitrary::Result<http::version::Version> {
    arbitrary::Result::Ok(
        u.choose(&[
            http::version::Version::HTTP_09,
            http::version::Version::HTTP_10,
            http::version::Version::HTTP_11,
            http::version::Version::HTTP_2,
            http::version::Version::HTTP_3,
        ])
        .map_or(http::version::Version::HTTP_11.clone(), |s| s.clone()),
    )
}

#[derive(Debug, Arbitrary)]
struct HttpSpec {
    req_or_resp: ReqOrResp,
    #[arbitrary(with = arbitrary_version)]
    version: http::version::Version,
    method: Option<Vec<u8>>,
    uri: Option<Vec<u8>>,
    headers: Option<Vec<(Vec<u8>, Vec<u8>)>>,
    status_codes: Option<Vec<u8>>,
}

fn execute(inp: HttpSpec) {
    match inp.req_or_resp {
        ReqOrResp::Req => {
            let mut b = Request::builder();

            b = b.version(inp.version);

            if let Some(method) = inp.method {
                if let Ok(method) = http::Method::from_bytes(&method) {
                    b = b.method(&method);
                }
            }
            if let Some(uri) = inp.uri {
                b = b.uri(uri);
            }

            if let Some(headers) = inp.headers {
                for (name, value) in headers.iter() {
                    b = b.header(&name[..], &value[..]);
                }
            }
            let r = b.body(());
            let _ = std::hint::black_box(r);
        }
        ReqOrResp::Resp => {
            let mut b = Response::builder();

            b = b.version(inp.version);

            if let Some(headers) = inp.headers {
                for (name, value) in headers.iter() {
                    b = b.header(&name[..], &value[..]);
                }
            }

            if let Some(status_bytes) = inp.status_codes {
                if let Ok(status) = StatusCode::from_bytes(&status_bytes[..]) {
                    b = b.status(status);
                }
            }

            let r = b.body(());
            let _ = std::hint::black_box(r);
        }
    }
}

fuzz_target!(|inp: HttpSpec| { execute(inp) });
