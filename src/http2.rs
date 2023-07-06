/// This file defines a "grammar" for generating HTTP2 data using Arbitrary.

use arbitrary::Arbitrary;
use bytes::BufMut;
use serde::{Deserialize, Serialize};

pub mod serializer {
    use serde::{Deserialize, Serialize};

    pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T, ()>
    where
        T: Deserialize<'a>,
    {
        match bincode::deserialize(bytes) {
            Ok(r) => Ok(r),
            Err(_) => Err(()),
        }
    }

    pub fn serialize_into<T: ?Sized>(dest: &mut [u8], value: &T) -> Result<usize, ()>
    where
        T: Serialize,
    {
        let size = if let Ok(size) = bincode::serialized_size(value) {
            size as usize
        } else {
            return Err(());
        };
        if size > dest.len() {
            return Err(());
        }

        if bincode::serialize_into(dest, value).is_ok() {
            Ok(size)
        } else {
            Err(())
        }
    }
}

pub static HTTP2_PREFACE: &'static [u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

pub const URI_CHARS: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //   x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //  1x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //  2x
    0, 0, 0, b'!', 0, b'#', b'$', 0, b'&', b'\'', //  3x
    b'(', b')', b'*', b'+', b',', b'-', b'.', b'/', b'0', b'1', //  4x
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';', //  5x
    0, b'=', 0, b'?', b'@', b'A', b'B', b'C', b'D', b'E', //  6x
    b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', //  7x
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', //  8x
    b'Z', b'[', 0, b']', 0, b'_', 0, b'a', b'b', b'c', //  9x
    b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', // 10x
    b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', // 11x
    b'x', b'y', b'z', 0, 0, 0, b'~', 0, 0, 0, // 12x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 13x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 14x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 15x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 16x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 17x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 18x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 19x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 20x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 21x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 22x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 23x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 24x
    0, 0, 0, 0, 0, 0, // 25x
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PercentEncodedString {
    pub data: Vec<u8>,
}

impl<'a> Arbitrary<'a> for PercentEncodedString {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let iter = u.arbitrary_iter::<u8>()?;
        let mut v = Vec::new();
        for char in iter {
            Self::encode_into(char?, &mut v);
        }
        Ok(Self { data: v })
    }
}

impl PercentEncodedString {
    pub fn from_str(data: &str) -> Self {
        Self {
            data: Self::encode(data.as_bytes()),
        }
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            data: Self::encode(data),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn encode(data: &[u8]) -> Vec<u8> {
        let mut v = Vec::with_capacity(data.len());
        for char in data.iter().copied() {
            Self::encode_into(char, &mut v);
        }
        v
    }

    pub fn encode_into(char: u8, buf: &mut Vec<u8>) {
        let push_char = URI_CHARS[char as usize];
        if push_char != 0 {
            buf.push(push_char);
        } else {
            // %-encode
            let x = format!("%{:2x}", char).into_bytes();
            buf.extend_from_slice(&x[..]);
        }
    }
}

#[derive(Debug, Clone, Copy, Arbitrary, Serialize, Deserialize, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum Http2FrameType {
    #[default]
    Data = 0,
    Headers = 1,
    Priority = 2,
    RstStream = 3,
    Settings = 4,
    PushPromise = 5,
    Ping = 6,
    GoAway = 7,
    WindowUpdate = 8,
    Continuation = 9,
}

pub trait Http2FrameToBuf {
    fn put(&self, buf: &mut impl BufMut, stream_id: u32, ctxt: &mut Http2WireContext) -> usize;
    fn len(&self) -> usize;
    fn flags(&self) -> u8;
    fn get_type(&self) -> Http2FrameType;
}

pub fn put_header(buf: &mut impl BufMut, length: u32, frame_type: u8, flags: u8, stream_id: u32) {
    buf.put_uint(length as u64, 3);
    buf.put_u8(frame_type);
    buf.put_u8(flags);
    buf.put_u32(stream_id & ((1u32 << 31) - 1));
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataFrame {
    pub pad_len: Option<u8>,
    pub end_stream: bool,
    pub data: Vec<u8>,
}

const PADDED: u8 = 0x8;
const END_STREAM: u8 = 0x1;

impl Http2FrameToBuf for DataFrame {
    fn len(&self) -> usize {
        (self.pad_len.unwrap_or(0) as usize) + self.data.len()
    }

    fn get_type(&self) -> Http2FrameType {
        Http2FrameType::Data
    }

    fn flags(&self) -> u8 {
        let mut flags = 0u8;
        if self.end_stream {
            flags |= END_STREAM
        }
        if self.pad_len.is_some() {
            flags |= PADDED
        }
        flags
    }

    fn put(&self, buf: &mut impl BufMut, stream_id: u32, _ctxt: &mut Http2WireContext) -> usize {
        /*
        +---------------+
        |Pad Length? (8)|
        +---------------+-----------------------------------------------+
        |                            Data (*)                         ...
        +---------------------------------------------------------------+
        |                           Padding (*)                       ...
        +---------------------------------------------------------------+
        */
        let pad_len = self.pad_len.unwrap_or(0) as u32;
        let total_len =
            self.data.len() as u32 + pad_len + if self.pad_len.is_some() { 1 } else { 0 };

        put_header(
            buf,
            total_len,
            self.get_type() as u8,
            self.flags() as u8,
            stream_id,
        );

        buf.put_slice(&self.data[..]);
        buf.put_bytes(0, pad_len as usize);

        total_len as usize
    }
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub enum StringOrBytes {
    String(String),
    Bytes(Vec<u8>),
}

impl StringOrBytes {
    pub fn from_str(s: &str) -> Self {
        Self::String(s.to_string())
    }
    pub fn from_bytes(b: &[u8]) -> Self {
        Self::Bytes(b.to_vec())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Bytes(b) => &b[..],
            Self::String(s) => s.as_bytes(),
        }
    }
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    Garbage(Vec<u8>),
}

impl HttpMethod {
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Options => b"OPTIONS",
            Self::Get => b"GET",
            Self::Post => b"POST",
            Self::Put => b"PUT",
            Self::Delete => b"DELETE",
            Self::Head => b"HEAD",
            Self::Trace => b"TRACE",
            Self::Connect => b"CONNECT",
            Self::Patch => b"PATCH",
            Self::Garbage(d) => &d[..],
        }
    }
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub enum Scheme {
    Http,
    Https,
    Garbage(Vec<u8>),
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub struct Authority {
    pub userinfo: Option<PercentEncodedString>,
    pub host: PercentEncodedString,
    pub port: Option<u16>,
}

impl Authority {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut v = Vec::new();
        if let Some(uinfo) = &self.userinfo {
            v.extend_from_slice(&uinfo.to_vec());
            v.push(b'@');
        }
        v.extend_from_slice(&self.host.to_vec());
        if let Some(port) = self.port {
            let b = format!(":{}", port).into_bytes();
            v.extend_from_slice(&b[..]);
        }
        v
    }
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorityE {
    Garbage(StringOrBytes),
    Structured(Authority),
}

impl AuthorityE {
    fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Garbage(b) => b.to_vec(),
            Self::Structured(s) => s.to_vec(),
        }
    }
}

impl Scheme {
    fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Http => b"http",
            Self::Https => b"https",
            Self::Garbage(d) => &d[..],
        }
    }
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequiredHttpRequestHeaders {
    pub method: HttpMethod,
    pub scheme: Scheme,
    pub path: StringOrBytes,
    pub authority: AuthorityE,
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequiredHttpResponseHeaders {
    pub status: u16,
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub enum RequiredHttpHeaders {
    Request(RequiredHttpRequestHeaders),
    Response(RequiredHttpResponseHeaders),
}

impl RequiredHttpHeaders {
    pub fn to_vec(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
        match self {
            Self::Response(r) => r.to_vec(),
            Self::Request(r) => r.to_vec(),
        }
    }
}

impl RequiredHttpRequestHeaders {
    pub fn to_vec(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
        vec![
            (b":method".to_vec(), self.method.to_vec()),
            (b":scheme".to_vec(), self.scheme.to_vec()),
            (b":path".to_vec(), self.path.to_vec()),
            (b":authority".to_vec(), self.authority.to_vec()),
        ]
    }

    // pub fn as_bytes(&self) -> Vec<(&[u8], &[u8])> {
    //     vec![
    //         (b":method", self.method.as_bytes()),
    //         (b":scheme", self.scheme.as_bytes()),
    //         (b":path", self.path.as_bytes()),
    //         (b":authority", self.authority.as_bytes()),
    //     ]
    // }
}

impl RequiredHttpResponseHeaders {
    pub fn to_vec(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
        vec![(b":status".to_vec(), format!("{}", self.status).into_bytes())]
    }
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub struct HeadersFrame {
    pub pad_len: Option<u8>,
    pub stream_dep: Option<u32>,
    pub weight: Option<u8>,
    pub required_headers: Option<RequiredHttpHeaders>,
    pub headers: Vec<(StringOrBytes, StringOrBytes)>,
    pub exclusive: bool,
    pub end_stream: bool,
    pub end_headers_anyway: bool,
    pub split_into: Option<u8>,
}

const END_HEADERS: u8 = 0x4;
const PRIORITY: u8 = 0x20;

impl Http2FrameToBuf for HeadersFrame {
    fn get_type(&self) -> Http2FrameType {
        Http2FrameType::Headers
    }

    fn len(&self) -> usize {
        panic!("cannot determine length before encoding headers");
    }

    fn flags(&self) -> u8 {
        let mut flags = 0u8;
        if self.pad_len.is_some() {
            flags |= PADDED;
        }
        if self.stream_dep.is_some() || self.weight.is_some() {
            flags |= PRIORITY;
        }
        if self.end_stream {
            flags |= END_STREAM;
        }
        if self.end_headers_anyway || self.split_into.is_none() {
            flags |= END_HEADERS;
        }
        flags
    }

    fn put(&self, buf: &mut impl BufMut, stream_id: u32, ctxt: &mut Http2WireContext) -> usize {
        /*
        +---------------+
        |Pad Length? (8)|
        +-+-------------+-----------------------------------------------+
        |E|                 Stream Dependency? (31)                     |
        +-+-------------+-----------------------------------------------+
        |  Weight? (8)  |
        +-+-------------+-----------------------------------------------+
        |                   Header Block Fragment (*)                 ...
        +---------------------------------------------------------------+
        |                           Padding (*)                       ...
        +---------------------------------------------------------------+
        */

        let pad_len = self.pad_len.unwrap_or(0);
        let headers = if let Some(required_headers) = &self.required_headers {
            let mut headers = Vec::with_capacity(self.headers.len() + 3);
            headers.extend(required_headers.to_vec().into_iter());
            headers.extend(self.headers.iter().map(|(k, v)| (k.to_vec(), v.to_vec())));
            headers
        } else {
            self.headers
                .iter()
                .map(|(k, v)| (k.to_vec(), v.to_vec()))
                .collect()
        };
        let encoded = ctxt
            .hpack_encoder
            .encode(headers.iter().map(|(k, v)| (&k[..], &v[..])));

        // tracing::debug!("encoded headers as {:?}", encoded);
        let mut total_len = pad_len as usize + encoded.len();
        if self.stream_dep.is_some() || self.weight.is_some() {
            total_len += 4 + 1;
        }
        if self.pad_len.is_some() {
            total_len += 1;
        }

        if self.split_into.is_none() {
            put_header(
                buf,
                total_len as u32,
                self.get_type() as u8,
                self.flags() as u8,
                stream_id,
            );

            self.put_start(buf, stream_id, ctxt);

            // header block fragment
            buf.put_slice(&encoded[..]);
            // padding
            buf.put_bytes(0, pad_len as usize);
        } else {
            let encoded_len = encoded.len();
            return self.put_continued(buf, stream_id, ctxt, encoded, total_len - encoded_len);
        }

        total_len as usize
    }
}

impl HeadersFrame {
    pub fn put_start(&self, buf: &mut impl BufMut, _stream_id: u32, _ctxt: &mut Http2WireContext) {
        if let Some(pad_len) = self.pad_len {
            buf.put_u8(pad_len);
        }

        if self.stream_dep.is_some() || self.weight.is_some() {
            let mut sid = 0u32;
            if self.exclusive {
                sid |= 1u32 << 31;
            }
            if let Some(id) = self.stream_dep {
                sid |= id;
            }
            buf.put_u32(sid);
            buf.put_u8(self.weight.unwrap_or(0));
        }
    }

    pub fn put_continued(
        &self,
        buf: &mut impl BufMut,
        stream_id: u32,
        ctxt: &mut Http2WireContext,
        encoded: Vec<u8>,
        pre_header_len: usize,
    ) -> usize {
        let split_into = self.split_into.unwrap();
        let mut factor = if split_into < 2 {
            2
        } else {
            split_into as usize
        };
        while encoded.len() % factor != 0 && factor > 0 {
            factor -= 1;
        }
        let mut encoded_slices = encoded.chunks(factor).peekable();
        if let Some(sl) = encoded_slices.next() {
            put_header(
                buf,
                (pre_header_len + sl.len()) as u32,
                self.get_type() as u8,
                self.flags() as u8,
                stream_id,
            );

            self.put_start(buf, stream_id, ctxt);
            buf.put_slice(sl);

            let pad_len = self.pad_len.unwrap_or(0);
            buf.put_bytes(0, pad_len as usize);
        }

        while let Some(sl) = encoded_slices.next() {
            let end_headers = encoded_slices.peek().is_none();
            put_header(
                buf,
                sl.len() as u32,
                Http2FrameType::Continuation as u8,
                if end_headers { END_HEADERS } else { 0 },
                stream_id,
            );
            buf.put_slice(sl);
        }

        // TODO:
        0
    }
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub struct PushPromiseFrame {
    pad_len: Option<u8>,
    promised_stream_id: u32,
    headers: Vec<(Vec<u8>, Vec<u8>)>,
    end_headers_anyway: bool,
    split_into: Option<u8>,
}

impl Http2FrameToBuf for PushPromiseFrame {
    fn get_type(&self) -> Http2FrameType {
        Http2FrameType::PushPromise
    }

    fn len(&self) -> usize {
        panic!("cannot determine length before encoding headers");
    }

    fn flags(&self) -> u8 {
        let mut flags = 0u8;
        if self.pad_len.is_some() {
            flags |= PADDED;
        }
        if self.end_headers_anyway {
            flags |= END_HEADERS;
        }
        flags
    }

    fn put(&self, buf: &mut impl BufMut, stream_id: u32, ctxt: &mut Http2WireContext) -> usize {
        /*
        +---------------+
        |Pad Length? (8)|
        +-+-------------+-----------------------------------------------+
        |R|                  Promised Stream ID (31)                    |
        +-+-----------------------------+-------------------------------+
        |                   Header Block Fragment (*)                 ...
        +---------------------------------------------------------------+
        |                           Padding (*)                       ...
        +---------------------------------------------------------------+
        */

        let pad_len = self.pad_len.unwrap_or(0);
        let encoded = ctxt
            .hpack_encoder
            .encode(self.headers.iter().map(|h| (&h.0[..], &h.1[..])));
        let mut total_len = pad_len as usize + encoded.len();
        if self.pad_len.is_some() {
            total_len += 1;
        }

        if self.split_into.is_none() {
            put_header(
                buf,
                total_len as u32,
                self.get_type() as u8,
                self.flags() as u8,
                stream_id,
            );
            if let Some(pad_len) = self.pad_len {
                buf.put_u8(pad_len);
            }
            buf.put_u32(self.promised_stream_id & ((1u32 << 31) - 1));
            // header block fragment
            buf.put_slice(&encoded[..]);
            // padding
            buf.put_bytes(0, pad_len as usize);
        } else {
            let encoded_len = encoded.len();
            return self.put_continued(buf, stream_id, ctxt, encoded, total_len - encoded_len);
        }

        total_len as usize
    }
}

impl PushPromiseFrame {
    pub fn put_continued(
        &self,
        buf: &mut impl BufMut,
        stream_id: u32,
        _ctxt: &mut Http2WireContext,
        encoded: Vec<u8>,
        pre_header_len: usize,
    ) -> usize {
        let split_into = self.split_into.unwrap();
        let mut factor = if split_into < 2 {
            2
        } else {
            split_into as usize
        };
        while encoded.len() % factor != 0 && factor > 0 {
            factor -= 1;
        }
        let mut encoded_slices = encoded.chunks(factor).peekable();
        if let Some(sl) = encoded_slices.next() {
            put_header(
                buf,
                (pre_header_len + sl.len()) as u32,
                self.get_type() as u8,
                self.flags() as u8,
                stream_id,
            );

            buf.put_u32(self.promised_stream_id & ((1u32 << 31) - 1));
            buf.put_slice(sl);

            let pad_len = self.pad_len.unwrap_or(0);
            buf.put_bytes(0, pad_len as usize);
        }

        while let Some(sl) = encoded_slices.next() {
            let end_headers = encoded_slices.peek().is_none();
            put_header(
                buf,
                sl.len() as u32,
                Http2FrameType::Continuation as u8,
                if end_headers { END_HEADERS } else { 0 },
                stream_id,
            );
            buf.put_slice(sl);
        }

        // TODO:
        0
    }
}

pub type Http2SettingsEntry = (u16, u32);

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub enum Http2FrameE {
    Garbage(u8, u8, Vec<u8>), // captures all other frame types - and invalid ones
    Data(DataFrame),
    Headers(HeadersFrame),
    Priority(bool, Option<u32>, u8),
    Settings(Option<Vec<Http2SettingsEntry>>),
    ResetStream(u32),
    Ping(u64, bool),
    Continuation(Vec<u8>, bool),
    GoAway(u32, u32, Vec<u8>),
    WindowUpdate(u32),
    PushPromise(PushPromiseFrame),
}

const ACK: u8 = 0x1;

impl Http2FrameE {
    pub fn put(&self, buf: &mut impl BufMut, stream_id: u32, ctxt: &mut Http2WireContext) {
        match self {
            Self::Garbage(frame_type, flags, data) => {
                put_header(buf, data.len() as u32, *frame_type, *flags, stream_id);
                buf.put_slice(&data[..]);
            }
            Self::Data(frame) => {
                frame.put(buf, stream_id, ctxt);
            }
            Self::Headers(frame) => {
                frame.put(buf, stream_id, ctxt);
            }
            Self::PushPromise(frame) => {
                frame.put(buf, stream_id, ctxt);
            }
            Self::Priority(exclusive, stream_dep, weight) => {
                put_header(buf, 5, Http2FrameType::Priority as u8, 0, stream_id);

                let mut sid = stream_dep.unwrap_or(1);
                if *exclusive {
                    sid |= 1u32 << 31;
                }
                buf.put_u32(sid);
                buf.put_u8(*weight);
            }
            Self::ResetStream(code) => {
                put_header(buf, 4, Http2FrameType::RstStream as u8, 0, stream_id);
                buf.put_u32(*code);
            }
            Self::Settings(settings) => {
                if let Some(settings) = settings {
                    put_settings_frame(settings, buf, stream_id);
                } else {
                    // send ACK flag
                    put_header(buf, 0, Http2FrameType::Settings as u8, ACK, stream_id);
                }
            }
            Self::Ping(data, is_ack) => {
                put_header(
                    buf,
                    8,
                    Http2FrameType::Ping as u8,
                    if *is_ack { ACK } else { 0 },
                    stream_id,
                );
                buf.put_u64(*data);
            }
            Self::GoAway(last_stream_id, error_code, data) => {
                put_header(buf, data.len() as u32, Http2FrameType::GoAway as u8, 0, 0);
                buf.put_u32(last_stream_id & ((1u32 << 31) - 1));
                buf.put_u32(*error_code);
                buf.put_slice(data);
            }
            Self::WindowUpdate(size_increment) => {
                put_header(buf, 4, Http2FrameType::WindowUpdate as u8, 0, stream_id);
                buf.put_u32(size_increment & ((1u32 << 31) - 1));
            }
            Self::Continuation(data, end_headers) => {
                put_header(
                    buf,
                    data.len() as u32,
                    Http2FrameType::Continuation as u8,
                    if *end_headers { END_HEADERS } else { 0 },
                    stream_id,
                );
                buf.put_slice(&data);
            }
        }
    }
}

pub fn put_settings_frame(settings: &[Http2SettingsEntry], buf: &mut impl BufMut, stream_id: u32) {
    let total_len = settings.len() * 6;
    put_header(
        buf,
        total_len as u32,
        Http2FrameType::Settings as u8,
        0,
        stream_id,
    );
    for (key, value) in settings.iter().copied() {
        buf.put_u16(key);
        buf.put_u32(value);
    }
}

#[derive(Debug, Clone, Arbitrary, Serialize, Deserialize, PartialEq, Eq)]
pub struct Http2Request {
    pub required_headers: RequiredHttpRequestHeaders,
    pub headers: Option<Vec<(StringOrBytes, StringOrBytes)>>,
    pub split_into: Option<u8>,

    pub dataframes: Option<Vec<DataFrame>>,
}

pub fn http2_request_put(
    request: Http2Request,
    buf: &mut impl BufMut,
    stream_id: u32,
    ctxt: &mut Http2WireContext,
) {
    let has_data_frames = if let Some(dataframes) = &request.dataframes {
        dataframes.len() > 0
    } else {
        false
    };

    let frame = HeadersFrame {
        stream_dep: None,
        weight: None,
        split_into: request.split_into,
        required_headers: Some(RequiredHttpHeaders::Request(request.required_headers)),
        headers: request.headers.unwrap_or(Vec::new()),
        end_headers_anyway: false,
        pad_len: None,
        end_stream: has_data_frames,
        exclusive: false,
    };
    frame.put(buf, stream_id, ctxt);

    if let Some(dataframes) = request.dataframes {
        let mut iter = dataframes.into_iter().peekable();
        while let Some(mut frame) = iter.next() {
            if iter.peek().is_none() {
                frame.end_stream = true;
            } else {
                frame.end_stream = false;
            }
            frame.put(buf, stream_id, ctxt);
        }
    }
}

pub struct Http2WireContext<'a> {
    hpack_encoder: Box<hpack::Encoder<'a>>,
}

impl<'a> Http2WireContext<'a> {
    pub fn new() -> Self {
        Self {
            hpack_encoder: Box::new(hpack::Encoder::new()),
        }
    }
}
