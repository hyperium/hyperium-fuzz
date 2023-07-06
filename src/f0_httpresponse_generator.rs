#![allow(unused)]
use rand::Rng;
use std::cell::Cell;

pub struct GrammarGenerator;

pub static TERMINALS: [&'static str; 526] = ["http://example.com", "http://example.com/example/sub", "http://example.com/example/sub?a=b", "http://example.com/example/sub?a=b&c=12345", "http://example.com/example/sub?a=b&c=12345#asdf-asdf", "http://example.com/example/sub#asdf-asdf", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "16", "31", "32", "33", "63", "64", "65", "255", "257", "256", "128", "127", "129", "65535", "65537", "65536", "32768", "32767", "32769", "4294967295", "4294967297", "4294967296", "2147483648", "2147483647", "2147483649", "281474976710655", "281474976710657", "281474976710656", "140737488355328", "140737488355327", "140737488355329", "18446744073709551615", "18446744073709551617", "18446744073709551616", "9223372036854775808", "9223372036854775807", "9223372036854775809", "340282366920938463463374607431768211455", "340282366920938463463374607431768211457", "340282366920938463463374607431768211456", "170141183460469231731687303715884105728", "170141183460469231731687303715884105727", "170141183460469231731687303715884105729", "115792089237316195423570985008687907853269984665640564039457584007913129639935", "115792089237316195423570985008687907853269984665640564039457584007913129639937", "115792089237316195423570985008687907853269984665640564039457584007913129639936", "57896044618658097711785492504343953926634992332820282019728792003956564819968", "57896044618658097711785492504343953926634992332820282019728792003956564819967", "57896044618658097711785492504343953926634992332820282019728792003956564819969", ".", "-", "e", "e+", "e-", "A", "B", "C", "D", "E", "F", "a", "b", "c", "d", "f", "0x", "0X", "h", "1234", "0001234", "0xdeadbeef", "0DEADBEEF", "0_1234_45667", "-1", "-9", "1234567890", "0123456789", "+", "/", "=", "==", "!", "#", "$", "%", "&", "\"", "(", ")", "*", ",", ":", ";", "<", ">", "?", "@", "[", "]", "^", "_", "`", "{", "|", "}", "~", " ", "Sun, 06 Nov 1994 08:49:37 GMT", "Sun, 06 Nov 2094 08:49:37 GMT", "Sun, 06 Nov 1900 08:49:37 GMT", "Sun, 45 Nov 2056 08:49:37 GMT", "Sun, 15 Nov 2006 08:49:37 GMT+10", "Sun, 15 Nov 2006 08:49:37 GMT+16", "Sun, 11 Nov 1111 11:11:11 GMT+11", "g", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Dec", "    ", "         ", "                ", "                 ", "08:49:37", "00:00:00", "01:01:01", "11:11:11", "GMT", "UTC", "GMT+", "UTC+", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "1991", "1091", "2091", "9999", "99999", "-1999", "example.com", "sub.example.com", "subsub.sub.example.com", "a.b.c.d.f.g.h.i.j.k.example.com", "http://", "https://", "ftp://", "file://", "Accept-Charset", "!<!string.spaces>", "\r\n", "Accept-Encoding:", "Accept-Language", "Accept-Ranges", "Accept:", "Allow", "Allow:", "ALPN", "ALPN:", "Alt-Used", "Alt-Used:", "Basic", "Bearer", "Digest", "HOBA", "Mutual", "Negotiate", "OAuth", "SCRAM-SHA-1", "SCRAM-SHA-256", "vapid", "Authorization", "Cache-Control:", "max-age=5", "max-stale=5", "min-fresh=5", "no-cache", "no-store", "no-transform", "only-if-cached", "CalDav-Timezones", "foo123.foocdn.example", "barcdn.example; trace=\"abcdef\"", "AnotherCDN; abc=123; def=\"456\"", "a=1", "CND-Loop", "utf-16", "utf-16BE", "utf-32", "utf-32BE", "us-ascii", "iso-8859-1", "utf-7", "utf-8", "AAAA", "BBBB", ";foo=bar", "Transfer-Encoding:", "chunked", "Content-Encoding", "Content-Language", "Content-Length: 200\r\n", "Content-Length: 0\r\n", "Content-Length: ", "Content-Location", "SID=31d4d96e407aad42", "PHPSESSID=298zf09hf012fh2; csrftoken=u32t4o3tb3gg43; _gat=1", "Cookie", "123456", "YWxhZGRpbjpvcGVuc2VzYW1l", "Date", "infinity", "Depth:", "Destination", "Early-Data", "gzip", "compress", "deflate", "br", "identity", "encoding-name", "\"xyzzy\"", "\"AAAAAAAAAAAAAAAAAAAAAAAAA\"", "Expect", "100-continue", "Expires:", "Forwarded", "by", "From", "webmaster@w3.org", "Accept", "Accept-Encoding", "Cache-Control", "CDN-Loop", "Content-Length", "Depth", "Expires", "HTTP2-Settings", "If", "If-Match", "If-Modified-Since", "If-None-Match", "If-Range", "If-Schedule-Tag-Match", "If-Unmodified-Since", "Link", "Max-Forwards", "MIME-Version", "OData-Isolation", "OData-MaxVersion", "OData-Version", "Ordering-Type", "Origin", "OSCORE", "Overwrite", "Position", "Pragma", "Prefer", "Proxy-Authorization", "Range", "Referer", "Schedule-Reply", "Sec-Token-Binding", "Sec-Websocket-Accept", "Sec-Websocket-Extensions", "Sec-Websocket-Key", "Sec-Websocket-Protocol", "Sec-Websocket-Version", "Slug", "TE", "Timeout", "Topic", "Trailer", "Transfer-Encoding", "TTL", "Urgency", "Upgrade", "User-Agent", "Via", "Server", "Last-Modified", "Transfer-Encoding: chunked\r\n", "Transfer-Encoding: identity\r\n", "Content-Length: 180\r\n", "Bar: Foo\r\n", "0.9", "1.0", "1.1", "2.0", "3.0", "HTTP/", "AAMAAABkAARAAAAAAAIAAAAA", "If-Match:", "If-Modified-Since:", "If-None-Match:", "If-Range:", "If-Schedule-Tag-Match:", "If-Unmodified-Since:", "fr", "en", "de", "Link:", "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "*/*", "application/octet-stream", "application/pdf", "application/pkcs8", "application/zip", "audio/mpeg", "audio/vorbis", "audio/example", "font/woff", "font/ttf", "font/otf", "image/jpeg", "image/png", "image/svg+xml", "model/3mf", "text/html", "video/mp4", "snapshot", "4.0", "DAV:unordered", "DAV:custom", "http://example.org/example.html", "null", "CSU", "AA", "first", "last", "after example.html", "respond-async", "wait=100", "handling=lenient", "http%2F1.1", "h2", "http%2F", "Proxy-Authorization:", "q=1.0", "q=0.0", "q=", "bytes", "none", "5-8", "5-", "Range:", "/example", "AIkAAgBBQLgtRpWFPN66kxhxGrtaKrzcMtHw7HV8", "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=", "deflate-stream", "mux", "max-channels:4; flow-control", "Sec-Websocket-Extensions:", "dGhlIHNhbXBsZSBub25jZQ==", "Sec-Websocket-Key:", "chat", "superchat", "Sec-Websocket-Protocol:", "13", "Sec-Websocket-Version:", "The Beach at S%C3%A8te", "Slug:", "(<urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2> [\"I am an ETag\"]) ([\"I am another ETag\"])", "(Not <urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2>", "<urn:uuid:58f202ac-22cf-11d1-b12d-002035b29092>)", "trailers", "TE:", "Seconds", "Hours", "Days", "Infinite", "Second-4100000000", "Second-", "Timeout:", "upd", "Topic:", "Trailer:", "TTL: 0", "TTL: 1", "TTL: ", "websocket", "HTTP/2.0", "SHTTP/1.3", "IRC/6.9", "RTA/x11", "Upgrade:", "very-low", "low", "normal", "high", "Urgency:", "curl/7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:104.0) Gecko/20100101 Firefox/104.0", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.102 Safari/537.36 OPR/90.0.4480.84", "whatever/13.37", "User-Agent:", "100 Continue", "101 Switching Protocols", "102 Processing", "103 Early Hints", "200 OK", "201 Created", "202 Accepted", "203 Non-Authoritative Information", "204 No Content", "205 Reset Content", "206 Partial Content", "207 Multi-Status", "208 Already Reported", "226 IM Used", "300 Multiple Choices", "301 Moved Permanently", "302 Found", "303 See Other", "304 Not Modified", "307 Temporary Redirect", "308 Permanent Redirect", "400 Bad Request", "401 Unauthorized", "402 Payment Required", "403 Forbidden", "404 Not Found", "405 Method Not Allowed", "406 Not Acceptable", "407 Proxy Authentication Required", "408 Request Timeout", "409 Conflict", "410 Gone", "411 Length Required", "412 Precondition Failed", "413 Content Too Large", "414 URI Too Long", "415 Unsupported Media Type", "416 Range Not Satisfiable", "417 Expectation Failed", "418 I'm a teapot", "421 Misdirected Request", "422 Unprocessable Content", "423 Locked", "424 Failed Dependency", "425 Too Early", "426 Upgrade Required", "428 Precondition Required", "429 Too Many Requests", "431 Request Header Fields Too Large", "451 Unavailable For Legal Reasons", "500 Internal Server Error", "501 Not Implemented", "502 Bad Gateway", "503 Service Unavailable", "504 Gateway Timeout", "505 HTTP Version Not Supported", "506 Variant Also Negotiates", "507 Insufficient Storage", "508 Loop Detected", "510 Not Extended", "511 Network Authentication Required", " fred", " whatever.example.com", " example.com", "Via:", ];

impl GrammarGenerator {
    pub fn terminals() -> &'static [&'static str] {
        return &TERMINALS;
    }

    pub fn generate_into(out: &mut Vec<u8>, max_depth: Option<usize>, rng: &mut impl Rng) {
        out.clear();
        Self::fragment_2(0, max_depth.unwrap_or(1024 as usize), out, rng);
    }

    pub fn generate_new(max_depth: Option<usize>, rng: &mut impl Rng) -> Vec<u8> {
        let mut out = Vec::new();
        Self::generate_into(&mut out, max_depth, rng);
        out
    }
    fn fragment_2(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2794(depth + 1, max_depth, buf, rng);
        Self::fragment_2795(depth + 1, max_depth, buf, rng);
        Self::fragment_2796(depth + 1, max_depth, buf, rng);
    }
    fn fragment_79(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1857(depth + 1, max_depth, buf, rng);
        Self::fragment_1858(depth + 1, max_depth, buf, rng);
    }
    fn fragment_221(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://example.com" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 18;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111,
                    109,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                18,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_223(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://example.com/example/sub" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 30;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111,
                    109, 47, 101, 120, 97, 109, 112, 108, 101, 47, 115, 117, 98,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                30,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_225(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://example.com/example/sub?a=b" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 34;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111,
                    109, 47, 101, 120, 97, 109, 112, 108, 101, 47, 115, 117, 98, 63, 97, 61, 98,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                34,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_227(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://example.com/example/sub?a=b&c=12345" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 42;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111,
                    109, 47, 101, 120, 97, 109, 112, 108, 101, 47, 115, 117, 98, 63, 97, 61, 98,
                    38, 99, 61, 49, 50, 51, 52, 53,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                42,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_229(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://example.com/example/sub?a=b&c=12345#asdf-asdf" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 52;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111,
                    109, 47, 101, 120, 97, 109, 112, 108, 101, 47, 115, 117, 98, 63, 97, 61, 98,
                    38, 99, 61, 49, 50, 51, 52, 53, 35, 97, 115, 100, 102, 45, 97, 115, 100, 102,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                52,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_231(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://example.com/example/sub#asdf-asdf" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 40;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111,
                    109, 47, 101, 120, 97, 109, 112, 108, 101, 47, 115, 117, 98, 35, 97, 115, 100,
                    102, 45, 97, 115, 100, 102,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                40,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_260(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_261(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_262(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_263(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_260(depth + 1, max_depth, buf, rng);
        Self::fragment_261(depth + 1, max_depth, buf, rng);
        Self::fragment_262(depth + 1, max_depth, buf, rng);
    }
    fn fragment_264(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_265(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_266(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_267(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_264(depth + 1, max_depth, buf, rng);
        Self::fragment_265(depth + 1, max_depth, buf, rng);
        Self::fragment_266(depth + 1, max_depth, buf, rng);
    }
    fn fragment_269(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_839(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_841(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_845(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_847(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_849(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_851(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_853(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_857(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_859(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_861(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_863(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_271(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_983(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_985(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_987(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_989(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_991(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_993(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_995(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_997(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_999(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1001(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1003(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1005(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1007(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1009(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1011(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1013(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1015(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1017(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1019(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1021(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1023(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1025(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1027(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1029(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1031(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1033(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_296(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_298(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_300(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2" */
        buf.push(50);
    }
    fn fragment_302(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3" */
        buf.push(51);
    }
    fn fragment_304(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_306(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5" */
        buf.push(53);
    }
    fn fragment_308(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "6" */
        buf.push(54);
    }
    fn fragment_310(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "7" */
        buf.push(55);
    }
    fn fragment_312(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_314(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9" */
        buf.push(57);
    }
    fn fragment_316(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_317(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_318(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_319(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_317(depth + 1, max_depth, buf, rng);
        Self::fragment_318(depth + 1, max_depth, buf, rng);
    }
    fn fragment_321(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_323(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_325(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_327(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_329(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "16" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_331(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "31" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_333(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "32" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 50].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_335(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "33" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 51].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_337(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "63" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [54, 51].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_339(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "64" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [54, 52].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_341(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "65" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [54, 53].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_343(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "255" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 53, 53].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_345(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "257" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 53, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_347(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "256" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 53, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_349(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "128" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 50, 56].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_351(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "127" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 50, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_353(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "129" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 50, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_355(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "65535" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [54, 53, 53, 51, 53].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_357(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "65537" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [54, 53, 53, 51, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_359(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "65536" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [54, 53, 53, 51, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_361(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "32768" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 50, 55, 54, 56].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_363(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "32767" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 50, 55, 54, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_365(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "32769" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 50, 55, 54, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_367(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4294967295" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 50, 57, 52, 57, 54, 55, 50, 57, 53].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_369(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4294967297" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 50, 57, 52, 57, 54, 55, 50, 57, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_371(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4294967296" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 50, 57, 52, 57, 54, 55, 50, 57, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_373(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2147483648" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 49, 52, 55, 52, 56, 51, 54, 52, 56].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_375(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2147483647" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 49, 52, 55, 52, 56, 51, 54, 52, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_377(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2147483649" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 49, 52, 55, 52, 56, 51, 54, 52, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_379(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "281474976710655" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 56, 49, 52, 55, 52, 57, 55, 54, 55, 49, 48, 54, 53, 53].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_381(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "281474976710657" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 56, 49, 52, 55, 52, 57, 55, 54, 55, 49, 48, 54, 53, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_383(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "281474976710656" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 56, 49, 52, 55, 52, 57, 55, 54, 55, 49, 48, 54, 53, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_385(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "140737488355328" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 52, 48, 55, 51, 55, 52, 56, 56, 51, 53, 53, 51, 50, 56].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_387(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "140737488355327" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 52, 48, 55, 51, 55, 52, 56, 56, 51, 53, 53, 51, 50, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_389(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "140737488355329" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 52, 48, 55, 51, 55, 52, 56, 56, 51, 53, 53, 51, 50, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_391(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "18446744073709551615" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54, 49, 53,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_393(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "18446744073709551617" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54, 49, 55,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_395(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "18446744073709551616" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 56, 52, 52, 54, 55, 52, 52, 48, 55, 51, 55, 48, 57, 53, 53, 49, 54, 49, 54,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_397(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9223372036854775808" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 56,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_399(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9223372036854775807" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 55,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_401(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9223372036854775809" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    57, 50, 50, 51, 51, 55, 50, 48, 51, 54, 56, 53, 52, 55, 55, 53, 56, 48, 57,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_403(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "340282366920938463463374607431768211455" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 39;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    51, 52, 48, 50, 56, 50, 51, 54, 54, 57, 50, 48, 57, 51, 56, 52, 54, 51, 52, 54,
                    51, 51, 55, 52, 54, 48, 55, 52, 51, 49, 55, 54, 56, 50, 49, 49, 52, 53, 53,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                39,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_405(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "340282366920938463463374607431768211457" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 39;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    51, 52, 48, 50, 56, 50, 51, 54, 54, 57, 50, 48, 57, 51, 56, 52, 54, 51, 52, 54,
                    51, 51, 55, 52, 54, 48, 55, 52, 51, 49, 55, 54, 56, 50, 49, 49, 52, 53, 55,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                39,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_407(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "340282366920938463463374607431768211456" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 39;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    51, 52, 48, 50, 56, 50, 51, 54, 54, 57, 50, 48, 57, 51, 56, 52, 54, 51, 52, 54,
                    51, 51, 55, 52, 54, 48, 55, 52, 51, 49, 55, 54, 56, 50, 49, 49, 52, 53, 54,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                39,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_409(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "170141183460469231731687303715884105728" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 39;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 55, 48, 49, 52, 49, 49, 56, 51, 52, 54, 48, 52, 54, 57, 50, 51, 49, 55, 51,
                    49, 54, 56, 55, 51, 48, 51, 55, 49, 53, 56, 56, 52, 49, 48, 53, 55, 50, 56,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                39,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_411(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "170141183460469231731687303715884105727" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 39;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 55, 48, 49, 52, 49, 49, 56, 51, 52, 54, 48, 52, 54, 57, 50, 51, 49, 55, 51,
                    49, 54, 56, 55, 51, 48, 51, 55, 49, 53, 56, 56, 52, 49, 48, 53, 55, 50, 55,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                39,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_413(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "170141183460469231731687303715884105729" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 39;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 55, 48, 49, 52, 49, 49, 56, 51, 52, 54, 48, 52, 54, 57, 50, 51, 49, 55, 51,
                    49, 54, 56, 55, 51, 48, 51, 55, 49, 53, 56, 56, 52, 49, 48, 53, 55, 50, 57,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                39,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_415(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "115792089237316195423570985008687907853269984665640564039457584007913129639935" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 78;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 49, 53, 55, 57, 50, 48, 56, 57, 50, 51, 55, 51, 49, 54, 49, 57, 53, 52, 50,
                    51, 53, 55, 48, 57, 56, 53, 48, 48, 56, 54, 56, 55, 57, 48, 55, 56, 53, 51, 50,
                    54, 57, 57, 56, 52, 54, 54, 53, 54, 52, 48, 53, 54, 52, 48, 51, 57, 52, 53, 55,
                    53, 56, 52, 48, 48, 55, 57, 49, 51, 49, 50, 57, 54, 51, 57, 57, 51, 53,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                78,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_417(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "115792089237316195423570985008687907853269984665640564039457584007913129639937" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 78;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 49, 53, 55, 57, 50, 48, 56, 57, 50, 51, 55, 51, 49, 54, 49, 57, 53, 52, 50,
                    51, 53, 55, 48, 57, 56, 53, 48, 48, 56, 54, 56, 55, 57, 48, 55, 56, 53, 51, 50,
                    54, 57, 57, 56, 52, 54, 54, 53, 54, 52, 48, 53, 54, 52, 48, 51, 57, 52, 53, 55,
                    53, 56, 52, 48, 48, 55, 57, 49, 51, 49, 50, 57, 54, 51, 57, 57, 51, 55,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                78,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_419(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "115792089237316195423570985008687907853269984665640564039457584007913129639936" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 78;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 49, 53, 55, 57, 50, 48, 56, 57, 50, 51, 55, 51, 49, 54, 49, 57, 53, 52, 50,
                    51, 53, 55, 48, 57, 56, 53, 48, 48, 56, 54, 56, 55, 57, 48, 55, 56, 53, 51, 50,
                    54, 57, 57, 56, 52, 54, 54, 53, 54, 52, 48, 53, 54, 52, 48, 51, 57, 52, 53, 55,
                    53, 56, 52, 48, 48, 55, 57, 49, 51, 49, 50, 57, 54, 51, 57, 57, 51, 54,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                78,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_421(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "57896044618658097711785492504343953926634992332820282019728792003956564819968" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 77;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 55, 56, 57, 54, 48, 52, 52, 54, 49, 56, 54, 53, 56, 48, 57, 55, 55, 49, 49,
                    55, 56, 53, 52, 57, 50, 53, 48, 52, 51, 52, 51, 57, 53, 51, 57, 50, 54, 54, 51,
                    52, 57, 57, 50, 51, 51, 50, 56, 50, 48, 50, 56, 50, 48, 49, 57, 55, 50, 56, 55,
                    57, 50, 48, 48, 51, 57, 53, 54, 53, 54, 52, 56, 49, 57, 57, 54, 56,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                77,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_423(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "57896044618658097711785492504343953926634992332820282019728792003956564819967" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 77;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 55, 56, 57, 54, 48, 52, 52, 54, 49, 56, 54, 53, 56, 48, 57, 55, 55, 49, 49,
                    55, 56, 53, 52, 57, 50, 53, 48, 52, 51, 52, 51, 57, 53, 51, 57, 50, 54, 54, 51,
                    52, 57, 57, 50, 51, 51, 50, 56, 50, 48, 50, 56, 50, 48, 49, 57, 55, 50, 56, 55,
                    57, 50, 48, 48, 51, 57, 53, 54, 53, 54, 52, 56, 49, 57, 57, 54, 55,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                77,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_425(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "57896044618658097711785492504343953926634992332820282019728792003956564819969" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 77;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 55, 56, 57, 54, 48, 52, 52, 54, 49, 56, 54, 53, 56, 48, 57, 55, 55, 49, 49,
                    55, 56, 53, 52, 57, 50, 53, 48, 52, 51, 52, 51, 57, 53, 51, 57, 50, 54, 54, 51,
                    52, 57, 57, 50, 51, 51, 50, 56, 50, 48, 50, 56, 50, 48, 49, 57, 55, 50, 56, 55,
                    57, 50, 48, 48, 51, 57, 53, 54, 53, 54, 52, 56, 49, 57, 57, 54, 57,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                77,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_426(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_427(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_428(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_429(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_426(depth + 1, max_depth, buf, rng);
        Self::fragment_427(depth + 1, max_depth, buf, rng);
        Self::fragment_428(depth + 1, max_depth, buf, rng);
    }
    fn fragment_430(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_431(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_432(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_433(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_434(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_430(depth + 1, max_depth, buf, rng);
        Self::fragment_431(depth + 1, max_depth, buf, rng);
        Self::fragment_432(depth + 1, max_depth, buf, rng);
        Self::fragment_433(depth + 1, max_depth, buf, rng);
    }
    fn fragment_435(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_436(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_437(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_438(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_435(depth + 1, max_depth, buf, rng);
        Self::fragment_436(depth + 1, max_depth, buf, rng);
        Self::fragment_437(depth + 1, max_depth, buf, rng);
    }
    fn fragment_439(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_440(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_441(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_442(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_443(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_439(depth + 1, max_depth, buf, rng);
        Self::fragment_440(depth + 1, max_depth, buf, rng);
        Self::fragment_441(depth + 1, max_depth, buf, rng);
        Self::fragment_442(depth + 1, max_depth, buf, rng);
    }
    fn fragment_444(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_445(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_446(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_447(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e" */
        buf.push(101);
    }
    fn fragment_448(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_449(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_444(depth + 1, max_depth, buf, rng);
        Self::fragment_445(depth + 1, max_depth, buf, rng);
        Self::fragment_446(depth + 1, max_depth, buf, rng);
        Self::fragment_447(depth + 1, max_depth, buf, rng);
        Self::fragment_448(depth + 1, max_depth, buf, rng);
    }
    fn fragment_450(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_451(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_452(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_453(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e+" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [101, 43].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_454(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_455(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_450(depth + 1, max_depth, buf, rng);
        Self::fragment_451(depth + 1, max_depth, buf, rng);
        Self::fragment_452(depth + 1, max_depth, buf, rng);
        Self::fragment_453(depth + 1, max_depth, buf, rng);
        Self::fragment_454(depth + 1, max_depth, buf, rng);
    }
    fn fragment_456(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_457(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_458(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_459(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e-" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [101, 45].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_460(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_461(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_456(depth + 1, max_depth, buf, rng);
        Self::fragment_457(depth + 1, max_depth, buf, rng);
        Self::fragment_458(depth + 1, max_depth, buf, rng);
        Self::fragment_459(depth + 1, max_depth, buf, rng);
        Self::fragment_460(depth + 1, max_depth, buf, rng);
    }
    fn fragment_463(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_465(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_467(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2" */
        buf.push(50);
    }
    fn fragment_469(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3" */
        buf.push(51);
    }
    fn fragment_471(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_473(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5" */
        buf.push(53);
    }
    fn fragment_475(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "6" */
        buf.push(54);
    }
    fn fragment_477(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "7" */
        buf.push(55);
    }
    fn fragment_479(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_481(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9" */
        buf.push(57);
    }
    fn fragment_483(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "A" */
        buf.push(65);
    }
    fn fragment_485(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "B" */
        buf.push(66);
    }
    fn fragment_487(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "C" */
        buf.push(67);
    }
    fn fragment_489(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "D" */
        buf.push(68);
    }
    fn fragment_491(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "E" */
        buf.push(69);
    }
    fn fragment_493(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "F" */
        buf.push(70);
    }
    fn fragment_495(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "a" */
        buf.push(97);
    }
    fn fragment_497(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "b" */
        buf.push(98);
    }
    fn fragment_499(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "c" */
        buf.push(99);
    }
    fn fragment_501(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "d" */
        buf.push(100);
    }
    fn fragment_503(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e" */
        buf.push(101);
    }
    fn fragment_505(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "f" */
        buf.push(102);
    }
    fn fragment_507(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_463(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_465(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_467(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_469(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_471(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_473(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_475(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_477(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_479(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_481(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_483(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_485(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_487(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_489(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_491(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_495(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_497(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_499(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_501(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_503(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_505(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_508(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_463(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_465(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_467(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_469(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_471(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_473(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_475(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_477(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_479(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_481(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_483(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_485(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_487(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_489(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_491(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_495(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_497(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_499(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_501(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_503(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_505(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_509(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_507(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_510(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_510(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_508(depth + 1, max_depth, buf, rng);
        Self::fragment_509(depth + 1, max_depth, buf, rng);
    }
    fn fragment_511(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0x" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 120].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_512(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_507(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_510(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_513(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_511(depth + 1, max_depth, buf, rng);
        Self::fragment_512(depth + 1, max_depth, buf, rng);
    }
    fn fragment_514(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0X" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 88].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_515(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_507(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_510(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_516(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_514(depth + 1, max_depth, buf, rng);
        Self::fragment_515(depth + 1, max_depth, buf, rng);
    }
    fn fragment_517(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_507(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_510(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_518(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "h" */
        buf.push(104);
    }
    fn fragment_519(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_517(depth + 1, max_depth, buf, rng);
        Self::fragment_518(depth + 1, max_depth, buf, rng);
    }
    fn fragment_521(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..53) {
            0 => Self::fragment_321(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_323(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_325(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_327(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_329(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_331(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_333(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_335(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_337(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_339(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_341(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_343(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_345(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_347(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_349(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_351(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_353(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_355(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_357(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_359(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_361(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_363(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_365(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_367(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_369(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_371(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_373(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_375(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_377(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_379(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_381(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_383(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_385(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_387(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_389(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_391(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_393(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_395(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_397(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_399(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_401(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_403(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_405(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_407(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_409(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_411(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_413(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_415(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_417(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_419(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_421(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_423(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_425(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_523(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_525(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_527(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_529(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_531(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "16" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_533(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1234" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 50, 51, 52].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_535(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0001234" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 48, 48, 49, 50, 51, 52].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_537(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0xdeadbeef" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 120, 100, 101, 97, 100, 98, 101, 101, 102].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_539(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0DEADBEEF" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 68, 69, 65, 68, 66, 69, 69, 70].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_541(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0_1234_45667" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 95, 49, 50, 51, 52, 95, 52, 53, 54, 54, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_543(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [45, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_545(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-9" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [45, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_547(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1234567890" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 50, 51, 52, 53, 54, 55, 56, 57, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_549(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0123456789" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 49, 50, 51, 52, 53, 54, 55, 56, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_551(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_521(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_523(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_553(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_582(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_585(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_587(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_590(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_555(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_557(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_429(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_434(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_438(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_443(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_449(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_455(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_461(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_559(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_513(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_516(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_519(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_582(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..53) {
            0 => Self::fragment_321(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_323(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_325(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_327(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_329(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_331(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_333(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_335(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_337(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_339(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_341(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_343(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_345(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_347(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_349(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_351(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_353(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_355(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_357(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_359(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_361(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_363(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_365(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_367(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_369(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_371(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_373(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_375(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_377(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_379(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_381(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_383(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_385(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_387(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_389(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_391(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_393(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_395(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_397(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_399(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_401(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_403(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_405(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_407(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_409(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_411(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_413(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_415(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_417(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_419(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_421(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_423(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_425(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_583(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_584(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..53) {
            0 => Self::fragment_321(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_323(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_325(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_327(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_329(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_331(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_333(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_335(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_337(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_339(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_341(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_343(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_345(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_347(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_349(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_351(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_353(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_355(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_357(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_359(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_361(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_363(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_365(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_367(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_369(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_371(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_373(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_375(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_377(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_379(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_381(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_383(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_385(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_387(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_389(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_391(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_393(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_395(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_397(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_399(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_401(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_403(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_405(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_407(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_409(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_411(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_413(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_415(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_417(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_419(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_421(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_423(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_425(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_585(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_583(depth + 1, max_depth, buf, rng);
        Self::fragment_584(depth + 1, max_depth, buf, rng);
    }
    fn fragment_587(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_588(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_589(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_590(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_588(depth + 1, max_depth, buf, rng);
        Self::fragment_589(depth + 1, max_depth, buf, rng);
    }
    fn fragment_591(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_593(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "+" */
        buf.push(43);
    }
    fn fragment_595(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_597(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_599(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "==" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [61, 61].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_601(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_269(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_271(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_591(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_593(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_595(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_602(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_269(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_271(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_591(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_593(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_595(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_603(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_597(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_599(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_604(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_602(depth + 1, max_depth, buf, rng);
        Self::fragment_603(depth + 1, max_depth, buf, rng);
    }
    fn fragment_605(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_269(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_271(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_591(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_593(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_595(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_606(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_601(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_604(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_607(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_607(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_605(depth + 1, max_depth, buf, rng);
        Self::fragment_606(depth + 1, max_depth, buf, rng);
    }
    fn fragment_609(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_839(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_841(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_845(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_847(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_849(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_851(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_853(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_857(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_859(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_861(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_863(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_611(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_983(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_985(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_987(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_989(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_991(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_993(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_995(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_997(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_999(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1001(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1003(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1005(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1007(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1009(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1011(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1013(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1015(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1017(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1019(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1021(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1023(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1025(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1027(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1029(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1031(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1033(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_613(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_736(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_738(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_740(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_742(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_744(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_746(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_748(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_750(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_752(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_754(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_615(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!" */
        buf.push(33);
    }
    fn fragment_617(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "#" */
        buf.push(35);
    }
    fn fragment_619(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "$" */
        buf.push(36);
    }
    fn fragment_621(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "%" */
        buf.push(37);
    }
    fn fragment_623(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_625(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\"" */
        buf.push(34);
    }
    fn fragment_627(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "(" */
        buf.push(40);
    }
    fn fragment_629(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ")" */
        buf.push(41);
    }
    fn fragment_631(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        buf.push(42);
    }
    fn fragment_633(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "+" */
        buf.push(43);
    }
    fn fragment_635(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_637(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_639(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_641(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_643(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_645(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_647(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "<" */
        buf.push(60);
    }
    fn fragment_649(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_651(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ">" */
        buf.push(62);
    }
    fn fragment_653(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_655(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "@" */
        buf.push(64);
    }
    fn fragment_657(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "[" */
        buf.push(91);
    }
    fn fragment_659(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "]" */
        buf.push(93);
    }
    fn fragment_661(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "^" */
        buf.push(94);
    }
    fn fragment_663(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "_" */
        buf.push(95);
    }
    fn fragment_665(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "`" */
        buf.push(96);
    }
    fn fragment_667(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "{" */
        buf.push(123);
    }
    fn fragment_669(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "|" */
        buf.push(124);
    }
    fn fragment_671(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "}" */
        buf.push(125);
    }
    fn fragment_673(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "~" */
        buf.push(126);
    }
    fn fragment_675(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_676(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
    }
    fn fragment_678(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_679(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_680(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_676(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_678(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_681(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_688(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_681(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_679(depth + 1, max_depth, buf, rng);
        Self::fragment_680(depth + 1, max_depth, buf, rng);
    }
    fn fragment_682(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_683(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_684(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_685(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_686(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_696(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_687(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_676(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_678(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_681(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_688(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_688(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_682(depth + 1, max_depth, buf, rng);
        Self::fragment_683(depth + 1, max_depth, buf, rng);
        Self::fragment_684(depth + 1, max_depth, buf, rng);
        Self::fragment_685(depth + 1, max_depth, buf, rng);
        Self::fragment_686(depth + 1, max_depth, buf, rng);
        Self::fragment_687(depth + 1, max_depth, buf, rng);
    }
    fn fragment_690(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_839(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_841(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_845(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_847(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_849(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_851(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_853(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_857(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_859(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_861(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_863(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_692(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_983(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_985(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_987(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_989(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_991(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_993(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_995(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_997(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_999(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1001(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1003(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1005(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1007(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1009(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1011(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1013(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1015(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1017(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1019(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1021(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1023(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1025(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1027(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1029(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1031(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1033(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_694(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_696(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_697(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_839(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_841(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_845(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_847(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_849(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_851(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_853(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_857(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_859(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_861(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_863(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_698(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_676(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_678(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_681(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_688(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_699(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_697(depth + 1, max_depth, buf, rng);
        Self::fragment_698(depth + 1, max_depth, buf, rng);
    }
    fn fragment_700(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_983(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_985(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_987(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_989(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_991(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_993(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_995(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_997(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_999(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1001(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1003(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1005(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1007(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1009(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1011(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1013(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1015(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1017(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1019(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1021(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1023(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1025(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1027(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1029(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1031(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1033(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_701(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_676(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_678(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_681(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_688(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_702(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_700(depth + 1, max_depth, buf, rng);
        Self::fragment_701(depth + 1, max_depth, buf, rng);
    }
    fn fragment_704(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sun, 06 Nov 1994 08:49:37 GMT" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 29;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 117, 110, 44, 32, 48, 54, 32, 78, 111, 118, 32, 49, 57, 57, 52, 32, 48, 56,
                    58, 52, 57, 58, 51, 55, 32, 71, 77, 84,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                29,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_706(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sun, 06 Nov 2094 08:49:37 GMT" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 29;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 117, 110, 44, 32, 48, 54, 32, 78, 111, 118, 32, 50, 48, 57, 52, 32, 48, 56,
                    58, 52, 57, 58, 51, 55, 32, 71, 77, 84,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                29,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_708(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sun, 06 Nov 1900 08:49:37 GMT" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 29;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 117, 110, 44, 32, 48, 54, 32, 78, 111, 118, 32, 49, 57, 48, 48, 32, 48, 56,
                    58, 52, 57, 58, 51, 55, 32, 71, 77, 84,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                29,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_710(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sun, 45 Nov 2056 08:49:37 GMT" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 29;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 117, 110, 44, 32, 52, 53, 32, 78, 111, 118, 32, 50, 48, 53, 54, 32, 48, 56,
                    58, 52, 57, 58, 51, 55, 32, 71, 77, 84,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                29,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_712(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sun, 15 Nov 2006 08:49:37 GMT+10" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 32;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 117, 110, 44, 32, 49, 53, 32, 78, 111, 118, 32, 50, 48, 48, 54, 32, 48, 56,
                    58, 52, 57, 58, 51, 55, 32, 71, 77, 84, 43, 49, 48,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                32,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_714(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sun, 15 Nov 2006 08:49:37 GMT+16" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 32;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 117, 110, 44, 32, 49, 53, 32, 78, 111, 118, 32, 50, 48, 48, 54, 32, 48, 56,
                    58, 52, 57, 58, 51, 55, 32, 71, 77, 84, 43, 49, 54,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                32,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_716(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sun, 11 Nov 1111 11:11:11 GMT+11" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 32;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 117, 110, 44, 32, 49, 49, 32, 78, 111, 118, 32, 49, 49, 49, 49, 32, 49, 49,
                    58, 49, 49, 58, 49, 49, 32, 71, 77, 84, 43, 49, 49,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                32,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_717(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_881(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_883(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_885(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_887(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_889(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_891(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_893(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_895(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_897(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_718(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_719(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_720(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_721(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_263(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_267(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_722(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_1086(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1088(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1090(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1092(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1094(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1096(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1098(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1103(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1105(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_723(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_949(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_951(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_953(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_955(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_962(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_971(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_724(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_973(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_975(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_978(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_981(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_725(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_717(depth + 1, max_depth, buf, rng);
        Self::fragment_718(depth + 1, max_depth, buf, rng);
        Self::fragment_719(depth + 1, max_depth, buf, rng);
        Self::fragment_720(depth + 1, max_depth, buf, rng);
        Self::fragment_721(depth + 1, max_depth, buf, rng);
        Self::fragment_722(depth + 1, max_depth, buf, rng);
        Self::fragment_723(depth + 1, max_depth, buf, rng);
        Self::fragment_724(depth + 1, max_depth, buf, rng);
    }
    fn fragment_726(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_263(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_267(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_727(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_728(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_729(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_730(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_263(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_267(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_731(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_1086(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1088(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1090(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1092(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1094(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1096(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1098(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1103(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1105(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_732(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_949(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_951(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_953(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_955(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_962(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_971(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_733(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_263(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_267(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_734(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_726(depth + 1, max_depth, buf, rng);
        Self::fragment_727(depth + 1, max_depth, buf, rng);
        Self::fragment_728(depth + 1, max_depth, buf, rng);
        Self::fragment_729(depth + 1, max_depth, buf, rng);
        Self::fragment_730(depth + 1, max_depth, buf, rng);
        Self::fragment_731(depth + 1, max_depth, buf, rng);
        Self::fragment_732(depth + 1, max_depth, buf, rng);
        Self::fragment_733(depth + 1, max_depth, buf, rng);
    }
    fn fragment_736(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_738(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_740(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2" */
        buf.push(50);
    }
    fn fragment_742(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3" */
        buf.push(51);
    }
    fn fragment_744(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_746(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5" */
        buf.push(53);
    }
    fn fragment_748(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "6" */
        buf.push(54);
    }
    fn fragment_750(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "7" */
        buf.push(55);
    }
    fn fragment_752(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_754(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9" */
        buf.push(57);
    }
    fn fragment_815(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "a" */
        buf.push(97);
    }
    fn fragment_817(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "b" */
        buf.push(98);
    }
    fn fragment_819(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "c" */
        buf.push(99);
    }
    fn fragment_821(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "d" */
        buf.push(100);
    }
    fn fragment_823(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e" */
        buf.push(101);
    }
    fn fragment_825(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "f" */
        buf.push(102);
    }
    fn fragment_827(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "g" */
        buf.push(103);
    }
    fn fragment_829(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "h" */
        buf.push(104);
    }
    fn fragment_831(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "i" */
        buf.push(105);
    }
    fn fragment_833(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "j" */
        buf.push(106);
    }
    fn fragment_835(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "k" */
        buf.push(107);
    }
    fn fragment_837(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "l" */
        buf.push(108);
    }
    fn fragment_839(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "m" */
        buf.push(109);
    }
    fn fragment_841(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "n" */
        buf.push(110);
    }
    fn fragment_843(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "o" */
        buf.push(111);
    }
    fn fragment_845(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "p" */
        buf.push(112);
    }
    fn fragment_847(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "q" */
        buf.push(113);
    }
    fn fragment_849(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "r" */
        buf.push(114);
    }
    fn fragment_851(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "s" */
        buf.push(115);
    }
    fn fragment_853(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "t" */
        buf.push(116);
    }
    fn fragment_855(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "u" */
        buf.push(117);
    }
    fn fragment_857(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "v" */
        buf.push(118);
    }
    fn fragment_859(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "w" */
        buf.push(119);
    }
    fn fragment_861(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "x" */
        buf.push(120);
    }
    fn fragment_863(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "y" */
        buf.push(121);
    }
    fn fragment_865(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "z" */
        buf.push(122);
    }
    fn fragment_881(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Jan" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [74, 97, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_883(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Feb" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [70, 101, 98].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_885(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Mar" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [77, 97, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_887(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Apr" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 112, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_889(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "May" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [77, 97, 121].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_891(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Jun" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [74, 117, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_893(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Jul" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [74, 117, 108].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_895(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Aug" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 117, 103].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_897(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sep" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 101, 112].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_899(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Dec" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 101, 99].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_900(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
    }
    fn fragment_902(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_904(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "    " */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [32, 32, 32, 32].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_906(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "         " */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [32, 32, 32, 32, 32, 32, 32, 32, 32].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_908(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "                " */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_910(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "                 " */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_911(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_912(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_913(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_911(depth + 1, max_depth, buf, rng);
        Self::fragment_912(depth + 1, max_depth, buf, rng);
    }
    fn fragment_915(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_916(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_917(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_918(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_919(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_920(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_921(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_922(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_923(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_924(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_916(depth + 1, max_depth, buf, rng);
        Self::fragment_917(depth + 1, max_depth, buf, rng);
        Self::fragment_918(depth + 1, max_depth, buf, rng);
        Self::fragment_919(depth + 1, max_depth, buf, rng);
        Self::fragment_920(depth + 1, max_depth, buf, rng);
        Self::fragment_921(depth + 1, max_depth, buf, rng);
        Self::fragment_922(depth + 1, max_depth, buf, rng);
        Self::fragment_923(depth + 1, max_depth, buf, rng);
    }
    fn fragment_925(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_926(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_927(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_928(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_929(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_930(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_931(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_932(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_933(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_934(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_925(depth + 1, max_depth, buf, rng);
        Self::fragment_926(depth + 1, max_depth, buf, rng);
        Self::fragment_927(depth + 1, max_depth, buf, rng);
        Self::fragment_928(depth + 1, max_depth, buf, rng);
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
        Self::fragment_932(depth + 1, max_depth, buf, rng);
        Self::fragment_933(depth + 1, max_depth, buf, rng);
    }
    fn fragment_935(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_936(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_937(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_935(depth + 1, max_depth, buf, rng);
        Self::fragment_936(depth + 1, max_depth, buf, rng);
    }
    fn fragment_938(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_939(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_940(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_941(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_942(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_943(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_944(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_945(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_609(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_611(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_613(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_615(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_617(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_619(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_621(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_623(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_625(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_627(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_629(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_631(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_633(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_635(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_637(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_639(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_641(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_643(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_645(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_647(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_649(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_651(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_653(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_655(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_657(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_659(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_661(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_663(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_665(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_667(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_669(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_671(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_673(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_946(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_947(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_938(depth + 1, max_depth, buf, rng);
        Self::fragment_939(depth + 1, max_depth, buf, rng);
        Self::fragment_940(depth + 1, max_depth, buf, rng);
        Self::fragment_941(depth + 1, max_depth, buf, rng);
        Self::fragment_942(depth + 1, max_depth, buf, rng);
        Self::fragment_943(depth + 1, max_depth, buf, rng);
        Self::fragment_944(depth + 1, max_depth, buf, rng);
        Self::fragment_945(depth + 1, max_depth, buf, rng);
        Self::fragment_946(depth + 1, max_depth, buf, rng);
    }
    fn fragment_949(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "08:49:37" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 56, 58, 52, 57, 58, 51, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_951(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "00:00:00" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 48, 58, 48, 48, 58, 48, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_953(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "01:01:01" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 49, 58, 48, 49, 58, 48, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_955(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "11:11:11" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 49, 58, 49, 49, 58, 49, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_956(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_957(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_958(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_959(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_960(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_961(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_962(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_956(depth + 1, max_depth, buf, rng);
        Self::fragment_957(depth + 1, max_depth, buf, rng);
        Self::fragment_958(depth + 1, max_depth, buf, rng);
        Self::fragment_959(depth + 1, max_depth, buf, rng);
        Self::fragment_960(depth + 1, max_depth, buf, rng);
        Self::fragment_961(depth + 1, max_depth, buf, rng);
    }
    fn fragment_963(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_964(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_965(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_966(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_967(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_968(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_969(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_970(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_971(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_963(depth + 1, max_depth, buf, rng);
        Self::fragment_964(depth + 1, max_depth, buf, rng);
        Self::fragment_965(depth + 1, max_depth, buf, rng);
        Self::fragment_966(depth + 1, max_depth, buf, rng);
        Self::fragment_967(depth + 1, max_depth, buf, rng);
        Self::fragment_968(depth + 1, max_depth, buf, rng);
        Self::fragment_969(depth + 1, max_depth, buf, rng);
        Self::fragment_970(depth + 1, max_depth, buf, rng);
    }
    fn fragment_973(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "GMT" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [71, 77, 84].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_975(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "UTC" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [85, 84, 67].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_976(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "GMT+" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [71, 77, 84, 43].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_977(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_978(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_976(depth + 1, max_depth, buf, rng);
        Self::fragment_977(depth + 1, max_depth, buf, rng);
    }
    fn fragment_979(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "UTC+" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [85, 84, 67, 43].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_980(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_981(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_979(depth + 1, max_depth, buf, rng);
        Self::fragment_980(depth + 1, max_depth, buf, rng);
    }
    fn fragment_983(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "A" */
        buf.push(65);
    }
    fn fragment_985(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "B" */
        buf.push(66);
    }
    fn fragment_987(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "C" */
        buf.push(67);
    }
    fn fragment_989(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "D" */
        buf.push(68);
    }
    fn fragment_991(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "E" */
        buf.push(69);
    }
    fn fragment_993(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "F" */
        buf.push(70);
    }
    fn fragment_995(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "G" */
        buf.push(71);
    }
    fn fragment_997(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "H" */
        buf.push(72);
    }
    fn fragment_999(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "I" */
        buf.push(73);
    }
    fn fragment_1001(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "J" */
        buf.push(74);
    }
    fn fragment_1003(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "K" */
        buf.push(75);
    }
    fn fragment_1005(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "L" */
        buf.push(76);
    }
    fn fragment_1007(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "M" */
        buf.push(77);
    }
    fn fragment_1009(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "N" */
        buf.push(78);
    }
    fn fragment_1011(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "O" */
        buf.push(79);
    }
    fn fragment_1013(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "P" */
        buf.push(80);
    }
    fn fragment_1015(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Q" */
        buf.push(81);
    }
    fn fragment_1017(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "R" */
        buf.push(82);
    }
    fn fragment_1019(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "S" */
        buf.push(83);
    }
    fn fragment_1021(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "T" */
        buf.push(84);
    }
    fn fragment_1023(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "U" */
        buf.push(85);
    }
    fn fragment_1025(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "V" */
        buf.push(86);
    }
    fn fragment_1027(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "W" */
        buf.push(87);
    }
    fn fragment_1029(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "X" */
        buf.push(88);
    }
    fn fragment_1031(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Y" */
        buf.push(89);
    }
    fn fragment_1033(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Z" */
        buf.push(90);
    }
    fn fragment_1035(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_839(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_841(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_845(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_847(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_849(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_851(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_853(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_857(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_859(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_861(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_863(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1037(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_983(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_985(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_987(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_989(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_991(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_993(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_995(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_997(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_999(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1001(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1003(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1005(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1007(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1009(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1011(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1013(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1015(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1017(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1019(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1021(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1023(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1025(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1027(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1029(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1031(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1033(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1039(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1041(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_1043(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1044(depth + 1, max_depth, buf, rng);
        Self::fragment_1045(depth + 1, max_depth, buf, rng);
        Self::fragment_1046(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1044(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "%" */
        buf.push(37);
    }
    fn fragment_1045(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_463(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_465(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_467(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_469(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_471(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_473(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_475(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_477(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_479(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_481(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_483(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_485(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_487(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_489(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_491(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_495(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_497(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_499(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_501(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_503(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_505(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1046(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_463(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_465(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_467(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_469(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_471(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_473(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_475(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_477(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_479(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_481(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_483(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_485(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_487(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_489(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_491(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_495(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_497(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_499(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_501(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_503(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_505(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1049(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1050(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1051(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1052(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1053(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1054(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1055(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1056(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1057(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1058(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1050(depth + 1, max_depth, buf, rng);
        Self::fragment_1051(depth + 1, max_depth, buf, rng);
        Self::fragment_1052(depth + 1, max_depth, buf, rng);
        Self::fragment_1053(depth + 1, max_depth, buf, rng);
        Self::fragment_1054(depth + 1, max_depth, buf, rng);
        Self::fragment_1055(depth + 1, max_depth, buf, rng);
        Self::fragment_1056(depth + 1, max_depth, buf, rng);
        Self::fragment_1057(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1059(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1060(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1049(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1058(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1061(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1071(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1061(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1059(depth + 1, max_depth, buf, rng);
        Self::fragment_1060(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1062(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1063(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1064(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1065(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1066(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1067(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1068(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1069(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1035(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1037(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1039(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1041(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1043(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1070(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1049(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1058(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1061(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1071(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1071(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1062(depth + 1, max_depth, buf, rng);
        Self::fragment_1063(depth + 1, max_depth, buf, rng);
        Self::fragment_1064(depth + 1, max_depth, buf, rng);
        Self::fragment_1065(depth + 1, max_depth, buf, rng);
        Self::fragment_1066(depth + 1, max_depth, buf, rng);
        Self::fragment_1067(depth + 1, max_depth, buf, rng);
        Self::fragment_1068(depth + 1, max_depth, buf, rng);
        Self::fragment_1069(depth + 1, max_depth, buf, rng);
        Self::fragment_1070(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1086(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1991" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 57, 57, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1088(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1091" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 48, 57, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1090(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2091" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 48, 57, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1092(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9999" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [57, 57, 57, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1094(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "99999" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [57, 57, 57, 57, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1096(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1098(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-1999" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [45, 49, 57, 57, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1099(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1100(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1101(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1102(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1103(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1099(depth + 1, max_depth, buf, rng);
        Self::fragment_1100(depth + 1, max_depth, buf, rng);
        Self::fragment_1101(depth + 1, max_depth, buf, rng);
        Self::fragment_1102(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1105(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1106(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1107(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1108(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_1109(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1106(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1110(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1110(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1107(depth + 1, max_depth, buf, rng);
        Self::fragment_1108(depth + 1, max_depth, buf, rng);
        Self::fragment_1109(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1112(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "example.com" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1114(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "sub.example.com" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    115, 117, 98, 46, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1116(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "subsub.sub.example.com" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    115, 117, 98, 115, 117, 98, 46, 115, 117, 98, 46, 101, 120, 97, 109, 112, 108,
                    101, 46, 99, 111, 109,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1118(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "a.b.c.d.f.g.h.i.j.k.example.com" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 31;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    97, 46, 98, 46, 99, 46, 100, 46, 102, 46, 103, 46, 104, 46, 105, 46, 106, 46,
                    107, 46, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                31,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1120(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1106(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1110(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1122(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1123(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1124(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_1125(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1122(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1126(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1126(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1123(depth + 1, max_depth, buf, rng);
        Self::fragment_1124(depth + 1, max_depth, buf, rng);
        Self::fragment_1125(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1130(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
        Self::fragment_1137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1131(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
        Self::fragment_1137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1132(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_1133(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1130(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1134(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1134(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1131(depth + 1, max_depth, buf, rng);
        Self::fragment_1132(depth + 1, max_depth, buf, rng);
        Self::fragment_1133(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1135(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1049(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1058(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1061(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1071(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1136(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_1137(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1140(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1142(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1140(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_601(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_604(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_607(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1142(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1049(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1058(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1061(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1071(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1143(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_1144(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
        Self::fragment_1137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1145(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1143(depth + 1, max_depth, buf, rng);
        Self::fragment_1144(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1146(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_1147(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
        Self::fragment_1137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1148(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_1149(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
        Self::fragment_1137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1150(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1146(depth + 1, max_depth, buf, rng);
        Self::fragment_1147(depth + 1, max_depth, buf, rng);
        Self::fragment_1148(depth + 1, max_depth, buf, rng);
        Self::fragment_1149(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1151(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_1152(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
        Self::fragment_1137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1153(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_1154(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
        Self::fragment_1137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1155(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_1156(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
        Self::fragment_1137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1157(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1151(depth + 1, max_depth, buf, rng);
        Self::fragment_1152(depth + 1, max_depth, buf, rng);
        Self::fragment_1153(depth + 1, max_depth, buf, rng);
        Self::fragment_1154(depth + 1, max_depth, buf, rng);
        Self::fragment_1155(depth + 1, max_depth, buf, rng);
        Self::fragment_1156(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1158(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_1159(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1130(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1134(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1160(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1158(depth + 1, max_depth, buf, rng);
        Self::fragment_1159(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1162(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [104, 116, 116, 112, 58, 47, 47].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1164(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "https://" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [104, 116, 116, 112, 115, 58, 47, 47].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1166(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "ftp://" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [102, 116, 112, 58, 47, 47].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1168(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "file://" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [102, 105, 108, 101, 58, 47, 47].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1170(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_221(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_223(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_225(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_227(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_229(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_231(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1171(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1162(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1164(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1166(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1168(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1172(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1112(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1114(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1116(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1118(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1120(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1173(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_1174(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1122(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1126(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1175(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1171(depth + 1, max_depth, buf, rng);
        Self::fragment_1172(depth + 1, max_depth, buf, rng);
        Self::fragment_1173(depth + 1, max_depth, buf, rng);
        Self::fragment_1174(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1176(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1162(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1164(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1166(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1168(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1177(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1112(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1114(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1116(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1118(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1120(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1178(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_1179(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1122(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1126(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1180(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1145(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1150(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1157(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1160(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1181(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1176(depth + 1, max_depth, buf, rng);
        Self::fragment_1177(depth + 1, max_depth, buf, rng);
        Self::fragment_1178(depth + 1, max_depth, buf, rng);
        Self::fragment_1179(depth + 1, max_depth, buf, rng);
        Self::fragment_1180(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1189(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept-Charset" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 99, 99, 101, 112, 116, 45, 67, 104, 97, 114, 115, 101, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1190(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1191(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1192(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1193(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1401(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1407(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1194(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1196(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept-Encoding:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1197(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1198(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1592(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1596(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1199(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1201(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept-Language" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 99, 99, 101, 112, 116, 45, 76, 97, 110, 103, 117, 97, 103, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1202(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1203(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1204(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1205(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1924(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1928(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1206(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1208(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept-Ranges" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 99, 99, 101, 112, 116, 45, 82, 97, 110, 103, 101, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1209(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1210(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1211(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1212(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2209(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2211(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1213(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1215(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_1991(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1993(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1995(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1997(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1999(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2001(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2003(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2005(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2007(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2009(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2011(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2013(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2015(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2017(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2019(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2021(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2023(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2025(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1216(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1217(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2182(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2184(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2189(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2192(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1220(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1215(depth + 1, max_depth, buf, rng);
        Self::fragment_1216(depth + 1, max_depth, buf, rng);
        Self::fragment_1217(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1221(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1215(depth + 1, max_depth, buf, rng);
        Self::fragment_1216(depth + 1, max_depth, buf, rng);
        Self::fragment_1217(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1222(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1223(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1215(depth + 1, max_depth, buf, rng);
        Self::fragment_1216(depth + 1, max_depth, buf, rng);
        Self::fragment_1217(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1224(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1221(depth + 1, max_depth, buf, rng);
        Self::fragment_1222(depth + 1, max_depth, buf, rng);
        Self::fragment_1223(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1225(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 99, 99, 101, 112, 116, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1226(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1227(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1220(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1224(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1228(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1229(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1238(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Allow" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 108, 108, 111, 119].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1239(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1240(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1241(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1242(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1983(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1987(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1243(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1244(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1238(depth + 1, max_depth, buf, rng);
        Self::fragment_1239(depth + 1, max_depth, buf, rng);
        Self::fragment_1240(depth + 1, max_depth, buf, rng);
        Self::fragment_1241(depth + 1, max_depth, buf, rng);
        Self::fragment_1242(depth + 1, max_depth, buf, rng);
        Self::fragment_1243(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1245(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Allow:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 108, 108, 111, 119, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1246(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1247(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1983(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1987(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1248(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1249(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1245(depth + 1, max_depth, buf, rng);
        Self::fragment_1246(depth + 1, max_depth, buf, rng);
        Self::fragment_1247(depth + 1, max_depth, buf, rng);
        Self::fragment_1248(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1250(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "ALPN" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 76, 80, 78].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1251(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1252(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1253(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1254(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2174(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1255(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1256(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1250(depth + 1, max_depth, buf, rng);
        Self::fragment_1251(depth + 1, max_depth, buf, rng);
        Self::fragment_1252(depth + 1, max_depth, buf, rng);
        Self::fragment_1253(depth + 1, max_depth, buf, rng);
        Self::fragment_1254(depth + 1, max_depth, buf, rng);
        Self::fragment_1255(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1257(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "ALPN:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 76, 80, 78, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1258(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1259(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2174(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1260(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1261(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1257(depth + 1, max_depth, buf, rng);
        Self::fragment_1258(depth + 1, max_depth, buf, rng);
        Self::fragment_1259(depth + 1, max_depth, buf, rng);
        Self::fragment_1260(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1264(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Alt-Used" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 108, 116, 45, 85, 115, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1265(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1266(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1267(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1268(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1175(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1181(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1269(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1270(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1264(depth + 1, max_depth, buf, rng);
        Self::fragment_1265(depth + 1, max_depth, buf, rng);
        Self::fragment_1266(depth + 1, max_depth, buf, rng);
        Self::fragment_1267(depth + 1, max_depth, buf, rng);
        Self::fragment_1268(depth + 1, max_depth, buf, rng);
        Self::fragment_1269(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1271(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Alt-Used:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 108, 116, 45, 85, 115, 101, 100, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1272(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1273(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1175(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1181(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1274(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1275(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1271(depth + 1, max_depth, buf, rng);
        Self::fragment_1272(depth + 1, max_depth, buf, rng);
        Self::fragment_1273(depth + 1, max_depth, buf, rng);
        Self::fragment_1274(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1277(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Basic" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [66, 97, 115, 105, 99].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1279(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Bearer" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [66, 101, 97, 114, 101, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1281(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Digest" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 105, 103, 101, 115, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1283(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "HOBA" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [72, 79, 66, 65].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1285(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Mutual" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [77, 117, 116, 117, 97, 108].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1287(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Negotiate" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [78, 101, 103, 111, 116, 105, 97, 116, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1289(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OAuth" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 65, 117, 116, 104].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1291(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "SCRAM-SHA-1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 67, 82, 65, 77, 45, 83, 72, 65, 45, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1293(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "SCRAM-SHA-256" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 67, 82, 65, 77, 45, 83, 72, 65, 45, 50, 53, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1295(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "vapid" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [118, 97, 112, 105, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1296(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Authorization" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 117, 116, 104, 111, 114, 105, 122, 97, 116, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1297(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1298(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1299(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_1277(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1279(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1281(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1283(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1285(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1287(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1289(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1291(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1293(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1295(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1300(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1301(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1532(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1534(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1536(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1302(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1303(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1306(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1449(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1454(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1308(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1049(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1058(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1061(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1071(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1310(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1312(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_601(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_604(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_607(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1314(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "T" */
        buf.push(84);
    }
    fn fragment_1316(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "F" */
        buf.push(70);
    }
    fn fragment_1319(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Cache-Control:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 97, 99, 104, 101, 45, 67, 111, 110, 116, 114, 111, 108, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1320(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1321(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1339(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1345(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1322(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1325(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "max-age=5" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [109, 97, 120, 45, 97, 103, 101, 61, 53].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1327(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "max-stale=5" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [109, 97, 120, 45, 115, 116, 97, 108, 101, 61, 53].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1329(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "min-fresh=5" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [109, 105, 110, 45, 102, 114, 101, 115, 104, 61, 53].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1331(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "no-cache" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [110, 111, 45, 99, 97, 99, 104, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1333(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "no-store" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [110, 111, 45, 115, 116, 111, 114, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1335(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "no-transform" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [110, 111, 45, 116, 114, 97, 110, 115, 102, 111, 114, 109].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1337(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "only-if-cached" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    111, 110, 108, 121, 45, 105, 102, 45, 99, 97, 99, 104, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1339(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1325(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1327(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1329(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1331(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1333(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1335(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1337(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1340(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1325(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1327(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1329(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1331(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1333(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1335(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1337(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1341(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1342(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1343(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1344(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1325(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1327(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1329(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1331(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1333(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1335(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1337(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1345(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1340(depth + 1, max_depth, buf, rng);
        Self::fragment_1341(depth + 1, max_depth, buf, rng);
        Self::fragment_1342(depth + 1, max_depth, buf, rng);
        Self::fragment_1343(depth + 1, max_depth, buf, rng);
        Self::fragment_1344(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1346(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "CalDav-Timezones" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 97, 108, 68, 97, 118, 45, 84, 105, 109, 101, 122, 111, 110, 101, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1347(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1348(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1349(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1350(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1314(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1316(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1351(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1354(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "foo123.foocdn.example" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    102, 111, 111, 49, 50, 51, 46, 102, 111, 111, 99, 100, 110, 46, 101, 120, 97,
                    109, 112, 108, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1356(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "barcdn.example; trace=\"abcdef\"" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 30;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    98, 97, 114, 99, 100, 110, 46, 101, 120, 97, 109, 112, 108, 101, 59, 32, 116,
                    114, 97, 99, 101, 61, 34, 97, 98, 99, 100, 101, 102, 34,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                30,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1358(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "AnotherCDN; abc=123; def=\"456\"" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 30;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 110, 111, 116, 104, 101, 114, 67, 68, 78, 59, 32, 97, 98, 99, 61, 49, 50,
                    51, 59, 32, 100, 101, 102, 61, 34, 52, 53, 54, 34,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                30,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1360(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1112(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1114(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1116(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1118(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1120(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1361(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1112(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1114(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1116(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1118(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1120(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1362(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1363(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "a=1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [97, 61, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1364(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1361(depth + 1, max_depth, buf, rng);
        Self::fragment_1362(depth + 1, max_depth, buf, rng);
        Self::fragment_1363(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1366(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1354(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1356(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1358(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1360(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1364(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1367(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1354(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1356(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1358(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1360(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1364(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1368(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1369(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1354(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1356(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1358(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1360(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1364(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1370(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1367(depth + 1, max_depth, buf, rng);
        Self::fragment_1368(depth + 1, max_depth, buf, rng);
        Self::fragment_1369(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1371(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "CND-Loop" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [67, 78, 68, 45, 76, 111, 111, 112].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1372(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1373(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1374(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1375(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1366(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1370(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1376(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1379(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "utf-16" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [117, 116, 102, 45, 49, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1381(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "utf-16BE" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [117, 116, 102, 45, 49, 54, 66, 69].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1383(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "utf-32" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [117, 116, 102, 45, 51, 50].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1385(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "utf-32BE" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [117, 116, 102, 45, 51, 50, 66, 69].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1387(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "us-ascii" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [117, 115, 45, 97, 115, 99, 105, 105].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1389(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "iso-8859-1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [105, 115, 111, 45, 56, 56, 53, 57, 45, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1391(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "utf-7" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [117, 116, 102, 45, 55].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1393(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "utf-8" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [117, 116, 102, 45, 56].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1394(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..8) {
            0 => Self::fragment_1379(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1381(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1383(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1385(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1387(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1389(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1391(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1393(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1395(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1396(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1397(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1398(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2182(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2184(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2189(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2192(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1401(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1394(depth + 1, max_depth, buf, rng);
        Self::fragment_1395(depth + 1, max_depth, buf, rng);
        Self::fragment_1396(depth + 1, max_depth, buf, rng);
        Self::fragment_1397(depth + 1, max_depth, buf, rng);
        Self::fragment_1398(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1402(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1394(depth + 1, max_depth, buf, rng);
        Self::fragment_1395(depth + 1, max_depth, buf, rng);
        Self::fragment_1396(depth + 1, max_depth, buf, rng);
        Self::fragment_1397(depth + 1, max_depth, buf, rng);
        Self::fragment_1398(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1403(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1404(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1405(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1406(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1394(depth + 1, max_depth, buf, rng);
        Self::fragment_1395(depth + 1, max_depth, buf, rng);
        Self::fragment_1396(depth + 1, max_depth, buf, rng);
        Self::fragment_1397(depth + 1, max_depth, buf, rng);
        Self::fragment_1398(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1407(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1402(depth + 1, max_depth, buf, rng);
        Self::fragment_1403(depth + 1, max_depth, buf, rng);
        Self::fragment_1404(depth + 1, max_depth, buf, rng);
        Self::fragment_1405(depth + 1, max_depth, buf, rng);
        Self::fragment_1406(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1409(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "AAAA" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 65, 65, 65].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1411(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "BBBB" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [66, 66, 66, 66].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1413(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1415(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_601(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_604(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_607(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1417(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";foo=bar" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [59, 102, 111, 111, 61, 98, 97, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1418(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1419(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1130(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1134(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1420(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1418(depth + 1, max_depth, buf, rng);
        Self::fragment_1419(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1422(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1440(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1445(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1423(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1440(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1445(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1424(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1425(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1422(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1426(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1426(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1423(depth + 1, max_depth, buf, rng);
        Self::fragment_1424(depth + 1, max_depth, buf, rng);
        Self::fragment_1425(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1428(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_1430(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "128" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 50, 56].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1432(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_521(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_523(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1434(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1435(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1428(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1430(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1432(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1434(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1436(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1417(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1420(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1437(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1438(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1409(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1411(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1413(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1415(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1439(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1440(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1435(depth + 1, max_depth, buf, rng);
        Self::fragment_1436(depth + 1, max_depth, buf, rng);
        Self::fragment_1437(depth + 1, max_depth, buf, rng);
        Self::fragment_1438(depth + 1, max_depth, buf, rng);
        Self::fragment_1439(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1441(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1428(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1430(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1432(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1434(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1442(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1443(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1409(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1411(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1413(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1415(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1444(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1445(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1441(depth + 1, max_depth, buf, rng);
        Self::fragment_1442(depth + 1, max_depth, buf, rng);
        Self::fragment_1443(depth + 1, max_depth, buf, rng);
        Self::fragment_1444(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1446(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1422(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1426(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1447(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1932(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1935(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1448(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1449(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1446(depth + 1, max_depth, buf, rng);
        Self::fragment_1447(depth + 1, max_depth, buf, rng);
        Self::fragment_1448(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1450(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1422(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1426(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1451(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1932(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1935(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1452(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1828(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1830(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1832(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1836(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1838(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1453(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1454(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1450(depth + 1, max_depth, buf, rng);
        Self::fragment_1451(depth + 1, max_depth, buf, rng);
        Self::fragment_1452(depth + 1, max_depth, buf, rng);
        Self::fragment_1453(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1455(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Transfer-Encoding:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 18;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    84, 114, 97, 110, 115, 102, 101, 114, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                    58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                18,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1456(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1457(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "chunked" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [99, 104, 117, 110, 107, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1458(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1460(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Encoding" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1461(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1462(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1463(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1464(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2556(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2558(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2560(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2562(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1465(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1467(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Language" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 97, 110, 103, 117, 97, 103, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1468(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1469(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1470(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1471(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1924(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1928(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1472(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1475(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Length: 200\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 50,
                    48, 48, 13, 10,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1477(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Length: 0\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 48,
                    13, 10,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1478(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Length: " */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1479(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1480(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1481(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1478(depth + 1, max_depth, buf, rng);
        Self::fragment_1479(depth + 1, max_depth, buf, rng);
        Self::fragment_1480(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1483(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1175(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1181(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1485(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2235(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2237(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1486(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Location" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 111, 99, 97, 116, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1487(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1488(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1489(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1490(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1483(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1485(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1491(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1501(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1509(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1511(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1515(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1519(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1523(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1502(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1509(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1511(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1515(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1519(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1523(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1503(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1504(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1505(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1506(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1501(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1507(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1507(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1502(depth + 1, max_depth, buf, rng);
        Self::fragment_1503(depth + 1, max_depth, buf, rng);
        Self::fragment_1504(depth + 1, max_depth, buf, rng);
        Self::fragment_1505(depth + 1, max_depth, buf, rng);
        Self::fragment_1506(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1509(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "SID=31d4d96e407aad42" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 73, 68, 61, 51, 49, 100, 52, 100, 57, 54, 101, 52, 48, 55, 97, 97, 100, 52,
                    50,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1511(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "PHPSESSID=298zf09hf012fh2; csrftoken=u32t4o3tb3gg43; _gat=1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 59;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    80, 72, 80, 83, 69, 83, 83, 73, 68, 61, 50, 57, 56, 122, 102, 48, 57, 104, 102,
                    48, 49, 50, 102, 104, 50, 59, 32, 99, 115, 114, 102, 116, 111, 107, 101, 110,
                    61, 117, 51, 50, 116, 52, 111, 51, 116, 98, 51, 103, 103, 52, 51, 59, 32, 95,
                    103, 97, 116, 61, 49,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                59,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1512(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1513(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_1514(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1515(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1512(depth + 1, max_depth, buf, rng);
        Self::fragment_1513(depth + 1, max_depth, buf, rng);
        Self::fragment_1514(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1516(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1517(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_1518(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1130(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1134(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1519(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1516(depth + 1, max_depth, buf, rng);
        Self::fragment_1517(depth + 1, max_depth, buf, rng);
        Self::fragment_1518(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1520(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1521(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_1522(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_601(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_604(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_607(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1523(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1520(depth + 1, max_depth, buf, rng);
        Self::fragment_1521(depth + 1, max_depth, buf, rng);
        Self::fragment_1522(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1524(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Cookie" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [67, 111, 111, 107, 105, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1525(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1526(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1527(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1528(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1501(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1507(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1529(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1532(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "123456" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 50, 51, 52, 53, 54].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1534(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "YWxhZGRpbjpvcGVuc2VzYW1l" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 24;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    89, 87, 120, 104, 90, 71, 82, 112, 98, 106, 112, 118, 99, 71, 86, 117, 99, 50,
                    86, 122, 89, 87, 49, 108,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                24,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1536(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_601(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_604(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_607(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1537(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Date" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 97, 116, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1538(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1539(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1540(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1541(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_704(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_706(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_708(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_710(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_712(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_714(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_716(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_725(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_734(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1542(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1545(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1547(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_1549(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "infinity" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [105, 110, 102, 105, 110, 105, 116, 121].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1551(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1552(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Depth:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 101, 112, 116, 104, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1553(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1554(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1545(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1547(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1549(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1551(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1555(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1557(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Destination" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1558(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1559(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1560(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1175(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1181(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1561(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1564(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_1566(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1567(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Early-Data" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [69, 97, 114, 108, 121, 45, 68, 97, 116, 97].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1568(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1569(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1570(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1564(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1566(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1571(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1574(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "gzip" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [103, 122, 105, 112].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1576(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "compress" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [99, 111, 109, 112, 114, 101, 115, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1578(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "deflate" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [100, 101, 102, 108, 97, 116, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1580(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "br" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [98, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1582(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "identity" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [105, 100, 101, 110, 116, 105, 116, 121].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1584(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "chunked" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [99, 104, 117, 110, 107, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1586(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "encoding-name" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [101, 110, 99, 111, 100, 105, 110, 103, 45, 110, 97, 109, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1587(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1574(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1576(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1578(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1580(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1582(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1584(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1588(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1589(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2182(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2184(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2189(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2192(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1590(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1587(depth + 1, max_depth, buf, rng);
        Self::fragment_1588(depth + 1, max_depth, buf, rng);
        Self::fragment_1589(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1592(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1586(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1590(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1593(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1586(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1590(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1594(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1595(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1586(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1590(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1596(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1593(depth + 1, max_depth, buf, rng);
        Self::fragment_1594(depth + 1, max_depth, buf, rng);
        Self::fragment_1595(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1598(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1475(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1477(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1481(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1600(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1455(depth + 1, max_depth, buf, rng);
        Self::fragment_1456(depth + 1, max_depth, buf, rng);
        Self::fragment_1457(depth + 1, max_depth, buf, rng);
        Self::fragment_1458(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1601(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1475(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1477(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1481(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1602(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1455(depth + 1, max_depth, buf, rng);
        Self::fragment_1456(depth + 1, max_depth, buf, rng);
        Self::fragment_1457(depth + 1, max_depth, buf, rng);
        Self::fragment_1458(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1603(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1601(depth + 1, max_depth, buf, rng);
        Self::fragment_1602(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1604(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1455(depth + 1, max_depth, buf, rng);
        Self::fragment_1456(depth + 1, max_depth, buf, rng);
        Self::fragment_1457(depth + 1, max_depth, buf, rng);
        Self::fragment_1458(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1605(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1475(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1477(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1481(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1606(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1604(depth + 1, max_depth, buf, rng);
        Self::fragment_1605(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1608(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        buf.push(42);
    }
    fn fragment_1610(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\"xyzzy\"" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [34, 120, 121, 122, 122, 121, 34].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1612(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\"AAAAAAAAAAAAAAAAAAAAAAAAA\"" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 27;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    34, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65, 65,
                    65, 65, 65, 65, 65, 65, 34,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                27,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1613(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\"" */
        buf.push(34);
    }
    fn fragment_1614(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1615(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\"" */
        buf.push(34);
    }
    fn fragment_1616(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1613(depth + 1, max_depth, buf, rng);
        Self::fragment_1614(depth + 1, max_depth, buf, rng);
        Self::fragment_1615(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1618(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1616(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1619(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1616(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1620(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1621(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1616(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1622(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1619(depth + 1, max_depth, buf, rng);
        Self::fragment_1620(depth + 1, max_depth, buf, rng);
        Self::fragment_1621(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1627(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Expect" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [69, 120, 112, 101, 99, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1628(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1629(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1630(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "100-continue" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 48, 48, 45, 99, 111, 110, 116, 105, 110, 117, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1631(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1633(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Expires:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [69, 120, 112, 105, 114, 101, 115, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1634(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1635(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_704(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_706(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_708(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_710(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_712(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_714(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_716(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_725(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_734(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1636(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1640(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Forwarded" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [70, 111, 114, 119, 97, 114, 100, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1641(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1642(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1643(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "by" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [98, 121].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1644(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_1645(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1175(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1181(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1646(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1650(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "From" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [70, 114, 111, 109].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1651(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1652(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1653(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "webmaster@w3.org" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    119, 101, 98, 109, 97, 115, 116, 101, 114, 64, 119, 51, 46, 111, 114, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1654(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1657(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1658(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1659(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1657(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1660(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1660(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1658(depth + 1, max_depth, buf, rng);
        Self::fragment_1659(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1663(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1667(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1670(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1674(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1679(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1685(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1688(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1664(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1667(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1598(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1600(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1603(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1668(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1598(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1600(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1603(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1669(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1670(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1668(depth + 1, max_depth, buf, rng);
        Self::fragment_1669(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1671(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1598(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1600(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1603(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1672(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1673(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1674(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1671(depth + 1, max_depth, buf, rng);
        Self::fragment_1672(depth + 1, max_depth, buf, rng);
        Self::fragment_1673(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1675(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1598(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1600(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1603(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1676(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1677(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1678(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1679(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1675(depth + 1, max_depth, buf, rng);
        Self::fragment_1676(depth + 1, max_depth, buf, rng);
        Self::fragment_1677(depth + 1, max_depth, buf, rng);
        Self::fragment_1678(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1680(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1598(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1600(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1603(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1681(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1682(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1683(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1684(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2334(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2336(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2338(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2340(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2342(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2344(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2346(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2348(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2350(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2352(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2354(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2356(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2358(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2360(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2362(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2364(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2366(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2368(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2370(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2372(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2374(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2376(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2378(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2380(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2382(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2384(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2386(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2388(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2390(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2392(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2394(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2396(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2398(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2400(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2402(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2404(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2406(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2408(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2410(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2412(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2414(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2416(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2418(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2420(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2422(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2424(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2426(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2428(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2430(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2432(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2434(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2436(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2438(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2440(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2442(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2444(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2446(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2448(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2450(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2452(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2454(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2456(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2458(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2460(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2462(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2464(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2466(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2468(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1685(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1680(depth + 1, max_depth, buf, rng);
        Self::fragment_1681(depth + 1, max_depth, buf, rng);
        Self::fragment_1682(depth + 1, max_depth, buf, rng);
        Self::fragment_1683(depth + 1, max_depth, buf, rng);
        Self::fragment_1684(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1686(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1598(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1600(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1603(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1687(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1657(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1660(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1688(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1686(depth + 1, max_depth, buf, rng);
        Self::fragment_1687(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1690(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 99, 99, 101, 112, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1692(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept-Charset" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 99, 99, 101, 112, 116, 45, 67, 104, 97, 114, 115, 101, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1694(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept-Encoding" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1696(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept-Language" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 99, 99, 101, 112, 116, 45, 76, 97, 110, 103, 117, 97, 103, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1698(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Accept-Ranges" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 99, 99, 101, 112, 116, 45, 82, 97, 110, 103, 101, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1700(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Allow" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 108, 108, 111, 119].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1702(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "ALPN" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 76, 80, 78].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1704(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Alt-Used" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 108, 116, 45, 85, 115, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1706(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Authorization" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 117, 116, 104, 111, 114, 105, 122, 97, 116, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1708(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Cache-Control" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [67, 97, 99, 104, 101, 45, 67, 111, 110, 116, 114, 111, 108].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1710(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "CalDav-Timezones" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 97, 108, 68, 97, 118, 45, 84, 105, 109, 101, 122, 111, 110, 101, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1712(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "CDN-Loop" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [67, 68, 78, 45, 76, 111, 111, 112].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1714(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Encoding" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1716(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Language" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 97, 110, 103, 117, 97, 103, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1718(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Length" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1720(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Location" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 111, 99, 97, 116, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1722(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Cookie" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [67, 111, 111, 107, 105, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1724(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Date" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 97, 116, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1726(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Depth" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 101, 112, 116, 104].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1728(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Destination" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1730(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Early-Data" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [69, 97, 114, 108, 121, 45, 68, 97, 116, 97].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1732(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Expect" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [69, 120, 112, 101, 99, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1734(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Expires" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [69, 120, 112, 105, 114, 101, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1736(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Forwarded" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [70, 111, 114, 119, 97, 114, 100, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1738(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "From" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [70, 114, 111, 109].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1740(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "HTTP2-Settings" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    72, 84, 84, 80, 50, 45, 83, 101, 116, 116, 105, 110, 103, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1742(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 102].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1744(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Match" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 102, 45, 77, 97, 116, 99, 104].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1746(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Modified-Since" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    73, 102, 45, 77, 111, 100, 105, 102, 105, 101, 100, 45, 83, 105, 110, 99, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1748(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-None-Match" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 102, 45, 78, 111, 110, 101, 45, 77, 97, 116, 99, 104].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1750(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Range" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 102, 45, 82, 97, 110, 103, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1752(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Schedule-Tag-Match" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    73, 102, 45, 83, 99, 104, 101, 100, 117, 108, 101, 45, 84, 97, 103, 45, 77, 97,
                    116, 99, 104,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1754(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Unmodified-Since" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    73, 102, 45, 85, 110, 109, 111, 100, 105, 102, 105, 101, 100, 45, 83, 105, 110,
                    99, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1756(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Link" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [76, 105, 110, 107].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1758(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Max-Forwards" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [77, 97, 120, 45, 70, 111, 114, 119, 97, 114, 100, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1760(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "MIME-Version" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [77, 73, 77, 69, 45, 86, 101, 114, 115, 105, 111, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1762(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OData-Isolation" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    79, 68, 97, 116, 97, 45, 73, 115, 111, 108, 97, 116, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1764(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OData-MaxVersion" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    79, 68, 97, 116, 97, 45, 77, 97, 120, 86, 101, 114, 115, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1766(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OData-Version" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 68, 97, 116, 97, 45, 86, 101, 114, 115, 105, 111, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1768(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Ordering-Type" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 114, 100, 101, 114, 105, 110, 103, 45, 84, 121, 112, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1770(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Origin" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 114, 105, 103, 105, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1772(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OSCORE" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 83, 67, 79, 82, 69].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1774(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Overwrite" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 118, 101, 114, 119, 114, 105, 116, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1776(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Position" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [80, 111, 115, 105, 116, 105, 111, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1778(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Pragma" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [80, 114, 97, 103, 109, 97].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1780(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Prefer" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [80, 114, 101, 102, 101, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1782(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Proxy-Authorization" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    80, 114, 111, 120, 121, 45, 65, 117, 116, 104, 111, 114, 105, 122, 97, 116,
                    105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1784(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Range" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [82, 97, 110, 103, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1786(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Referer" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [82, 101, 102, 101, 114, 101, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1788(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Schedule-Reply" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 99, 104, 101, 100, 117, 108, 101, 45, 82, 101, 112, 108, 121,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1790(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Token-Binding" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 84, 111, 107, 101, 110, 45, 66, 105, 110, 100, 105, 110, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1792(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Accept" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 65, 99, 99, 101,
                    112, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1794(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Extensions" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 24;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 69, 120, 116,
                    101, 110, 115, 105, 111, 110, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                24,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1796(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Key" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 75, 101, 121,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1798(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Protocol" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 80, 114, 111,
                    116, 111, 99, 111, 108,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1800(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Version" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 86, 101, 114,
                    115, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1802(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Slug" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 108, 117, 103].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1804(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "TE" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 69].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1806(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Timeout" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 105, 109, 101, 111, 117, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1808(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Topic" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 111, 112, 105, 99].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1810(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Trailer" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 114, 97, 105, 108, 101, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1812(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Transfer-Encoding" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    84, 114, 97, 110, 115, 102, 101, 114, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1814(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "TTL" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 84, 76].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1816(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Urgency" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [85, 114, 103, 101, 110, 99, 121].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1818(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Upgrade" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [85, 112, 103, 114, 97, 100, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1820(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "User-Agent" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [85, 115, 101, 114, 45, 65, 103, 101, 110, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1822(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Via" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [86, 105, 97].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1824(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Server" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 101, 114, 118, 101, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1826(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Last-Modified" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [76, 97, 115, 116, 45, 77, 111, 100, 105, 102, 105, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1828(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Transfer-Encoding: chunked\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 28;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    84, 114, 97, 110, 115, 102, 101, 114, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                    58, 32, 99, 104, 117, 110, 107, 101, 100, 13, 10,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                28,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1830(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Transfer-Encoding: identity\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 29;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    84, 114, 97, 110, 115, 102, 101, 114, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                    58, 32, 105, 100, 101, 110, 116, 105, 116, 121, 13, 10,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                29,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1832(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Length: 180\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32, 49,
                    56, 48, 13, 10,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1833(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Content-Length: " */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    67, 111, 110, 116, 101, 110, 116, 45, 76, 101, 110, 103, 116, 104, 58, 32,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1834(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_521(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_523(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1835(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1836(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1833(depth + 1, max_depth, buf, rng);
        Self::fragment_1834(depth + 1, max_depth, buf, rng);
        Self::fragment_1835(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1838(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Bar: Foo\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [66, 97, 114, 58, 32, 70, 111, 111, 13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1840(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0.9" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [48, 46, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1842(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1844(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1.1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 46, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1846(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1848(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1849(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1850(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_1851(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1852(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1849(depth + 1, max_depth, buf, rng);
        Self::fragment_1850(depth + 1, max_depth, buf, rng);
        Self::fragment_1851(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1853(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1854(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_1855(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1856(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1853(depth + 1, max_depth, buf, rng);
        Self::fragment_1854(depth + 1, max_depth, buf, rng);
        Self::fragment_1855(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1857(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "HTTP/" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [72, 84, 84, 80, 47].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1858(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1840(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1842(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1844(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1846(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1848(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1852(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1856(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1862(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "HTTP2-Settings" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    72, 84, 84, 80, 50, 45, 83, 101, 116, 116, 105, 110, 103, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1863(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1864(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1865(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "AAMAAABkAARAAAAAAAIAAAAA" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 24;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 65, 77, 65, 65, 65, 66, 107, 65, 65, 82, 65, 65, 65, 65, 65, 65, 65, 73,
                    65, 65, 65, 65, 65,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                24,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1866(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1870(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Match:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 102, 45, 77, 97, 116, 99, 104, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1871(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1872(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1618(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1622(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1873(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1875(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Modified-Since:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 18;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    73, 102, 45, 77, 111, 100, 105, 102, 105, 101, 100, 45, 83, 105, 110, 99, 101,
                    58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                18,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1876(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1877(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_704(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_706(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_708(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_710(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_712(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_714(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_716(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_725(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_734(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1878(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1880(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-None-Match:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 102, 45, 78, 111, 110, 101, 45, 77, 97, 116, 99, 104, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1881(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1882(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1618(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1622(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1883(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1886(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1616(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1888(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_704(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_706(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_708(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_710(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_712(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_714(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_716(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_725(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_734(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1889(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Range:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 102, 45, 82, 97, 110, 103, 101, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1890(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1891(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1886(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1888(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1892(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1894(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Schedule-Tag-Match:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    73, 102, 45, 83, 99, 104, 101, 100, 117, 108, 101, 45, 84, 97, 103, 45, 77, 97,
                    116, 99, 104, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1895(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1896(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1616(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1897(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1899(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If-Unmodified-Since:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    73, 102, 45, 85, 110, 109, 111, 100, 105, 102, 105, 101, 100, 45, 83, 105, 110,
                    99, 101, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1900(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1901(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_704(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_706(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_708(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_710(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_712(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_714(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_716(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_725(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_734(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1902(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1904(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "If" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 102].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1905(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1906(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1907(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2470(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2473(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1908(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1911(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "fr" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [102, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1913(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "en" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [101, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1915(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "de" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [100, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1916(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_839(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_841(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_845(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_847(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_849(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_851(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_853(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_857(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_859(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_861(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_863(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1917(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_839(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_841(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_845(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_847(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_849(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_851(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_853(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_857(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_859(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_861(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_863(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1918(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1916(depth + 1, max_depth, buf, rng);
        Self::fragment_1917(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1919(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1911(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1913(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1915(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1918(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1920(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1921(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2182(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2184(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2189(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2192(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1924(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1919(depth + 1, max_depth, buf, rng);
        Self::fragment_1920(depth + 1, max_depth, buf, rng);
        Self::fragment_1921(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1925(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1919(depth + 1, max_depth, buf, rng);
        Self::fragment_1920(depth + 1, max_depth, buf, rng);
        Self::fragment_1921(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1926(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1927(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1919(depth + 1, max_depth, buf, rng);
        Self::fragment_1920(depth + 1, max_depth, buf, rng);
        Self::fragment_1921(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1928(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1925(depth + 1, max_depth, buf, rng);
        Self::fragment_1926(depth + 1, max_depth, buf, rng);
        Self::fragment_1927(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1929(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1930(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1417(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1420(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1931(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1932(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1929(depth + 1, max_depth, buf, rng);
        Self::fragment_1930(depth + 1, max_depth, buf, rng);
        Self::fragment_1931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1933(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1934(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1935(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1933(depth + 1, max_depth, buf, rng);
        Self::fragment_1934(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1943(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "<" */
        buf.push(60);
    }
    fn fragment_1944(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1175(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1181(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1945(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ">" */
        buf.push(62);
    }
    fn fragment_1947(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Link:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [76, 105, 110, 107, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1948(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1949(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1943(depth + 1, max_depth, buf, rng);
        Self::fragment_1944(depth + 1, max_depth, buf, rng);
        Self::fragment_1945(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1950(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1957(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1959(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_1960(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Max-Forwards" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [77, 97, 120, 45, 70, 111, 114, 119, 97, 114, 100, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1961(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1962(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1963(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1957(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1959(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1964(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1967(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "GET" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [71, 69, 84].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1969(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "HEAD" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [72, 69, 65, 68].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1971(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "POST" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [80, 79, 83, 84].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1973(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "PUT" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [80, 85, 84].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1975(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "DELETE" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 69, 76, 69, 84, 69].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1977(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "CONNECT" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [67, 79, 78, 78, 69, 67, 84].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1979(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OPTIONS" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 80, 84, 73, 79, 78, 83].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1981(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "TRACE" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 82, 65, 67, 69].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1983(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..8) {
            0 => Self::fragment_1967(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1969(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1971(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1973(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1975(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1977(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1979(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1981(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1984(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..8) {
            0 => Self::fragment_1967(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1969(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1971(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1973(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1975(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1977(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1979(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1981(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1985(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1986(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1983(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1987(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1987(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1984(depth + 1, max_depth, buf, rng);
        Self::fragment_1985(depth + 1, max_depth, buf, rng);
        Self::fragment_1986(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1991(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }

        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [42, 47, 42].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1993(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "application/octet-stream" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 24;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 111, 99, 116, 101, 116,
                    45, 115, 116, 114, 101, 97, 109,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                24,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1995(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "application/pdf" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 112, 100, 102,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1997(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "application/pkcs8" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 112, 107, 99, 115, 56,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_1999(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "application/zip" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 122, 105, 112,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2001(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "audio/mpeg" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [97, 117, 100, 105, 111, 47, 109, 112, 101, 103].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2003(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "audio/vorbis" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [97, 117, 100, 105, 111, 47, 118, 111, 114, 98, 105, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2005(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "audio/example" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [97, 117, 100, 105, 111, 47, 101, 120, 97, 109, 112, 108, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2007(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "font/woff" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [102, 111, 110, 116, 47, 119, 111, 102, 102].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2009(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "font/ttf" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [102, 111, 110, 116, 47, 116, 116, 102].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2011(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "font/otf" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [102, 111, 110, 116, 47, 111, 116, 102].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2013(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "image/jpeg" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [105, 109, 97, 103, 101, 47, 106, 112, 101, 103].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2015(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "image/png" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [105, 109, 97, 103, 101, 47, 112, 110, 103].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2017(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "image/svg+xml" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [105, 109, 97, 103, 101, 47, 115, 118, 103, 43, 120, 109, 108].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2019(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "model/3mf" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [109, 111, 100, 101, 108, 47, 51, 109, 102].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2021(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "text/html" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [116, 101, 120, 116, 47, 104, 116, 109, 108].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2023(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "video/mp4" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [118, 105, 100, 101, 111, 47, 109, 112, 52].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2025(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2203(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2207(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2029(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2031(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1.1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 46, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2032(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "MIME-Version" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [77, 73, 77, 69, 45, 86, 101, 114, 115, 105, 111, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2033(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2034(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2035(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2029(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2031(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2036(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2044(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OData-Isolation" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    79, 68, 97, 116, 97, 45, 73, 115, 111, 108, 97, 116, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2045(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2046(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2047(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "snapshot" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [115, 110, 97, 112, 115, 104, 111, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2048(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2052(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OData-MaxVersion" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    79, 68, 97, 116, 97, 45, 77, 97, 120, 86, 101, 114, 115, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2053(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2054(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2055(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2056(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2062(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OData-Version" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 68, 97, 116, 97, 45, 86, 101, 114, 115, 105, 111, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2063(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2064(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2065(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2066(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2073(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "DAV:unordered" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 65, 86, 58, 117, 110, 111, 114, 100, 101, 114, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2075(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "DAV:custom" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 65, 86, 58, 99, 117, 115, 116, 111, 109].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2077(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://example.org/example.html" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 31;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 111, 114,
                    103, 47, 101, 120, 97, 109, 112, 108, 101, 46, 104, 116, 109, 108,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                31,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2078(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Ordering-Type" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 114, 100, 101, 114, 105, 110, 103, 45, 84, 121, 112, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2079(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2080(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2081(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2073(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2075(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2077(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2082(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2087(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http://example.com" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 18;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 116, 116, 112, 58, 47, 47, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111,
                    109,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                18,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2089(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "null" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [110, 117, 108, 108].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2090(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Origin" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 114, 105, 103, 105, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2091(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2092(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2093(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2087(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2089(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2094(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2099(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "CSU" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [67, 83, 85].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2101(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "AA" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [65, 65].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2102(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "OSCORE" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 83, 67, 79, 82, 69].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2103(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2104(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2105(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2099(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2101(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2106(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2110(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Overwrite" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [79, 118, 101, 114, 119, 114, 105, 116, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2111(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2112(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2113(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1314(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1316(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2114(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2119(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "first" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [102, 105, 114, 115, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2121(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "last" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [108, 97, 115, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2123(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "after example.html" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 18;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    97, 102, 116, 101, 114, 32, 101, 120, 97, 109, 112, 108, 101, 46, 104, 116,
                    109, 108,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                18,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2124(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Position" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [80, 111, 115, 105, 116, 105, 111, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2125(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2126(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2127(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2119(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2121(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2123(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2128(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2134(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Pragma" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [80, 114, 97, 103, 109, 97].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2135(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2136(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2137(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "no-cache" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [110, 111, 45, 99, 97, 99, 104, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2138(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2142(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Prefer" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [80, 114, 101, 102, 101, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2143(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2144(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2145(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2155(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2157(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2159(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2161(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2146(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2149(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "respond-async" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [114, 101, 115, 112, 111, 110, 100, 45, 97, 115, 121, 110, 99].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2151(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "wait=100" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [119, 97, 105, 116, 61, 49, 48, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2153(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "handling=lenient" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    104, 97, 110, 100, 108, 105, 110, 103, 61, 108, 101, 110, 105, 101, 110, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2155(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2149(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2151(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2153(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2157(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2149(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2151(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2153(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2159(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2161(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2149(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2151(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2153(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2163(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http%2F1.1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [104, 116, 116, 112, 37, 50, 70, 49, 46, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2165(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "h2" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [104, 50].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2166(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "http%2F" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [104, 116, 116, 112, 37, 50, 70].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2167(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1840(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1842(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1844(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1846(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1848(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1852(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1856(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2168(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2166(depth + 1, max_depth, buf, rng);
        Self::fragment_2167(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2170(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2163(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2165(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2168(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2171(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2163(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2165(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2168(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2172(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2173(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2163(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2165(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2168(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2174(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2171(depth + 1, max_depth, buf, rng);
        Self::fragment_2172(depth + 1, max_depth, buf, rng);
        Self::fragment_2173(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2175(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Proxy-Authorization:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    80, 114, 111, 120, 121, 45, 65, 117, 116, 104, 111, 114, 105, 122, 97, 116,
                    105, 111, 110, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2176(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_1277(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1279(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1281(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1283(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1285(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1287(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1289(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1291(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1293(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1295(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2177(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_2178(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1532(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1534(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1536(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2179(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2182(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "q=1.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [113, 61, 49, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2184(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "q=0.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [113, 61, 48, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2185(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "q=" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [113, 61].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2186(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2187(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_2188(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2189(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2185(depth + 1, max_depth, buf, rng);
        Self::fragment_2186(depth + 1, max_depth, buf, rng);
        Self::fragment_2187(depth + 1, max_depth, buf, rng);
        Self::fragment_2188(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2190(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "q=" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [113, 61].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2191(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2192(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2190(depth + 1, max_depth, buf, rng);
        Self::fragment_2191(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2193(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2194(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2195(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2196(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2197(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2198(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2200(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2201(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_2202(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2203(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2200(depth + 1, max_depth, buf, rng);
        Self::fragment_2201(depth + 1, max_depth, buf, rng);
        Self::fragment_2202(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2204(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2205(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_2206(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2207(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2204(depth + 1, max_depth, buf, rng);
        Self::fragment_2205(depth + 1, max_depth, buf, rng);
        Self::fragment_2206(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2209(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "bytes" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [98, 121, 116, 101, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2211(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "none" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [110, 111, 110, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2213(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5-8" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [53, 45, 56].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2215(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5-" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [53, 45].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2216(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2217(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_2218(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2219(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2216(depth + 1, max_depth, buf, rng);
        Self::fragment_2217(depth + 1, max_depth, buf, rng);
        Self::fragment_2218(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2220(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Range:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [82, 97, 110, 103, 101, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2221(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2209(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2211(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2222(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_2223(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2213(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2215(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2219(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2224(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2228(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Referer" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [82, 101, 102, 101, 114, 101, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2229(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2230(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2231(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1170(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1175(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1181(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2232(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2235(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/example" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [47, 101, 120, 97, 109, 112, 108, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2237(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1122(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1126(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2240(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Schedule-Reply" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 99, 104, 101, 100, 117, 108, 101, 45, 82, 101, 112, 108, 121,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2241(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2242(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2243(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1314(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1316(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2244(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2250(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Token-Binding" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 84, 111, 107, 101, 110, 45, 66, 105, 110, 100, 105, 110, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2251(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2252(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2253(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "AIkAAgBBQLgtRpWFPN66kxhxGrtaKrzcMtHw7HV8" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 40;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    65, 73, 107, 65, 65, 103, 66, 66, 81, 76, 103, 116, 82, 112, 87, 70, 80, 78,
                    54, 54, 107, 120, 104, 120, 71, 114, 116, 97, 75, 114, 122, 99, 77, 116, 72,
                    119, 55, 72, 86, 56,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                40,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2254(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2260(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Accept" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 65, 99, 99, 101,
                    112, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2261(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2262(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2263(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 28;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    115, 51, 112, 80, 76, 77, 66, 105, 84, 120, 97, 81, 57, 107, 89, 71, 122, 122,
                    104, 90, 82, 98, 75, 43, 120, 79, 111, 61,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                28,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2264(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2267(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "deflate-stream" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    100, 101, 102, 108, 97, 116, 101, 45, 115, 116, 114, 101, 97, 109,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2269(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "mux" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [109, 117, 120].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2271(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "max-channels:4; flow-control" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 28;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    109, 97, 120, 45, 99, 104, 97, 110, 110, 101, 108, 115, 58, 52, 59, 32, 102,
                    108, 111, 119, 45, 99, 111, 110, 116, 114, 111, 108,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                28,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2273(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2267(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2269(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2271(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2274(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2267(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2269(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2271(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2275(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2276(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2267(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2269(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2271(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2277(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2274(depth + 1, max_depth, buf, rng);
        Self::fragment_2275(depth + 1, max_depth, buf, rng);
        Self::fragment_2276(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2278(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Extensions:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 25;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 69, 120, 116,
                    101, 110, 115, 105, 111, 110, 115, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                25,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2279(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2273(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2277(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2280(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2283(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "dGhlIHNhbXBsZSBub25jZQ==" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 24;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    100, 71, 104, 108, 73, 72, 78, 104, 98, 88, 66, 115, 90, 83, 66, 117, 98, 50,
                    53, 106, 90, 81, 61, 61,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                24,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2285(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_601(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_604(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_607(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2286(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Key:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 18;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 75, 101, 121, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                18,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2287(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2288(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2283(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2285(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2289(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2292(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "chat" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [99, 104, 97, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2294(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "superchat" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [115, 117, 112, 101, 114, 99, 104, 97, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2296(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2292(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2294(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2297(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2292(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2294(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2298(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2299(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2292(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2294(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2300(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2297(depth + 1, max_depth, buf, rng);
        Self::fragment_2298(depth + 1, max_depth, buf, rng);
        Self::fragment_2299(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2301(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Protocol:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 23;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 80, 114, 111,
                    116, 111, 99, 111, 108, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                23,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2302(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2303(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2300(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2304(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2307(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "13" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 51].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2309(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2310(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Sec-Websocket-Version:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 45, 87, 101, 98, 115, 111, 99, 107, 101, 116, 45, 86, 101, 114,
                    115, 105, 111, 110, 58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2311(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2312(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2307(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2309(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2313(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2325(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "The Beach at S%C3%A8te" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    84, 104, 101, 32, 66, 101, 97, 99, 104, 32, 97, 116, 32, 83, 37, 67, 51, 37,
                    65, 56, 116, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2327(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2328(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Slug:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 108, 117, 103, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2329(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2330(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2325(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2327(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2331(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2334(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1225(depth + 1, max_depth, buf, rng);
        Self::fragment_1226(depth + 1, max_depth, buf, rng);
        Self::fragment_1227(depth + 1, max_depth, buf, rng);
        Self::fragment_1228(depth + 1, max_depth, buf, rng);
        Self::fragment_1229(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2336(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1189(depth + 1, max_depth, buf, rng);
        Self::fragment_1190(depth + 1, max_depth, buf, rng);
        Self::fragment_1191(depth + 1, max_depth, buf, rng);
        Self::fragment_1192(depth + 1, max_depth, buf, rng);
        Self::fragment_1193(depth + 1, max_depth, buf, rng);
        Self::fragment_1194(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2338(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1196(depth + 1, max_depth, buf, rng);
        Self::fragment_1197(depth + 1, max_depth, buf, rng);
        Self::fragment_1198(depth + 1, max_depth, buf, rng);
        Self::fragment_1199(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2340(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1201(depth + 1, max_depth, buf, rng);
        Self::fragment_1202(depth + 1, max_depth, buf, rng);
        Self::fragment_1203(depth + 1, max_depth, buf, rng);
        Self::fragment_1204(depth + 1, max_depth, buf, rng);
        Self::fragment_1205(depth + 1, max_depth, buf, rng);
        Self::fragment_1206(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2342(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1208(depth + 1, max_depth, buf, rng);
        Self::fragment_1209(depth + 1, max_depth, buf, rng);
        Self::fragment_1210(depth + 1, max_depth, buf, rng);
        Self::fragment_1211(depth + 1, max_depth, buf, rng);
        Self::fragment_1212(depth + 1, max_depth, buf, rng);
        Self::fragment_1213(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2344(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1244(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1249(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2346(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1256(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1261(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2348(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1270(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1275(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2350(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1296(depth + 1, max_depth, buf, rng);
        Self::fragment_1297(depth + 1, max_depth, buf, rng);
        Self::fragment_1298(depth + 1, max_depth, buf, rng);
        Self::fragment_1299(depth + 1, max_depth, buf, rng);
        Self::fragment_1300(depth + 1, max_depth, buf, rng);
        Self::fragment_1301(depth + 1, max_depth, buf, rng);
        Self::fragment_1302(depth + 1, max_depth, buf, rng);
        Self::fragment_1303(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2352(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1319(depth + 1, max_depth, buf, rng);
        Self::fragment_1320(depth + 1, max_depth, buf, rng);
        Self::fragment_1321(depth + 1, max_depth, buf, rng);
        Self::fragment_1322(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2354(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1346(depth + 1, max_depth, buf, rng);
        Self::fragment_1347(depth + 1, max_depth, buf, rng);
        Self::fragment_1348(depth + 1, max_depth, buf, rng);
        Self::fragment_1349(depth + 1, max_depth, buf, rng);
        Self::fragment_1350(depth + 1, max_depth, buf, rng);
        Self::fragment_1351(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2356(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1371(depth + 1, max_depth, buf, rng);
        Self::fragment_1372(depth + 1, max_depth, buf, rng);
        Self::fragment_1373(depth + 1, max_depth, buf, rng);
        Self::fragment_1374(depth + 1, max_depth, buf, rng);
        Self::fragment_1375(depth + 1, max_depth, buf, rng);
        Self::fragment_1376(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2358(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1460(depth + 1, max_depth, buf, rng);
        Self::fragment_1461(depth + 1, max_depth, buf, rng);
        Self::fragment_1462(depth + 1, max_depth, buf, rng);
        Self::fragment_1463(depth + 1, max_depth, buf, rng);
        Self::fragment_1464(depth + 1, max_depth, buf, rng);
        Self::fragment_1465(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2360(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1467(depth + 1, max_depth, buf, rng);
        Self::fragment_1468(depth + 1, max_depth, buf, rng);
        Self::fragment_1469(depth + 1, max_depth, buf, rng);
        Self::fragment_1470(depth + 1, max_depth, buf, rng);
        Self::fragment_1471(depth + 1, max_depth, buf, rng);
        Self::fragment_1472(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2362(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1475(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1477(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1481(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2364(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1486(depth + 1, max_depth, buf, rng);
        Self::fragment_1487(depth + 1, max_depth, buf, rng);
        Self::fragment_1488(depth + 1, max_depth, buf, rng);
        Self::fragment_1489(depth + 1, max_depth, buf, rng);
        Self::fragment_1490(depth + 1, max_depth, buf, rng);
        Self::fragment_1491(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2366(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1524(depth + 1, max_depth, buf, rng);
        Self::fragment_1525(depth + 1, max_depth, buf, rng);
        Self::fragment_1526(depth + 1, max_depth, buf, rng);
        Self::fragment_1527(depth + 1, max_depth, buf, rng);
        Self::fragment_1528(depth + 1, max_depth, buf, rng);
        Self::fragment_1529(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2368(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1537(depth + 1, max_depth, buf, rng);
        Self::fragment_1538(depth + 1, max_depth, buf, rng);
        Self::fragment_1539(depth + 1, max_depth, buf, rng);
        Self::fragment_1540(depth + 1, max_depth, buf, rng);
        Self::fragment_1541(depth + 1, max_depth, buf, rng);
        Self::fragment_1542(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2370(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1552(depth + 1, max_depth, buf, rng);
        Self::fragment_1553(depth + 1, max_depth, buf, rng);
        Self::fragment_1554(depth + 1, max_depth, buf, rng);
        Self::fragment_1555(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2372(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1557(depth + 1, max_depth, buf, rng);
        Self::fragment_1558(depth + 1, max_depth, buf, rng);
        Self::fragment_1559(depth + 1, max_depth, buf, rng);
        Self::fragment_1560(depth + 1, max_depth, buf, rng);
        Self::fragment_1561(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2374(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1567(depth + 1, max_depth, buf, rng);
        Self::fragment_1568(depth + 1, max_depth, buf, rng);
        Self::fragment_1569(depth + 1, max_depth, buf, rng);
        Self::fragment_1570(depth + 1, max_depth, buf, rng);
        Self::fragment_1571(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2376(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1627(depth + 1, max_depth, buf, rng);
        Self::fragment_1628(depth + 1, max_depth, buf, rng);
        Self::fragment_1629(depth + 1, max_depth, buf, rng);
        Self::fragment_1630(depth + 1, max_depth, buf, rng);
        Self::fragment_1631(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2378(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1633(depth + 1, max_depth, buf, rng);
        Self::fragment_1634(depth + 1, max_depth, buf, rng);
        Self::fragment_1635(depth + 1, max_depth, buf, rng);
        Self::fragment_1636(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2380(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1640(depth + 1, max_depth, buf, rng);
        Self::fragment_1641(depth + 1, max_depth, buf, rng);
        Self::fragment_1642(depth + 1, max_depth, buf, rng);
        Self::fragment_1643(depth + 1, max_depth, buf, rng);
        Self::fragment_1644(depth + 1, max_depth, buf, rng);
        Self::fragment_1645(depth + 1, max_depth, buf, rng);
        Self::fragment_1646(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2382(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1650(depth + 1, max_depth, buf, rng);
        Self::fragment_1651(depth + 1, max_depth, buf, rng);
        Self::fragment_1652(depth + 1, max_depth, buf, rng);
        Self::fragment_1653(depth + 1, max_depth, buf, rng);
        Self::fragment_1654(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2384(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1862(depth + 1, max_depth, buf, rng);
        Self::fragment_1863(depth + 1, max_depth, buf, rng);
        Self::fragment_1864(depth + 1, max_depth, buf, rng);
        Self::fragment_1865(depth + 1, max_depth, buf, rng);
        Self::fragment_1866(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2386(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1904(depth + 1, max_depth, buf, rng);
        Self::fragment_1905(depth + 1, max_depth, buf, rng);
        Self::fragment_1906(depth + 1, max_depth, buf, rng);
        Self::fragment_1907(depth + 1, max_depth, buf, rng);
        Self::fragment_1908(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2388(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1870(depth + 1, max_depth, buf, rng);
        Self::fragment_1871(depth + 1, max_depth, buf, rng);
        Self::fragment_1872(depth + 1, max_depth, buf, rng);
        Self::fragment_1873(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2390(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1875(depth + 1, max_depth, buf, rng);
        Self::fragment_1876(depth + 1, max_depth, buf, rng);
        Self::fragment_1877(depth + 1, max_depth, buf, rng);
        Self::fragment_1878(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2392(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1880(depth + 1, max_depth, buf, rng);
        Self::fragment_1881(depth + 1, max_depth, buf, rng);
        Self::fragment_1882(depth + 1, max_depth, buf, rng);
        Self::fragment_1883(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2394(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1889(depth + 1, max_depth, buf, rng);
        Self::fragment_1890(depth + 1, max_depth, buf, rng);
        Self::fragment_1891(depth + 1, max_depth, buf, rng);
        Self::fragment_1892(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2396(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1894(depth + 1, max_depth, buf, rng);
        Self::fragment_1895(depth + 1, max_depth, buf, rng);
        Self::fragment_1896(depth + 1, max_depth, buf, rng);
        Self::fragment_1897(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2398(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1899(depth + 1, max_depth, buf, rng);
        Self::fragment_1900(depth + 1, max_depth, buf, rng);
        Self::fragment_1901(depth + 1, max_depth, buf, rng);
        Self::fragment_1902(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2400(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1947(depth + 1, max_depth, buf, rng);
        Self::fragment_1948(depth + 1, max_depth, buf, rng);
        Self::fragment_1949(depth + 1, max_depth, buf, rng);
        Self::fragment_1950(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2402(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1960(depth + 1, max_depth, buf, rng);
        Self::fragment_1961(depth + 1, max_depth, buf, rng);
        Self::fragment_1962(depth + 1, max_depth, buf, rng);
        Self::fragment_1963(depth + 1, max_depth, buf, rng);
        Self::fragment_1964(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2404(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2032(depth + 1, max_depth, buf, rng);
        Self::fragment_2033(depth + 1, max_depth, buf, rng);
        Self::fragment_2034(depth + 1, max_depth, buf, rng);
        Self::fragment_2035(depth + 1, max_depth, buf, rng);
        Self::fragment_2036(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2406(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2044(depth + 1, max_depth, buf, rng);
        Self::fragment_2045(depth + 1, max_depth, buf, rng);
        Self::fragment_2046(depth + 1, max_depth, buf, rng);
        Self::fragment_2047(depth + 1, max_depth, buf, rng);
        Self::fragment_2048(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2408(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2052(depth + 1, max_depth, buf, rng);
        Self::fragment_2053(depth + 1, max_depth, buf, rng);
        Self::fragment_2054(depth + 1, max_depth, buf, rng);
        Self::fragment_2055(depth + 1, max_depth, buf, rng);
        Self::fragment_2056(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2410(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2062(depth + 1, max_depth, buf, rng);
        Self::fragment_2063(depth + 1, max_depth, buf, rng);
        Self::fragment_2064(depth + 1, max_depth, buf, rng);
        Self::fragment_2065(depth + 1, max_depth, buf, rng);
        Self::fragment_2066(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2412(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2078(depth + 1, max_depth, buf, rng);
        Self::fragment_2079(depth + 1, max_depth, buf, rng);
        Self::fragment_2080(depth + 1, max_depth, buf, rng);
        Self::fragment_2081(depth + 1, max_depth, buf, rng);
        Self::fragment_2082(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2414(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2090(depth + 1, max_depth, buf, rng);
        Self::fragment_2091(depth + 1, max_depth, buf, rng);
        Self::fragment_2092(depth + 1, max_depth, buf, rng);
        Self::fragment_2093(depth + 1, max_depth, buf, rng);
        Self::fragment_2094(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2416(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2102(depth + 1, max_depth, buf, rng);
        Self::fragment_2103(depth + 1, max_depth, buf, rng);
        Self::fragment_2104(depth + 1, max_depth, buf, rng);
        Self::fragment_2105(depth + 1, max_depth, buf, rng);
        Self::fragment_2106(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2418(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2110(depth + 1, max_depth, buf, rng);
        Self::fragment_2111(depth + 1, max_depth, buf, rng);
        Self::fragment_2112(depth + 1, max_depth, buf, rng);
        Self::fragment_2113(depth + 1, max_depth, buf, rng);
        Self::fragment_2114(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2420(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2124(depth + 1, max_depth, buf, rng);
        Self::fragment_2125(depth + 1, max_depth, buf, rng);
        Self::fragment_2126(depth + 1, max_depth, buf, rng);
        Self::fragment_2127(depth + 1, max_depth, buf, rng);
        Self::fragment_2128(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2422(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2134(depth + 1, max_depth, buf, rng);
        Self::fragment_2135(depth + 1, max_depth, buf, rng);
        Self::fragment_2136(depth + 1, max_depth, buf, rng);
        Self::fragment_2137(depth + 1, max_depth, buf, rng);
        Self::fragment_2138(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2424(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2142(depth + 1, max_depth, buf, rng);
        Self::fragment_2143(depth + 1, max_depth, buf, rng);
        Self::fragment_2144(depth + 1, max_depth, buf, rng);
        Self::fragment_2145(depth + 1, max_depth, buf, rng);
        Self::fragment_2146(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2426(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2175(depth + 1, max_depth, buf, rng);
        Self::fragment_2176(depth + 1, max_depth, buf, rng);
        Self::fragment_2177(depth + 1, max_depth, buf, rng);
        Self::fragment_2178(depth + 1, max_depth, buf, rng);
        Self::fragment_2179(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2428(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2220(depth + 1, max_depth, buf, rng);
        Self::fragment_2221(depth + 1, max_depth, buf, rng);
        Self::fragment_2222(depth + 1, max_depth, buf, rng);
        Self::fragment_2223(depth + 1, max_depth, buf, rng);
        Self::fragment_2224(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2430(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2228(depth + 1, max_depth, buf, rng);
        Self::fragment_2229(depth + 1, max_depth, buf, rng);
        Self::fragment_2230(depth + 1, max_depth, buf, rng);
        Self::fragment_2231(depth + 1, max_depth, buf, rng);
        Self::fragment_2232(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2432(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2240(depth + 1, max_depth, buf, rng);
        Self::fragment_2241(depth + 1, max_depth, buf, rng);
        Self::fragment_2242(depth + 1, max_depth, buf, rng);
        Self::fragment_2243(depth + 1, max_depth, buf, rng);
        Self::fragment_2244(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2434(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2250(depth + 1, max_depth, buf, rng);
        Self::fragment_2251(depth + 1, max_depth, buf, rng);
        Self::fragment_2252(depth + 1, max_depth, buf, rng);
        Self::fragment_2253(depth + 1, max_depth, buf, rng);
        Self::fragment_2254(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2436(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2260(depth + 1, max_depth, buf, rng);
        Self::fragment_2261(depth + 1, max_depth, buf, rng);
        Self::fragment_2262(depth + 1, max_depth, buf, rng);
        Self::fragment_2263(depth + 1, max_depth, buf, rng);
        Self::fragment_2264(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2438(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2278(depth + 1, max_depth, buf, rng);
        Self::fragment_2279(depth + 1, max_depth, buf, rng);
        Self::fragment_2280(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2440(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2286(depth + 1, max_depth, buf, rng);
        Self::fragment_2287(depth + 1, max_depth, buf, rng);
        Self::fragment_2288(depth + 1, max_depth, buf, rng);
        Self::fragment_2289(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2442(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2301(depth + 1, max_depth, buf, rng);
        Self::fragment_2302(depth + 1, max_depth, buf, rng);
        Self::fragment_2303(depth + 1, max_depth, buf, rng);
        Self::fragment_2304(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2444(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2310(depth + 1, max_depth, buf, rng);
        Self::fragment_2311(depth + 1, max_depth, buf, rng);
        Self::fragment_2312(depth + 1, max_depth, buf, rng);
        Self::fragment_2313(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2446(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2328(depth + 1, max_depth, buf, rng);
        Self::fragment_2329(depth + 1, max_depth, buf, rng);
        Self::fragment_2330(depth + 1, max_depth, buf, rng);
        Self::fragment_2331(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2448(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2498(depth + 1, max_depth, buf, rng);
        Self::fragment_2499(depth + 1, max_depth, buf, rng);
        Self::fragment_2500(depth + 1, max_depth, buf, rng);
        Self::fragment_2501(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2450(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2524(depth + 1, max_depth, buf, rng);
        Self::fragment_2525(depth + 1, max_depth, buf, rng);
        Self::fragment_2526(depth + 1, max_depth, buf, rng);
        Self::fragment_2527(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2452(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2533(depth + 1, max_depth, buf, rng);
        Self::fragment_2534(depth + 1, max_depth, buf, rng);
        Self::fragment_2535(depth + 1, max_depth, buf, rng);
        Self::fragment_2536(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2454(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2542(depth + 1, max_depth, buf, rng);
        Self::fragment_2543(depth + 1, max_depth, buf, rng);
        Self::fragment_2544(depth + 1, max_depth, buf, rng);
        Self::fragment_2545(depth + 1, max_depth, buf, rng);
        Self::fragment_2546(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2456(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2549(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2554(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2458(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2564(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2566(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2569(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2460(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2602(depth + 1, max_depth, buf, rng);
        Self::fragment_2603(depth + 1, max_depth, buf, rng);
        Self::fragment_2604(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2462(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2589(depth + 1, max_depth, buf, rng);
        Self::fragment_2590(depth + 1, max_depth, buf, rng);
        Self::fragment_2591(depth + 1, max_depth, buf, rng);
        Self::fragment_2592(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2464(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2626(depth + 1, max_depth, buf, rng);
        Self::fragment_2627(depth + 1, max_depth, buf, rng);
        Self::fragment_2628(depth + 1, max_depth, buf, rng);
        Self::fragment_2629(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2466(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2779(depth + 1, max_depth, buf, rng);
        Self::fragment_2780(depth + 1, max_depth, buf, rng);
        Self::fragment_2781(depth + 1, max_depth, buf, rng);
        Self::fragment_2782(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2468(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2193(depth + 1, max_depth, buf, rng);
        Self::fragment_2194(depth + 1, max_depth, buf, rng);
        Self::fragment_2195(depth + 1, max_depth, buf, rng);
        Self::fragment_2196(depth + 1, max_depth, buf, rng);
        Self::fragment_2197(depth + 1, max_depth, buf, rng);
        Self::fragment_2198(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2470(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "(<urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2> [\"I am an ETag\"]) ([\"I am another ETag\"])" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 90;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    40, 60, 117, 114, 110, 58, 117, 117, 105, 100, 58, 49, 56, 49, 100, 52, 102,
                    97, 101, 45, 55, 100, 56, 99, 45, 49, 49, 100, 48, 45, 97, 55, 54, 53, 45, 48,
                    48, 97, 48, 99, 57, 49, 101, 54, 98, 102, 50, 62, 32, 91, 34, 73, 32, 97, 109,
                    32, 97, 110, 32, 69, 84, 97, 103, 34, 93, 41, 32, 40, 91, 34, 73, 32, 97, 109,
                    32, 97, 110, 111, 116, 104, 101, 114, 32, 69, 84, 97, 103, 34, 93, 41,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                90,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2471(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "(Not <urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 52;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    40, 78, 111, 116, 32, 60, 117, 114, 110, 58, 117, 117, 105, 100, 58, 49, 56,
                    49, 100, 52, 102, 97, 101, 45, 55, 100, 56, 99, 45, 49, 49, 100, 48, 45, 97,
                    55, 54, 53, 45, 48, 48, 97, 48, 99, 57, 49, 101, 54, 98, 102, 50, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                52,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2472(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "<urn:uuid:58f202ac-22cf-11d1-b12d-002035b29092>)" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 48;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    60, 117, 114, 110, 58, 117, 117, 105, 100, 58, 53, 56, 102, 50, 48, 50, 97, 99,
                    45, 50, 50, 99, 102, 45, 49, 49, 100, 49, 45, 98, 49, 50, 100, 45, 48, 48, 50,
                    48, 51, 53, 98, 50, 57, 48, 57, 50, 62, 41,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                48,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2473(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2471(depth + 1, max_depth, buf, rng);
        Self::fragment_2472(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2475(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "gzip" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [103, 122, 105, 112].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2477(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "compress" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [99, 111, 109, 112, 114, 101, 115, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2479(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "deflate" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [100, 101, 102, 108, 97, 116, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2481(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "br" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [98, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2483(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "identity" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [105, 100, 101, 110, 116, 105, 116, 121].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2485(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "chunked" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [99, 104, 117, 110, 107, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2487(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "trailers" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [116, 114, 97, 105, 108, 101, 114, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2488(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_2475(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2477(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2479(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2481(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2483(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2485(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2487(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2489(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_2490(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2182(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2184(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2189(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2192(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2493(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2488(depth + 1, max_depth, buf, rng);
        Self::fragment_2489(depth + 1, max_depth, buf, rng);
        Self::fragment_2490(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2494(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2488(depth + 1, max_depth, buf, rng);
        Self::fragment_2489(depth + 1, max_depth, buf, rng);
        Self::fragment_2490(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2495(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2496(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2488(depth + 1, max_depth, buf, rng);
        Self::fragment_2489(depth + 1, max_depth, buf, rng);
        Self::fragment_2490(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2497(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2494(depth + 1, max_depth, buf, rng);
        Self::fragment_2495(depth + 1, max_depth, buf, rng);
        Self::fragment_2496(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2498(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "TE:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 69, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2499(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2500(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2493(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2497(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2501(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2503(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Seconds" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 101, 99, 111, 110, 100, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2504(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Hours" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [72, 111, 117, 114, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2505(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Days" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [68, 97, 121, 115].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2508(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Infinite" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 110, 102, 105, 110, 105, 116, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2510(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Second-4100000000" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    83, 101, 99, 111, 110, 100, 45, 52, 49, 48, 48, 48, 48, 48, 48, 48, 48,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2511(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Second-" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 101, 99, 111, 110, 100, 45].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2512(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2513(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2511(depth + 1, max_depth, buf, rng);
        Self::fragment_2512(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2514(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2503(depth + 1, max_depth, buf, rng);
        Self::fragment_2504(depth + 1, max_depth, buf, rng);
        Self::fragment_2505(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2515(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_2516(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2517(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2514(depth + 1, max_depth, buf, rng);
        Self::fragment_2515(depth + 1, max_depth, buf, rng);
        Self::fragment_2516(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2519(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2508(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2510(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2513(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2517(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2520(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2508(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2510(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2513(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2517(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2521(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2522(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2508(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2510(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2513(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2517(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2523(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2520(depth + 1, max_depth, buf, rng);
        Self::fragment_2521(depth + 1, max_depth, buf, rng);
        Self::fragment_2522(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2524(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Timeout:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 105, 109, 101, 111, 117, 116, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2525(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2526(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2519(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2523(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2527(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2530(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "upd" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [117, 112, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2532(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2533(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Topic:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 111, 112, 105, 99, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2534(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2535(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2530(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2532(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2536(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2539(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..69) {
            0 => Self::fragment_1690(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1692(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1694(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1696(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1698(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1700(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1702(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1704(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1706(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1708(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1710(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1712(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1714(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1716(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1718(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1720(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1722(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1724(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1726(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1728(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1730(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1732(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1734(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1736(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1738(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1740(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_1742(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_1744(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_1746(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_1748(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_1750(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_1752(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_1754(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_1756(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_1758(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_1760(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_1762(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_1764(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_1766(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_1768(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_1770(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_1772(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_1774(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_1776(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_1778(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_1780(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_1782(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_1784(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_1786(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_1788(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_1790(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_1792(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_1794(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_1796(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_1798(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_1800(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_1802(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_1804(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_1806(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_1808(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_1810(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_1812(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_1814(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_1816(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_1818(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_1820(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_1822(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_1824(depth + 1, max_depth, buf, rng),
            68 => Self::fragment_1826(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2541(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_699(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_702(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2542(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Trailer:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 114, 97, 105, 108, 101, 114, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2543(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2544(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2539(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2541(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2545(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!<!string.spaces>" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    33, 60, 33, 115, 116, 114, 105, 110, 103, 46, 115, 112, 97, 99, 101, 115, 62,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2546(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2549(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Transfer-Encoding: chunked\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 28;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    84, 114, 97, 110, 115, 102, 101, 114, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                    58, 32, 99, 104, 117, 110, 107, 101, 100, 13, 10,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                28,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2550(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Transfer-Encoding:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 18;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    84, 114, 97, 110, 115, 102, 101, 114, 45, 69, 110, 99, 111, 100, 105, 110, 103,
                    58,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                18,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2551(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2552(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2556(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2558(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2560(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2562(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2553(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2554(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2550(depth + 1, max_depth, buf, rng);
        Self::fragment_2551(depth + 1, max_depth, buf, rng);
        Self::fragment_2552(depth + 1, max_depth, buf, rng);
        Self::fragment_2553(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2556(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1574(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1576(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1578(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1580(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1582(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1584(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2558(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1574(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1576(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1578(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1580(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1582(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1584(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2560(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2562(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1574(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1576(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1578(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1580(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1582(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1584(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2564(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "TTL: 0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 84, 76, 58, 32, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2566(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "TTL: 1" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 84, 76, 58, 32, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2567(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "TTL: " */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [84, 84, 76, 58, 32].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2568(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_525(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_527(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_529(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_531(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_533(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_535(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_537(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_539(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_541(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_543(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_545(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_547(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_549(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_551(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_553(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_555(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_557(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_559(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2569(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2567(depth + 1, max_depth, buf, rng);
        Self::fragment_2568(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2571(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "websocket" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [119, 101, 98, 115, 111, 99, 107, 101, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2573(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "HTTP/2.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [72, 84, 84, 80, 47, 50, 46, 48].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2574(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "HTTP/" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [72, 84, 84, 80, 47].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2575(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1840(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1842(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1844(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1846(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1848(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1852(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1856(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2576(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2574(depth + 1, max_depth, buf, rng);
        Self::fragment_2575(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2578(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "SHTTP/1.3" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [83, 72, 84, 84, 80, 47, 49, 46, 51].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2580(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "IRC/6.9" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [73, 82, 67, 47, 54, 46, 57].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2582(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "RTA/x11" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 7;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [82, 84, 65, 47, 120, 49, 49].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                7,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2584(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_2571(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2573(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2576(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2578(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2580(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2582(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2585(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_2571(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2573(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2576(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2578(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2580(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2582(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2586(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2587(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_2571(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2573(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2576(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2578(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2580(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2582(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2588(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2585(depth + 1, max_depth, buf, rng);
        Self::fragment_2586(depth + 1, max_depth, buf, rng);
        Self::fragment_2587(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2589(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Upgrade:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [85, 112, 103, 114, 97, 100, 101, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2590(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2591(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2584(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2588(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2592(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2595(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "very-low" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [118, 101, 114, 121, 45, 108, 111, 119].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2597(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "low" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [108, 111, 119].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2599(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "normal" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [110, 111, 114, 109, 97, 108].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2601(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "high" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [104, 105, 103, 104].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2602(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Urgency:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [85, 114, 103, 101, 110, 99, 121, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2603(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2595(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2597(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2599(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2601(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2604(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2615(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "curl/7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 52;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    99, 117, 114, 108, 47, 55, 46, 49, 54, 46, 51, 32, 108, 105, 98, 99, 117, 114,
                    108, 47, 55, 46, 49, 54, 46, 51, 32, 79, 112, 101, 110, 83, 83, 76, 47, 48, 46,
                    57, 46, 55, 108, 32, 122, 108, 105, 98, 47, 49, 46, 50, 46, 51,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                52,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2617(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 111;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    77, 111, 122, 105, 108, 108, 97, 47, 53, 46, 48, 32, 40, 87, 105, 110, 100,
                    111, 119, 115, 32, 78, 84, 32, 49, 48, 46, 48, 59, 32, 87, 105, 110, 54, 52,
                    59, 32, 120, 54, 52, 41, 32, 65, 112, 112, 108, 101, 87, 101, 98, 75, 105, 116,
                    47, 53, 51, 55, 46, 51, 54, 32, 40, 75, 72, 84, 77, 76, 44, 32, 108, 105, 107,
                    101, 32, 71, 101, 99, 107, 111, 41, 32, 67, 104, 114, 111, 109, 101, 47, 49,
                    49, 49, 46, 48, 46, 48, 46, 48, 32, 83, 97, 102, 97, 114, 105, 47, 53, 51, 55,
                    46, 51, 54,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                111,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2619(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:104.0) Gecko/20100101 Firefox/104.0" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 84;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    77, 111, 122, 105, 108, 108, 97, 47, 53, 46, 48, 32, 40, 77, 97, 99, 105, 110,
                    116, 111, 115, 104, 59, 32, 73, 110, 116, 101, 108, 32, 77, 97, 99, 32, 79, 83,
                    32, 88, 32, 49, 48, 46, 49, 53, 59, 32, 114, 118, 58, 49, 48, 52, 46, 48, 41,
                    32, 71, 101, 99, 107, 111, 47, 50, 48, 49, 48, 48, 49, 48, 49, 32, 70, 105,
                    114, 101, 102, 111, 120, 47, 49, 48, 52, 46, 48,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                84,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2621(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.102 Safari/537.36 OPR/90.0.4480.84" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 123;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    77, 111, 122, 105, 108, 108, 97, 47, 53, 46, 48, 32, 40, 88, 49, 49, 59, 32,
                    76, 105, 110, 117, 120, 32, 120, 56, 54, 95, 54, 52, 41, 32, 65, 112, 112, 108,
                    101, 87, 101, 98, 75, 105, 116, 47, 53, 51, 55, 46, 51, 54, 32, 40, 75, 72, 84,
                    77, 76, 44, 32, 108, 105, 107, 101, 32, 71, 101, 99, 107, 111, 41, 32, 67, 104,
                    114, 111, 109, 101, 47, 49, 48, 52, 46, 48, 46, 53, 49, 49, 50, 46, 49, 48, 50,
                    32, 83, 97, 102, 97, 114, 105, 47, 53, 51, 55, 46, 51, 54, 32, 79, 80, 82, 47,
                    57, 48, 46, 48, 46, 52, 52, 56, 48, 46, 56, 52,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                123,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2623(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "whatever/13.37" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    119, 104, 97, 116, 101, 118, 101, 114, 47, 49, 51, 46, 51, 55,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2625(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2626(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "User-Agent:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [85, 115, 101, 114, 45, 65, 103, 101, 110, 116, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2627(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2628(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_2615(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2617(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2619(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2621(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2623(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2625(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2629(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2632(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "100 Continue" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [49, 48, 48, 32, 67, 111, 110, 116, 105, 110, 117, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2634(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "101 Switching Protocols" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 23;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 48, 49, 32, 83, 119, 105, 116, 99, 104, 105, 110, 103, 32, 80, 114, 111,
                    116, 111, 99, 111, 108, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                23,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2636(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "102 Processing" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 48, 50, 32, 80, 114, 111, 99, 101, 115, 115, 105, 110, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2638(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "103 Early Hints" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    49, 48, 51, 32, 69, 97, 114, 108, 121, 32, 72, 105, 110, 116, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2640(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "200 OK" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 6;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 48, 48, 32, 79, 75].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                6,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2642(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "201 Created" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 48, 49, 32, 67, 114, 101, 97, 116, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2644(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "202 Accepted" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 48, 50, 32, 65, 99, 99, 101, 112, 116, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2646(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "203 Non-Authoritative Information" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 33;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    50, 48, 51, 32, 78, 111, 110, 45, 65, 117, 116, 104, 111, 114, 105, 116, 97,
                    116, 105, 118, 101, 32, 73, 110, 102, 111, 114, 109, 97, 116, 105, 111, 110,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                33,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2648(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "204 No Content" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 14;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    50, 48, 52, 32, 78, 111, 32, 67, 111, 110, 116, 101, 110, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                14,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2650(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "205 Reset Content" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    50, 48, 53, 32, 82, 101, 115, 101, 116, 32, 67, 111, 110, 116, 101, 110, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2652(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "206 Partial Content" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    50, 48, 54, 32, 80, 97, 114, 116, 105, 97, 108, 32, 67, 111, 110, 116, 101,
                    110, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2654(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "207 Multi-Status" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    50, 48, 55, 32, 77, 117, 108, 116, 105, 45, 83, 116, 97, 116, 117, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2656(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "208 Already Reported" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    50, 48, 56, 32, 65, 108, 114, 101, 97, 100, 121, 32, 82, 101, 112, 111, 114,
                    116, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2658(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "226 IM Used" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 11;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [50, 50, 54, 32, 73, 77, 32, 85, 115, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                11,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2660(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "300 Multiple Choices" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    51, 48, 48, 32, 77, 117, 108, 116, 105, 112, 108, 101, 32, 67, 104, 111, 105,
                    99, 101, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2662(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "301 Moved Permanently" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    51, 48, 49, 32, 77, 111, 118, 101, 100, 32, 80, 101, 114, 109, 97, 110, 101,
                    110, 116, 108, 121,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2664(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "302 Found" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 9;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 48, 50, 32, 70, 111, 117, 110, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                9,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2666(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "303 See Other" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [51, 48, 51, 32, 83, 101, 101, 32, 79, 116, 104, 101, 114].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2668(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "304 Not Modified" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    51, 48, 52, 32, 78, 111, 116, 32, 77, 111, 100, 105, 102, 105, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2670(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "307 Temporary Redirect" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    51, 48, 55, 32, 84, 101, 109, 112, 111, 114, 97, 114, 121, 32, 82, 101, 100,
                    105, 114, 101, 99, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2672(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "308 Permanent Redirect" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    51, 48, 56, 32, 80, 101, 114, 109, 97, 110, 101, 110, 116, 32, 82, 101, 100,
                    105, 114, 101, 99, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2674(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "400 Bad Request" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 48, 48, 32, 66, 97, 100, 32, 82, 101, 113, 117, 101, 115, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2676(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "401 Unauthorized" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 48, 49, 32, 85, 110, 97, 117, 116, 104, 111, 114, 105, 122, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2678(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "402 Payment Required" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 48, 50, 32, 80, 97, 121, 109, 101, 110, 116, 32, 82, 101, 113, 117, 105,
                    114, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2680(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "403 Forbidden" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 48, 51, 32, 70, 111, 114, 98, 105, 100, 100, 101, 110].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2682(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "404 Not Found" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 48, 52, 32, 78, 111, 116, 32, 70, 111, 117, 110, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2684(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "405 Method Not Allowed" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 48, 53, 32, 77, 101, 116, 104, 111, 100, 32, 78, 111, 116, 32, 65, 108,
                    108, 111, 119, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2686(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "406 Not Acceptable" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 18;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 48, 54, 32, 78, 111, 116, 32, 65, 99, 99, 101, 112, 116, 97, 98, 108, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                18,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2688(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "407 Proxy Authentication Required" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 33;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 48, 55, 32, 80, 114, 111, 120, 121, 32, 65, 117, 116, 104, 101, 110, 116,
                    105, 99, 97, 116, 105, 111, 110, 32, 82, 101, 113, 117, 105, 114, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                33,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2690(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "408 Request Timeout" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 48, 56, 32, 82, 101, 113, 117, 101, 115, 116, 32, 84, 105, 109, 101, 111,
                    117, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2692(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "409 Conflict" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 48, 57, 32, 67, 111, 110, 102, 108, 105, 99, 116].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2694(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "410 Gone" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 8;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 49, 48, 32, 71, 111, 110, 101].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                8,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2696(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "411 Length Required" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 49, 49, 32, 76, 101, 110, 103, 116, 104, 32, 82, 101, 113, 117, 105, 114,
                    101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2698(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "412 Precondition Failed" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 23;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 49, 50, 32, 80, 114, 101, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32,
                    70, 97, 105, 108, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                23,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2700(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "413 Content Too Large" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 49, 51, 32, 67, 111, 110, 116, 101, 110, 116, 32, 84, 111, 111, 32, 76, 97,
                    114, 103, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2702(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "414 URI Too Long" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 49, 52, 32, 85, 82, 73, 32, 84, 111, 111, 32, 76, 111, 110, 103,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2704(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "415 Unsupported Media Type" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 26;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 49, 53, 32, 85, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 77,
                    101, 100, 105, 97, 32, 84, 121, 112, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                26,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2706(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "416 Range Not Satisfiable" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 25;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 49, 54, 32, 82, 97, 110, 103, 101, 32, 78, 111, 116, 32, 83, 97, 116, 105,
                    115, 102, 105, 97, 98, 108, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                25,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2708(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "417 Expectation Failed" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 22;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 49, 55, 32, 69, 120, 112, 101, 99, 116, 97, 116, 105, 111, 110, 32, 70, 97,
                    105, 108, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                22,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2710(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "418 I'm a teapot" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 49, 56, 32, 73, 39, 109, 32, 97, 32, 116, 101, 97, 112, 111, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2712(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "421 Misdirected Request" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 23;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 50, 49, 32, 77, 105, 115, 100, 105, 114, 101, 99, 116, 101, 100, 32, 82,
                    101, 113, 117, 101, 115, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                23,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2714(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "422 Unprocessable Content" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 25;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 50, 50, 32, 85, 110, 112, 114, 111, 99, 101, 115, 115, 97, 98, 108, 101,
                    32, 67, 111, 110, 116, 101, 110, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                25,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2716(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "423 Locked" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 10;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 50, 51, 32, 76, 111, 99, 107, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                10,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2718(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "424 Failed Dependency" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 50, 52, 32, 70, 97, 105, 108, 101, 100, 32, 68, 101, 112, 101, 110, 100,
                    101, 110, 99, 121,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2720(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "425 Too Early" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 13;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [52, 50, 53, 32, 84, 111, 111, 32, 69, 97, 114, 108, 121].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                13,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2722(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "426 Upgrade Required" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 20;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 50, 54, 32, 85, 112, 103, 114, 97, 100, 101, 32, 82, 101, 113, 117, 105,
                    114, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                20,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2724(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "428 Precondition Required" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 25;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 50, 56, 32, 80, 114, 101, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32,
                    82, 101, 113, 117, 105, 114, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                25,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2726(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "429 Too Many Requests" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 50, 57, 32, 84, 111, 111, 32, 77, 97, 110, 121, 32, 82, 101, 113, 117, 101,
                    115, 116, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2728(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "431 Request Header Fields Too Large" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 35;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 51, 49, 32, 82, 101, 113, 117, 101, 115, 116, 32, 72, 101, 97, 100, 101,
                    114, 32, 70, 105, 101, 108, 100, 115, 32, 84, 111, 111, 32, 76, 97, 114, 103,
                    101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                35,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2730(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "451 Unavailable For Legal Reasons" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 33;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    52, 53, 49, 32, 85, 110, 97, 118, 97, 105, 108, 97, 98, 108, 101, 32, 70, 111,
                    114, 32, 76, 101, 103, 97, 108, 32, 82, 101, 97, 115, 111, 110, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                33,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2732(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "500 Internal Server Error" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 25;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 48, 32, 73, 110, 116, 101, 114, 110, 97, 108, 32, 83, 101, 114, 118,
                    101, 114, 32, 69, 114, 114, 111, 114,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                25,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2734(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "501 Not Implemented" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 49, 32, 78, 111, 116, 32, 73, 109, 112, 108, 101, 109, 101, 110, 116,
                    101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2736(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "502 Bad Gateway" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 15;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 50, 32, 66, 97, 100, 32, 71, 97, 116, 101, 119, 97, 121,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                15,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2738(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "503 Service Unavailable" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 23;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 51, 32, 83, 101, 114, 118, 105, 99, 101, 32, 85, 110, 97, 118, 97, 105,
                    108, 97, 98, 108, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                23,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2740(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "504 Gateway Timeout" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 19;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 52, 32, 71, 97, 116, 101, 119, 97, 121, 32, 84, 105, 109, 101, 111,
                    117, 116,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                19,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2742(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "505 HTTP Version Not Supported" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 30;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 53, 32, 72, 84, 84, 80, 32, 86, 101, 114, 115, 105, 111, 110, 32, 78,
                    111, 116, 32, 83, 117, 112, 112, 111, 114, 116, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                30,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2744(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "506 Variant Also Negotiates" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 27;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 54, 32, 86, 97, 114, 105, 97, 110, 116, 32, 65, 108, 115, 111, 32, 78,
                    101, 103, 111, 116, 105, 97, 116, 101, 115,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                27,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2746(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "507 Insufficient Storage" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 24;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 55, 32, 73, 110, 115, 117, 102, 102, 105, 99, 105, 101, 110, 116, 32,
                    83, 116, 111, 114, 97, 103, 101,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                24,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2748(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "508 Loop Detected" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 17;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 48, 56, 32, 76, 111, 111, 112, 32, 68, 101, 116, 101, 99, 116, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                17,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2750(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "510 Not Extended" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 16;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 49, 48, 32, 78, 111, 116, 32, 69, 120, 116, 101, 110, 100, 101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                16,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2752(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "511 Network Authentication Required" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 35;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    53, 49, 49, 32, 78, 101, 116, 119, 111, 114, 107, 32, 65, 117, 116, 104, 101,
                    110, 116, 105, 99, 97, 116, 105, 111, 110, 32, 82, 101, 113, 117, 105, 114,
                    101, 100,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                35,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2760(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1840(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1842(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1844(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1846(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1848(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1852(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1856(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2761(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " fred" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 5;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [32, 102, 114, 101, 100].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                5,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2762(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2760(depth + 1, max_depth, buf, rng);
        Self::fragment_2761(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2763(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1840(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1842(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1844(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1846(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1848(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1852(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1856(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2764(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " whatever.example.com" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 21;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [
                    32, 119, 104, 97, 116, 101, 118, 101, 114, 46, 101, 120, 97, 109, 112, 108,
                    101, 46, 99, 111, 109,
                ]
                .as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                21,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2765(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2763(depth + 1, max_depth, buf, rng);
        Self::fragment_2764(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2766(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1840(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1842(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1844(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1846(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1848(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1852(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1856(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2767(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " example.com" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 12;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [32, 101, 120, 97, 109, 112, 108, 101, 46, 99, 111, 109].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                12,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2768(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2766(depth + 1, max_depth, buf, rng);
        Self::fragment_2767(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2769(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1840(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1842(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1844(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1846(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1848(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1852(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1856(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2770(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_2771(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1112(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1114(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1116(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1118(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1120(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2772(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2769(depth + 1, max_depth, buf, rng);
        Self::fragment_2770(depth + 1, max_depth, buf, rng);
        Self::fragment_2771(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2774(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2762(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2765(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2768(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2772(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2775(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2762(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2765(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2768(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2772(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2776(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2777(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2774(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2778(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2778(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2775(depth + 1, max_depth, buf, rng);
        Self::fragment_2776(depth + 1, max_depth, buf, rng);
        Self::fragment_2777(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2779(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Via:" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 4;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [86, 105, 97, 58].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                4,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2780(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2781(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2774(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2778(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2782(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2790(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2791(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2800(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2805(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2812(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2792(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2794(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_79(depth + 1, max_depth, buf, rng);
        Self::fragment_2790(depth + 1, max_depth, buf, rng);
        Self::fragment_2791(depth + 1, max_depth, buf, rng);
        Self::fragment_2792(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2795(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1663(depth + 1, max_depth, buf, rng);
        Self::fragment_1664(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2796(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1306(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1308(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1310(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1312(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2798(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..61) {
            0 => Self::fragment_2632(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2634(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2636(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2638(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2640(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2642(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2644(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2646(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2648(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2650(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2652(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2654(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2656(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2658(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2660(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2662(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2664(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2666(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2668(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2670(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2672(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2674(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2676(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2678(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2680(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2682(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2684(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2686(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2688(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2690(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2692(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2694(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2696(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2698(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2700(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2702(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2704(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2706(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2708(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2710(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2712(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2714(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2716(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2718(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2720(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2722(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2724(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2726(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2728(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2730(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2732(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2734(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2736(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2738(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2740(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2742(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2744(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2746(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2748(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2750(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2752(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2799(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2800(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2798(depth + 1, max_depth, buf, rng);
        Self::fragment_2799(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2801(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_316(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_319(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2802(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2803(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2804(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2805(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2801(depth + 1, max_depth, buf, rng);
        Self::fragment_2802(depth + 1, max_depth, buf, rng);
        Self::fragment_2803(depth + 1, max_depth, buf, rng);
        Self::fragment_2804(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2806(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2807(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2808(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_296(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_298(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_300(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_302(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_304(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_306(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_308(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_310(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_312(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_314(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2809(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_902(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_913(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2810(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_915(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_937(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_947(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2811(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\r\n" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 2;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [13, 10].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                2,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_2812(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2806(depth + 1, max_depth, buf, rng);
        Self::fragment_2807(depth + 1, max_depth, buf, rng);
        Self::fragment_2808(depth + 1, max_depth, buf, rng);
        Self::fragment_2809(depth + 1, max_depth, buf, rng);
        Self::fragment_2810(depth + 1, max_depth, buf, rng);
        Self::fragment_2811(depth + 1, max_depth, buf, rng);
    }
}
