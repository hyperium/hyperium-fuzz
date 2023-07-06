#![allow(unused)]
use rand::Rng;
use std::cell::Cell;

pub struct GrammarGenerator;

pub static TERMINALS: [&'static str; 465] = ["http://example.com", "http://example.com/example/sub", "http://example.com/example/sub?a=b", "http://example.com/example/sub?a=b&c=12345", "http://example.com/example/sub?a=b&c=12345#asdf-asdf", "http://example.com/example/sub#asdf-asdf", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "16", "31", "32", "33", "63", "64", "65", "255", "257", "256", "128", "127", "129", "65535", "65537", "65536", "32768", "32767", "32769", "4294967295", "4294967297", "4294967296", "2147483648", "2147483647", "2147483649", "281474976710655", "281474976710657", "281474976710656", "140737488355328", "140737488355327", "140737488355329", "18446744073709551615", "18446744073709551617", "18446744073709551616", "9223372036854775808", "9223372036854775807", "9223372036854775809", "340282366920938463463374607431768211455", "340282366920938463463374607431768211457", "340282366920938463463374607431768211456", "170141183460469231731687303715884105728", "170141183460469231731687303715884105727", "170141183460469231731687303715884105729", "115792089237316195423570985008687907853269984665640564039457584007913129639935", "115792089237316195423570985008687907853269984665640564039457584007913129639937", "115792089237316195423570985008687907853269984665640564039457584007913129639936", "57896044618658097711785492504343953926634992332820282019728792003956564819968", "57896044618658097711785492504343953926634992332820282019728792003956564819967", "57896044618658097711785492504343953926634992332820282019728792003956564819969", ".", "-", "e", "e+", "e-", "A", "B", "C", "D", "E", "F", "a", "b", "c", "d", "f", "0x", "0X", "h", "1234", "0001234", "0xdeadbeef", "0DEADBEEF", "0_1234_45667", "-1", "-9", "1234567890", "0123456789", "+", "/", "=", "==", "!", "#", "$", "%", "&", "\"", "(", ")", "*", ",", ":", ";", "<", ">", "?", "@", "[", "]", "^", "_", "`", "{", "|", "}", "~", " ", "Sun, 06 Nov 1994 08:49:37 GMT", "Sun, 06 Nov 2094 08:49:37 GMT", "Sun, 06 Nov 1900 08:49:37 GMT", "Sun, 45 Nov 2056 08:49:37 GMT", "Sun, 15 Nov 2006 08:49:37 GMT+10", "Sun, 15 Nov 2006 08:49:37 GMT+16", "Sun, 11 Nov 1111 11:11:11 GMT+11", "g", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Dec", "    ", "         ", "                ", "                 ", "08:49:37", "00:00:00", "01:01:01", "11:11:11", "GMT", "UTC", "GMT+", "UTC+", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "1991", "1091", "2091", "9999", "99999", "-1999", "example.com", "sub.example.com", "subsub.sub.example.com", "a.b.c.d.f.g.h.i.j.k.example.com", "http://", "https://", "ftp://", "file://", "Accept-Charset", "!<!string.spaces>", "\r\n", "Accept-Encoding:", "Accept-Language", "Accept-Ranges", "Accept:", "Allow", "Allow:", "ALPN", "ALPN:", "Alt-Used", "Alt-Used:", "Basic", "Bearer", "Digest", "HOBA", "Mutual", "Negotiate", "OAuth", "SCRAM-SHA-1", "SCRAM-SHA-256", "vapid", "Authorization", "Cache-Control:", "max-age=5", "max-stale=5", "min-fresh=5", "no-cache", "no-store", "no-transform", "only-if-cached", "CalDav-Timezones", "foo123.foocdn.example", "barcdn.example; trace=\"abcdef\"", "AnotherCDN; abc=123; def=\"456\"", "a=1", "CND-Loop", "utf-16", "utf-16BE", "utf-32", "utf-32BE", "us-ascii", "iso-8859-1", "utf-7", "utf-8", "AAAA", "BBBB", ";foo=bar", "Transfer-Encoding:", "chunked", "Content-Encoding", "Content-Language", "Content-Length: 200\r\n", "Content-Length: 0\r\n", "Content-Length: ", "Content-Location", "SID=31d4d96e407aad42", "PHPSESSID=298zf09hf012fh2; csrftoken=u32t4o3tb3gg43; _gat=1", "Cookie", "123456", "YWxhZGRpbjpvcGVuc2VzYW1l", "Date", "infinity", "Depth:", "Destination", "Early-Data", "gzip", "compress", "deflate", "br", "identity", "encoding-name", "\"xyzzy\"", "\"AAAAAAAAAAAAAAAAAAAAAAAAA\"", "Expect", "100-continue", "Expires:", "Forwarded", "by", "From", "webmaster@w3.org", "Accept", "Accept-Encoding", "Cache-Control", "CDN-Loop", "Content-Length", "Depth", "Expires", "HTTP2-Settings", "If", "If-Match", "If-Modified-Since", "If-None-Match", "If-Range", "If-Schedule-Tag-Match", "If-Unmodified-Since", "Link", "Max-Forwards", "MIME-Version", "OData-Isolation", "OData-MaxVersion", "OData-Version", "Ordering-Type", "Origin", "OSCORE", "Overwrite", "Position", "Pragma", "Prefer", "Proxy-Authorization", "Range", "Referer", "Schedule-Reply", "Sec-Token-Binding", "Sec-Websocket-Accept", "Sec-Websocket-Extensions", "Sec-Websocket-Key", "Sec-Websocket-Protocol", "Sec-Websocket-Version", "Slug", "TE", "Timeout", "Topic", "Trailer", "Transfer-Encoding", "TTL", "Urgency", "Upgrade", "User-Agent", "Via", "Server", "Last-Modified", "Transfer-Encoding: chunked\r\n", "Transfer-Encoding: identity\r\n", "Content-Length: 180\r\n", "Bar: Foo\r\n", "0.9", "1.0", "1.1", "2.0", "3.0", "HTTP/", "AAMAAABkAARAAAAAAAIAAAAA", "If-Match:", "If-Modified-Since:", "If-None-Match:", "If-Range:", "If-Schedule-Tag-Match:", "If-Unmodified-Since:", "fr", "en", "de", "Link:", "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "*/*", "application/octet-stream", "application/pdf", "application/pkcs8", "application/zip", "audio/mpeg", "audio/vorbis", "audio/example", "font/woff", "font/ttf", "font/otf", "image/jpeg", "image/png", "image/svg+xml", "model/3mf", "text/html", "video/mp4", "snapshot", "4.0", "DAV:unordered", "DAV:custom", "http://example.org/example.html", "null", "CSU", "AA", "first", "last", "after example.html", "respond-async", "wait=100", "handling=lenient", "http%2F1.1", "h2", "http%2F", "Proxy-Authorization:", "q=1.0", "q=0.0", "q=", "bytes", "none", "5-8", "5-", "Range:", "/example", "AIkAAgBBQLgtRpWFPN66kxhxGrtaKrzcMtHw7HV8", "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=", "deflate-stream", "mux", "max-channels:4; flow-control", "Sec-Websocket-Extensions:", "dGhlIHNhbXBsZSBub25jZQ==", "Sec-Websocket-Key:", "chat", "superchat", "Sec-Websocket-Protocol:", "13", "Sec-Websocket-Version:", "The Beach at S%C3%A8te", "Slug:", "(<urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2> [\"I am an ETag\"]) ([\"I am another ETag\"])", "(Not <urn:uuid:181d4fae-7d8c-11d0-a765-00a0c91e6bf2>", "<urn:uuid:58f202ac-22cf-11d1-b12d-002035b29092>)", "trailers", "TE:", "Seconds", "Hours", "Days", "Infinite", "Second-4100000000", "Second-", "Timeout:", "upd", "Topic:", "Trailer:", "TTL: 0", "TTL: 1", "TTL: ", "websocket", "HTTP/2.0", "SHTTP/1.3", "IRC/6.9", "RTA/x11", "Upgrade:", "very-low", "low", "normal", "high", "Urgency:", "curl/7.16.3 libcurl/7.16.3 OpenSSL/0.9.7l zlib/1.2.3", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:104.0) Gecko/20100101 Firefox/104.0", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.102 Safari/537.36 OPR/90.0.4480.84", "whatever/13.37", "User-Agent:", " fred", " whatever.example.com", " example.com", "Via:", ];

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
        Self::fragment_2798(depth + 1, max_depth, buf, rng);
        Self::fragment_2799(depth + 1, max_depth, buf, rng);
        Self::fragment_2800(depth + 1, max_depth, buf, rng);
    }
    fn fragment_101(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..8) {
            0 => Self::fragment_1966(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1968(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1970(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1972(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1974(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1976(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1978(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1980(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_220(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_222(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_224(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_226(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_228(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_230(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_259(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_260(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_261(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_262(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_259(depth + 1, max_depth, buf, rng);
        Self::fragment_260(depth + 1, max_depth, buf, rng);
        Self::fragment_261(depth + 1, max_depth, buf, rng);
    }
    fn fragment_263(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_264(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_265(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_266(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_263(depth + 1, max_depth, buf, rng);
        Self::fragment_264(depth + 1, max_depth, buf, rng);
        Self::fragment_265(depth + 1, max_depth, buf, rng);
    }
    fn fragment_268(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_814(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_816(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_818(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_820(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_822(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_824(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_826(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_828(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_830(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_832(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_834(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_836(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_838(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_840(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_842(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_844(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_846(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_848(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_850(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_854(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_856(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_858(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_860(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_862(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_864(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_270(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_982(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_984(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_986(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_988(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_990(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_992(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_994(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_996(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_998(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1000(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1002(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1004(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1006(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1008(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1010(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1012(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1014(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1016(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1018(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1020(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1022(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1024(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1026(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1028(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1030(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1032(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_295(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_297(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_299(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2" */
        buf.push(50);
    }
    fn fragment_301(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3" */
        buf.push(51);
    }
    fn fragment_303(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_305(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5" */
        buf.push(53);
    }
    fn fragment_307(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "6" */
        buf.push(54);
    }
    fn fragment_309(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "7" */
        buf.push(55);
    }
    fn fragment_311(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_313(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9" */
        buf.push(57);
    }
    fn fragment_315(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_316(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_317(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_318(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_316(depth + 1, max_depth, buf, rng);
        Self::fragment_317(depth + 1, max_depth, buf, rng);
    }
    fn fragment_320(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_322(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_324(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_326(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_328(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_330(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_332(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_334(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_336(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_338(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_340(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_342(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_344(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_346(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_348(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_350(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_352(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_354(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_356(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_358(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_360(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_362(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_364(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_366(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_368(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_370(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_372(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_374(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_376(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_378(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_380(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_382(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_384(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_386(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_388(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_390(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_392(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_394(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_396(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_398(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_400(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_402(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_404(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_406(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_408(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_410(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_412(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_414(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_416(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_418(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_420(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_422(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_424(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_425(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_426(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_427(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_428(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_425(depth + 1, max_depth, buf, rng);
        Self::fragment_426(depth + 1, max_depth, buf, rng);
        Self::fragment_427(depth + 1, max_depth, buf, rng);
    }
    fn fragment_429(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_430(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_431(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_432(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_433(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_429(depth + 1, max_depth, buf, rng);
        Self::fragment_430(depth + 1, max_depth, buf, rng);
        Self::fragment_431(depth + 1, max_depth, buf, rng);
        Self::fragment_432(depth + 1, max_depth, buf, rng);
    }
    fn fragment_434(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_435(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_436(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_437(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_434(depth + 1, max_depth, buf, rng);
        Self::fragment_435(depth + 1, max_depth, buf, rng);
        Self::fragment_436(depth + 1, max_depth, buf, rng);
    }
    fn fragment_438(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_439(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_440(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_441(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_442(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_438(depth + 1, max_depth, buf, rng);
        Self::fragment_439(depth + 1, max_depth, buf, rng);
        Self::fragment_440(depth + 1, max_depth, buf, rng);
        Self::fragment_441(depth + 1, max_depth, buf, rng);
    }
    fn fragment_443(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_444(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_445(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_446(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e" */
        buf.push(101);
    }
    fn fragment_447(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_448(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_443(depth + 1, max_depth, buf, rng);
        Self::fragment_444(depth + 1, max_depth, buf, rng);
        Self::fragment_445(depth + 1, max_depth, buf, rng);
        Self::fragment_446(depth + 1, max_depth, buf, rng);
        Self::fragment_447(depth + 1, max_depth, buf, rng);
    }
    fn fragment_449(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_450(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_451(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_452(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_453(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_454(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_449(depth + 1, max_depth, buf, rng);
        Self::fragment_450(depth + 1, max_depth, buf, rng);
        Self::fragment_451(depth + 1, max_depth, buf, rng);
        Self::fragment_452(depth + 1, max_depth, buf, rng);
        Self::fragment_453(depth + 1, max_depth, buf, rng);
    }
    fn fragment_455(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_456(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_457(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_458(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_459(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_460(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_455(depth + 1, max_depth, buf, rng);
        Self::fragment_456(depth + 1, max_depth, buf, rng);
        Self::fragment_457(depth + 1, max_depth, buf, rng);
        Self::fragment_458(depth + 1, max_depth, buf, rng);
        Self::fragment_459(depth + 1, max_depth, buf, rng);
    }
    fn fragment_462(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_464(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_466(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2" */
        buf.push(50);
    }
    fn fragment_468(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3" */
        buf.push(51);
    }
    fn fragment_470(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_472(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5" */
        buf.push(53);
    }
    fn fragment_474(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "6" */
        buf.push(54);
    }
    fn fragment_476(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "7" */
        buf.push(55);
    }
    fn fragment_478(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_480(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9" */
        buf.push(57);
    }
    fn fragment_482(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "A" */
        buf.push(65);
    }
    fn fragment_484(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "B" */
        buf.push(66);
    }
    fn fragment_486(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "C" */
        buf.push(67);
    }
    fn fragment_488(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "D" */
        buf.push(68);
    }
    fn fragment_490(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "E" */
        buf.push(69);
    }
    fn fragment_492(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "F" */
        buf.push(70);
    }
    fn fragment_494(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "a" */
        buf.push(97);
    }
    fn fragment_496(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "b" */
        buf.push(98);
    }
    fn fragment_498(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "c" */
        buf.push(99);
    }
    fn fragment_500(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "d" */
        buf.push(100);
    }
    fn fragment_502(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e" */
        buf.push(101);
    }
    fn fragment_504(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "f" */
        buf.push(102);
    }
    fn fragment_506(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_462(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_464(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_466(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_468(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_470(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_472(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_474(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_476(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_478(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_480(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_482(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_492(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_494(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_498(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_500(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_502(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_504(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_507(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_462(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_464(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_466(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_468(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_470(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_472(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_474(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_476(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_478(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_480(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_482(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_492(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_494(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_498(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_500(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_502(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_504(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_508(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_506(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_509(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_509(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_507(depth + 1, max_depth, buf, rng);
        Self::fragment_508(depth + 1, max_depth, buf, rng);
    }
    fn fragment_510(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_511(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_506(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_509(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_512(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_510(depth + 1, max_depth, buf, rng);
        Self::fragment_511(depth + 1, max_depth, buf, rng);
    }
    fn fragment_513(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_514(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_506(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_509(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_515(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_513(depth + 1, max_depth, buf, rng);
        Self::fragment_514(depth + 1, max_depth, buf, rng);
    }
    fn fragment_516(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_506(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_509(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_517(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "h" */
        buf.push(104);
    }
    fn fragment_518(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_516(depth + 1, max_depth, buf, rng);
        Self::fragment_517(depth + 1, max_depth, buf, rng);
    }
    fn fragment_520(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..53) {
            0 => Self::fragment_320(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_322(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_324(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_326(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_328(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_330(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_332(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_334(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_336(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_338(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_340(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_342(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_344(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_346(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_348(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_350(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_352(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_354(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_356(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_358(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_360(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_362(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_364(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_366(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_368(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_370(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_372(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_374(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_376(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_378(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_380(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_382(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_384(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_386(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_388(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_390(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_392(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_394(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_396(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_398(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_400(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_402(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_404(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_406(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_408(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_410(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_412(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_414(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_416(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_418(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_420(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_422(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_424(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_522(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_524(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_526(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_528(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_530(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_532(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_534(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_536(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_538(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_540(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_542(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_544(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_546(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_548(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_550(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_520(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_522(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_552(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_581(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_584(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_586(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_589(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_554(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_556(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_428(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_433(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_437(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_442(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_448(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_454(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_460(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_558(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_512(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_515(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_518(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_581(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..53) {
            0 => Self::fragment_320(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_322(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_324(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_326(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_328(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_330(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_332(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_334(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_336(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_338(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_340(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_342(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_344(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_346(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_348(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_350(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_352(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_354(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_356(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_358(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_360(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_362(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_364(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_366(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_368(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_370(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_372(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_374(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_376(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_378(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_380(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_382(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_384(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_386(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_388(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_390(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_392(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_394(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_396(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_398(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_400(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_402(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_404(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_406(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_408(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_410(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_412(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_414(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_416(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_418(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_420(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_422(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_424(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_582(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_583(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..53) {
            0 => Self::fragment_320(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_322(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_324(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_326(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_328(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_330(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_332(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_334(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_336(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_338(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_340(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_342(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_344(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_346(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_348(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_350(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_352(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_354(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_356(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_358(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_360(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_362(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_364(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_366(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_368(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_370(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_372(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_374(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_376(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_378(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_380(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_382(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_384(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_386(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_388(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_390(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_392(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_394(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_396(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_398(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_400(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_402(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_404(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_406(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_408(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_410(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_412(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_414(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_416(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_418(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_420(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_422(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_424(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_584(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_582(depth + 1, max_depth, buf, rng);
        Self::fragment_583(depth + 1, max_depth, buf, rng);
    }
    fn fragment_586(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_587(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_588(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_589(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_587(depth + 1, max_depth, buf, rng);
        Self::fragment_588(depth + 1, max_depth, buf, rng);
    }
    fn fragment_590(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_592(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "+" */
        buf.push(43);
    }
    fn fragment_594(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_596(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_598(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_600(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_268(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_270(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_590(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_592(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_594(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_601(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_268(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_270(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_590(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_592(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_594(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_602(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_596(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_598(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_603(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_601(depth + 1, max_depth, buf, rng);
        Self::fragment_602(depth + 1, max_depth, buf, rng);
    }
    fn fragment_604(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_268(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_270(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_590(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_592(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_594(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_605(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_600(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_603(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_606(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_604(depth + 1, max_depth, buf, rng);
        Self::fragment_605(depth + 1, max_depth, buf, rng);
    }
    fn fragment_608(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_814(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_816(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_818(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_820(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_822(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_824(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_826(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_828(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_830(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_832(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_834(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_836(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_838(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_840(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_842(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_844(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_846(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_848(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_850(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_854(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_856(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_858(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_860(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_862(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_864(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_610(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_982(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_984(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_986(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_988(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_990(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_992(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_994(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_996(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_998(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1000(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1002(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1004(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1006(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1008(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1010(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1012(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1014(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1016(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1018(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1020(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1022(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1024(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1026(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1028(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1030(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1032(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_612(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_735(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_737(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_739(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_741(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_743(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_745(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_747(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_749(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_751(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_753(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_614(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "!" */
        buf.push(33);
    }
    fn fragment_616(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "#" */
        buf.push(35);
    }
    fn fragment_618(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "$" */
        buf.push(36);
    }
    fn fragment_620(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "%" */
        buf.push(37);
    }
    fn fragment_622(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_624(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\"" */
        buf.push(34);
    }
    fn fragment_626(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "(" */
        buf.push(40);
    }
    fn fragment_628(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ")" */
        buf.push(41);
    }
    fn fragment_630(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        buf.push(42);
    }
    fn fragment_632(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "+" */
        buf.push(43);
    }
    fn fragment_634(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_636(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_638(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_640(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_642(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_644(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_646(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "<" */
        buf.push(60);
    }
    fn fragment_648(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_650(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ">" */
        buf.push(62);
    }
    fn fragment_652(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_654(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "@" */
        buf.push(64);
    }
    fn fragment_656(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "[" */
        buf.push(91);
    }
    fn fragment_658(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "]" */
        buf.push(93);
    }
    fn fragment_660(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "^" */
        buf.push(94);
    }
    fn fragment_662(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "_" */
        buf.push(95);
    }
    fn fragment_664(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "`" */
        buf.push(96);
    }
    fn fragment_666(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "{" */
        buf.push(123);
    }
    fn fragment_668(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "|" */
        buf.push(124);
    }
    fn fragment_670(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "}" */
        buf.push(125);
    }
    fn fragment_672(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "~" */
        buf.push(126);
    }
    fn fragment_674(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_675(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
    }
    fn fragment_677(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_678(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_679(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_677(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_680(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_687(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_680(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_678(depth + 1, max_depth, buf, rng);
        Self::fragment_679(depth + 1, max_depth, buf, rng);
    }
    fn fragment_681(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_682(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_683(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_684(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_685(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_695(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_686(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_677(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_680(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_687(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_687(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_681(depth + 1, max_depth, buf, rng);
        Self::fragment_682(depth + 1, max_depth, buf, rng);
        Self::fragment_683(depth + 1, max_depth, buf, rng);
        Self::fragment_684(depth + 1, max_depth, buf, rng);
        Self::fragment_685(depth + 1, max_depth, buf, rng);
        Self::fragment_686(depth + 1, max_depth, buf, rng);
    }
    fn fragment_689(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_814(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_816(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_818(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_820(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_822(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_824(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_826(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_828(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_830(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_832(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_834(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_836(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_838(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_840(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_842(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_844(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_846(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_848(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_850(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_854(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_856(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_858(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_860(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_862(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_864(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_691(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_982(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_984(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_986(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_988(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_990(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_992(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_994(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_996(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_998(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1000(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1002(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1004(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1006(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1008(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1010(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1012(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1014(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1016(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1018(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1020(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1022(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1024(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1026(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1028(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1030(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1032(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_693(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_695(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_696(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_814(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_816(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_818(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_820(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_822(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_824(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_826(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_828(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_830(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_832(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_834(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_836(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_838(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_840(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_842(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_844(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_846(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_848(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_850(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_854(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_856(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_858(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_860(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_862(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_864(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_697(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_677(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_680(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_687(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_698(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_696(depth + 1, max_depth, buf, rng);
        Self::fragment_697(depth + 1, max_depth, buf, rng);
    }
    fn fragment_699(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_982(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_984(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_986(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_988(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_990(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_992(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_994(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_996(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_998(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1000(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1002(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1004(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1006(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1008(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1010(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1012(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1014(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1016(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1018(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1020(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1022(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1024(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1026(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1028(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1030(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1032(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_700(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_675(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_677(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_680(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_687(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_701(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_699(depth + 1, max_depth, buf, rng);
        Self::fragment_700(depth + 1, max_depth, buf, rng);
    }
    fn fragment_703(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_705(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_707(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_709(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_711(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_713(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_715(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_716(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_880(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_882(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_884(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_886(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_888(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_890(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_892(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_894(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_896(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_898(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_717(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_718(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_719(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_720(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_262(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_266(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_721(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_1085(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1087(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1089(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1091(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1093(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1095(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1097(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1102(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1104(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_722(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_948(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_950(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_952(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_954(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_961(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_970(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_723(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_972(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_974(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_977(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_980(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_724(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_716(depth + 1, max_depth, buf, rng);
        Self::fragment_717(depth + 1, max_depth, buf, rng);
        Self::fragment_718(depth + 1, max_depth, buf, rng);
        Self::fragment_719(depth + 1, max_depth, buf, rng);
        Self::fragment_720(depth + 1, max_depth, buf, rng);
        Self::fragment_721(depth + 1, max_depth, buf, rng);
        Self::fragment_722(depth + 1, max_depth, buf, rng);
        Self::fragment_723(depth + 1, max_depth, buf, rng);
    }
    fn fragment_725(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_262(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_266(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_726(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_727(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_728(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_729(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_262(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_266(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_730(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_1085(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1087(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1089(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1091(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1093(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1095(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1097(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1102(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1104(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_731(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_948(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_950(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_952(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_954(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_961(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_970(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_732(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_262(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_266(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_733(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_725(depth + 1, max_depth, buf, rng);
        Self::fragment_726(depth + 1, max_depth, buf, rng);
        Self::fragment_727(depth + 1, max_depth, buf, rng);
        Self::fragment_728(depth + 1, max_depth, buf, rng);
        Self::fragment_729(depth + 1, max_depth, buf, rng);
        Self::fragment_730(depth + 1, max_depth, buf, rng);
        Self::fragment_731(depth + 1, max_depth, buf, rng);
        Self::fragment_732(depth + 1, max_depth, buf, rng);
    }
    fn fragment_735(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_737(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_739(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2" */
        buf.push(50);
    }
    fn fragment_741(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3" */
        buf.push(51);
    }
    fn fragment_743(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_745(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5" */
        buf.push(53);
    }
    fn fragment_747(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "6" */
        buf.push(54);
    }
    fn fragment_749(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "7" */
        buf.push(55);
    }
    fn fragment_751(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_753(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9" */
        buf.push(57);
    }
    fn fragment_814(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "a" */
        buf.push(97);
    }
    fn fragment_816(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "b" */
        buf.push(98);
    }
    fn fragment_818(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "c" */
        buf.push(99);
    }
    fn fragment_820(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "d" */
        buf.push(100);
    }
    fn fragment_822(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e" */
        buf.push(101);
    }
    fn fragment_824(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "f" */
        buf.push(102);
    }
    fn fragment_826(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "g" */
        buf.push(103);
    }
    fn fragment_828(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "h" */
        buf.push(104);
    }
    fn fragment_830(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "i" */
        buf.push(105);
    }
    fn fragment_832(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "j" */
        buf.push(106);
    }
    fn fragment_834(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "k" */
        buf.push(107);
    }
    fn fragment_836(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "l" */
        buf.push(108);
    }
    fn fragment_838(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "m" */
        buf.push(109);
    }
    fn fragment_840(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "n" */
        buf.push(110);
    }
    fn fragment_842(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "o" */
        buf.push(111);
    }
    fn fragment_844(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "p" */
        buf.push(112);
    }
    fn fragment_846(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "q" */
        buf.push(113);
    }
    fn fragment_848(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "r" */
        buf.push(114);
    }
    fn fragment_850(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "s" */
        buf.push(115);
    }
    fn fragment_852(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "t" */
        buf.push(116);
    }
    fn fragment_854(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "u" */
        buf.push(117);
    }
    fn fragment_856(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "v" */
        buf.push(118);
    }
    fn fragment_858(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "w" */
        buf.push(119);
    }
    fn fragment_860(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "x" */
        buf.push(120);
    }
    fn fragment_862(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "y" */
        buf.push(121);
    }
    fn fragment_864(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "z" */
        buf.push(122);
    }
    fn fragment_880(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_882(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_884(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_886(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_888(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_890(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_892(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_894(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_896(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_898(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_899(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
    }
    fn fragment_901(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_903(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_905(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_907(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_909(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_910(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_911(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_912(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_910(depth + 1, max_depth, buf, rng);
        Self::fragment_911(depth + 1, max_depth, buf, rng);
    }
    fn fragment_914(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_915(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_916(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_917(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_918(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_919(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_920(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_921(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_922(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_923(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_915(depth + 1, max_depth, buf, rng);
        Self::fragment_916(depth + 1, max_depth, buf, rng);
        Self::fragment_917(depth + 1, max_depth, buf, rng);
        Self::fragment_918(depth + 1, max_depth, buf, rng);
        Self::fragment_919(depth + 1, max_depth, buf, rng);
        Self::fragment_920(depth + 1, max_depth, buf, rng);
        Self::fragment_921(depth + 1, max_depth, buf, rng);
        Self::fragment_922(depth + 1, max_depth, buf, rng);
    }
    fn fragment_924(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_925(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_926(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_927(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_928(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_929(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_930(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_931(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_932(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_933(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_924(depth + 1, max_depth, buf, rng);
        Self::fragment_925(depth + 1, max_depth, buf, rng);
        Self::fragment_926(depth + 1, max_depth, buf, rng);
        Self::fragment_927(depth + 1, max_depth, buf, rng);
        Self::fragment_928(depth + 1, max_depth, buf, rng);
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
        Self::fragment_932(depth + 1, max_depth, buf, rng);
    }
    fn fragment_934(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_935(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_936(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_934(depth + 1, max_depth, buf, rng);
        Self::fragment_935(depth + 1, max_depth, buf, rng);
    }
    fn fragment_937(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_938(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_939(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_940(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_941(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_942(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_943(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_944(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..34) {
            0 => Self::fragment_608(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_610(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_612(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_614(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_616(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_618(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_620(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_622(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_624(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_626(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_628(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_630(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_632(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_634(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_636(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_638(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_640(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_642(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_644(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_646(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_648(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_650(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_652(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_654(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_656(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_658(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_660(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_662(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_664(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_666(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_668(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_670(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_672(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_674(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_945(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_946(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_937(depth + 1, max_depth, buf, rng);
        Self::fragment_938(depth + 1, max_depth, buf, rng);
        Self::fragment_939(depth + 1, max_depth, buf, rng);
        Self::fragment_940(depth + 1, max_depth, buf, rng);
        Self::fragment_941(depth + 1, max_depth, buf, rng);
        Self::fragment_942(depth + 1, max_depth, buf, rng);
        Self::fragment_943(depth + 1, max_depth, buf, rng);
        Self::fragment_944(depth + 1, max_depth, buf, rng);
        Self::fragment_945(depth + 1, max_depth, buf, rng);
    }
    fn fragment_948(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_950(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_952(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_954(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_955(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_956(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_957(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_958(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_959(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_960(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_961(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_955(depth + 1, max_depth, buf, rng);
        Self::fragment_956(depth + 1, max_depth, buf, rng);
        Self::fragment_957(depth + 1, max_depth, buf, rng);
        Self::fragment_958(depth + 1, max_depth, buf, rng);
        Self::fragment_959(depth + 1, max_depth, buf, rng);
        Self::fragment_960(depth + 1, max_depth, buf, rng);
    }
    fn fragment_962(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_963(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_964(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_965(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_966(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_967(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_968(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_969(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_970(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_962(depth + 1, max_depth, buf, rng);
        Self::fragment_963(depth + 1, max_depth, buf, rng);
        Self::fragment_964(depth + 1, max_depth, buf, rng);
        Self::fragment_965(depth + 1, max_depth, buf, rng);
        Self::fragment_966(depth + 1, max_depth, buf, rng);
        Self::fragment_967(depth + 1, max_depth, buf, rng);
        Self::fragment_968(depth + 1, max_depth, buf, rng);
        Self::fragment_969(depth + 1, max_depth, buf, rng);
    }
    fn fragment_972(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_974(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_975(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_976(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_977(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_975(depth + 1, max_depth, buf, rng);
        Self::fragment_976(depth + 1, max_depth, buf, rng);
    }
    fn fragment_978(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_979(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_980(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_978(depth + 1, max_depth, buf, rng);
        Self::fragment_979(depth + 1, max_depth, buf, rng);
    }
    fn fragment_982(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "A" */
        buf.push(65);
    }
    fn fragment_984(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "B" */
        buf.push(66);
    }
    fn fragment_986(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "C" */
        buf.push(67);
    }
    fn fragment_988(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "D" */
        buf.push(68);
    }
    fn fragment_990(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "E" */
        buf.push(69);
    }
    fn fragment_992(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "F" */
        buf.push(70);
    }
    fn fragment_994(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "G" */
        buf.push(71);
    }
    fn fragment_996(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "H" */
        buf.push(72);
    }
    fn fragment_998(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "I" */
        buf.push(73);
    }
    fn fragment_1000(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "J" */
        buf.push(74);
    }
    fn fragment_1002(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "K" */
        buf.push(75);
    }
    fn fragment_1004(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "L" */
        buf.push(76);
    }
    fn fragment_1006(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "M" */
        buf.push(77);
    }
    fn fragment_1008(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "N" */
        buf.push(78);
    }
    fn fragment_1010(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "O" */
        buf.push(79);
    }
    fn fragment_1012(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "P" */
        buf.push(80);
    }
    fn fragment_1014(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Q" */
        buf.push(81);
    }
    fn fragment_1016(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "R" */
        buf.push(82);
    }
    fn fragment_1018(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "S" */
        buf.push(83);
    }
    fn fragment_1020(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "T" */
        buf.push(84);
    }
    fn fragment_1022(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "U" */
        buf.push(85);
    }
    fn fragment_1024(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "V" */
        buf.push(86);
    }
    fn fragment_1026(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "W" */
        buf.push(87);
    }
    fn fragment_1028(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "X" */
        buf.push(88);
    }
    fn fragment_1030(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Y" */
        buf.push(89);
    }
    fn fragment_1032(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Z" */
        buf.push(90);
    }
    fn fragment_1034(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_814(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_816(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_818(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_820(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_822(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_824(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_826(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_828(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_830(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_832(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_834(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_836(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_838(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_840(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_842(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_844(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_846(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_848(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_850(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_854(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_856(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_858(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_860(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_862(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_864(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1036(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_982(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_984(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_986(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_988(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_990(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_992(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_994(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_996(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_998(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1000(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1002(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1004(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1006(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1008(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1010(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1012(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1014(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1016(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1018(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1020(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1022(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1024(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1026(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1028(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1030(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1032(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1038(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1040(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_1042(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1043(depth + 1, max_depth, buf, rng);
        Self::fragment_1044(depth + 1, max_depth, buf, rng);
        Self::fragment_1045(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1043(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "%" */
        buf.push(37);
    }
    fn fragment_1044(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_462(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_464(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_466(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_468(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_470(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_472(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_474(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_476(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_478(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_480(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_482(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_492(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_494(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_498(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_500(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_502(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_504(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1045(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_462(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_464(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_466(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_468(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_470(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_472(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_474(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_476(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_478(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_480(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_482(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_492(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_494(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_498(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_500(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_502(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_504(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1048(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1049(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1050(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1051(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1052(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1053(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1054(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1055(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1056(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1057(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1049(depth + 1, max_depth, buf, rng);
        Self::fragment_1050(depth + 1, max_depth, buf, rng);
        Self::fragment_1051(depth + 1, max_depth, buf, rng);
        Self::fragment_1052(depth + 1, max_depth, buf, rng);
        Self::fragment_1053(depth + 1, max_depth, buf, rng);
        Self::fragment_1054(depth + 1, max_depth, buf, rng);
        Self::fragment_1055(depth + 1, max_depth, buf, rng);
        Self::fragment_1056(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1058(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1059(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1048(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1057(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1060(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1070(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1060(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1058(depth + 1, max_depth, buf, rng);
        Self::fragment_1059(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1061(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1062(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1063(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1064(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1065(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1066(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1067(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1068(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1034(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1036(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1038(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1040(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1042(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1069(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1048(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1057(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1060(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1070(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1070(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1061(depth + 1, max_depth, buf, rng);
        Self::fragment_1062(depth + 1, max_depth, buf, rng);
        Self::fragment_1063(depth + 1, max_depth, buf, rng);
        Self::fragment_1064(depth + 1, max_depth, buf, rng);
        Self::fragment_1065(depth + 1, max_depth, buf, rng);
        Self::fragment_1066(depth + 1, max_depth, buf, rng);
        Self::fragment_1067(depth + 1, max_depth, buf, rng);
        Self::fragment_1068(depth + 1, max_depth, buf, rng);
        Self::fragment_1069(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1085(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1087(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1089(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1091(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1093(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1095(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1097(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1098(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1099(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1100(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1101(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1102(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1098(depth + 1, max_depth, buf, rng);
        Self::fragment_1099(depth + 1, max_depth, buf, rng);
        Self::fragment_1100(depth + 1, max_depth, buf, rng);
        Self::fragment_1101(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1104(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1105(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1106(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1107(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_1108(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1105(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1109(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1109(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1106(depth + 1, max_depth, buf, rng);
        Self::fragment_1107(depth + 1, max_depth, buf, rng);
        Self::fragment_1108(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1111(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1113(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1115(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1117(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1119(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1105(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1109(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1121(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1122(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1123(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_1124(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1121(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1125(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1125(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1122(depth + 1, max_depth, buf, rng);
        Self::fragment_1123(depth + 1, max_depth, buf, rng);
        Self::fragment_1124(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1129(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1134(depth + 1, max_depth, buf, rng);
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1130(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1134(depth + 1, max_depth, buf, rng);
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1131(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_1132(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1129(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1133(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1133(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1130(depth + 1, max_depth, buf, rng);
        Self::fragment_1131(depth + 1, max_depth, buf, rng);
        Self::fragment_1132(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1134(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1048(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1057(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1060(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1070(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1135(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_1136(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1139(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1141(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1139(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_600(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_603(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1141(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1048(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1057(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1060(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1070(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1142(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_1143(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1134(depth + 1, max_depth, buf, rng);
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1144(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1142(depth + 1, max_depth, buf, rng);
        Self::fragment_1143(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1145(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_1146(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1134(depth + 1, max_depth, buf, rng);
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1147(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_1148(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1134(depth + 1, max_depth, buf, rng);
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1149(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1145(depth + 1, max_depth, buf, rng);
        Self::fragment_1146(depth + 1, max_depth, buf, rng);
        Self::fragment_1147(depth + 1, max_depth, buf, rng);
        Self::fragment_1148(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1150(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_1151(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1134(depth + 1, max_depth, buf, rng);
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1152(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_1153(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1134(depth + 1, max_depth, buf, rng);
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1154(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_1155(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1134(depth + 1, max_depth, buf, rng);
        Self::fragment_1135(depth + 1, max_depth, buf, rng);
        Self::fragment_1136(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1156(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1150(depth + 1, max_depth, buf, rng);
        Self::fragment_1151(depth + 1, max_depth, buf, rng);
        Self::fragment_1152(depth + 1, max_depth, buf, rng);
        Self::fragment_1153(depth + 1, max_depth, buf, rng);
        Self::fragment_1154(depth + 1, max_depth, buf, rng);
        Self::fragment_1155(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1157(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_1158(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1129(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1133(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1159(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1157(depth + 1, max_depth, buf, rng);
        Self::fragment_1158(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1161(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1163(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1165(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1167(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1169(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_220(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_222(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_224(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_226(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_228(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_230(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1170(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1161(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1163(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1165(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1167(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1171(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1111(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1113(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1115(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1117(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1119(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1172(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_1173(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1121(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1125(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1174(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1170(depth + 1, max_depth, buf, rng);
        Self::fragment_1171(depth + 1, max_depth, buf, rng);
        Self::fragment_1172(depth + 1, max_depth, buf, rng);
        Self::fragment_1173(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1175(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1161(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1163(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1165(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1167(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1176(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1111(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1113(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1115(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1117(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1119(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1177(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_1178(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1121(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1125(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1179(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1144(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1149(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1156(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1159(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1180(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1175(depth + 1, max_depth, buf, rng);
        Self::fragment_1176(depth + 1, max_depth, buf, rng);
        Self::fragment_1177(depth + 1, max_depth, buf, rng);
        Self::fragment_1178(depth + 1, max_depth, buf, rng);
        Self::fragment_1179(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1188(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1189(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1190(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1191(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1192(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1400(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1406(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1193(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1195(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1196(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1197(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1591(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1595(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1198(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1200(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1201(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1202(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1203(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1204(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1923(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1927(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1205(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1207(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1208(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1209(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1210(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1211(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2208(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2210(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1212(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1214(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_1990(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1992(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1994(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1996(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1998(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2000(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2002(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2004(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2006(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2008(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2010(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2012(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2014(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2016(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2018(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2020(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2022(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2024(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1215(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1216(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2181(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2183(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2188(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2191(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1219(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1214(depth + 1, max_depth, buf, rng);
        Self::fragment_1215(depth + 1, max_depth, buf, rng);
        Self::fragment_1216(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1220(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1214(depth + 1, max_depth, buf, rng);
        Self::fragment_1215(depth + 1, max_depth, buf, rng);
        Self::fragment_1216(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1221(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1222(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1214(depth + 1, max_depth, buf, rng);
        Self::fragment_1215(depth + 1, max_depth, buf, rng);
        Self::fragment_1216(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1223(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1220(depth + 1, max_depth, buf, rng);
        Self::fragment_1221(depth + 1, max_depth, buf, rng);
        Self::fragment_1222(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1224(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1225(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1226(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1219(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1223(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1227(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1228(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1237(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1238(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1239(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1240(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1241(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1982(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1986(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1242(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1243(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1237(depth + 1, max_depth, buf, rng);
        Self::fragment_1238(depth + 1, max_depth, buf, rng);
        Self::fragment_1239(depth + 1, max_depth, buf, rng);
        Self::fragment_1240(depth + 1, max_depth, buf, rng);
        Self::fragment_1241(depth + 1, max_depth, buf, rng);
        Self::fragment_1242(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1244(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1245(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1246(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1982(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1986(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1247(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1248(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1244(depth + 1, max_depth, buf, rng);
        Self::fragment_1245(depth + 1, max_depth, buf, rng);
        Self::fragment_1246(depth + 1, max_depth, buf, rng);
        Self::fragment_1247(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1249(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1250(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1251(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1252(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1253(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2173(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1254(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1255(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1249(depth + 1, max_depth, buf, rng);
        Self::fragment_1250(depth + 1, max_depth, buf, rng);
        Self::fragment_1251(depth + 1, max_depth, buf, rng);
        Self::fragment_1252(depth + 1, max_depth, buf, rng);
        Self::fragment_1253(depth + 1, max_depth, buf, rng);
        Self::fragment_1254(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1256(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1257(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1258(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2173(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1259(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1260(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1256(depth + 1, max_depth, buf, rng);
        Self::fragment_1257(depth + 1, max_depth, buf, rng);
        Self::fragment_1258(depth + 1, max_depth, buf, rng);
        Self::fragment_1259(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1263(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1264(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1265(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1266(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1267(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1174(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1180(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1268(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1269(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1263(depth + 1, max_depth, buf, rng);
        Self::fragment_1264(depth + 1, max_depth, buf, rng);
        Self::fragment_1265(depth + 1, max_depth, buf, rng);
        Self::fragment_1266(depth + 1, max_depth, buf, rng);
        Self::fragment_1267(depth + 1, max_depth, buf, rng);
        Self::fragment_1268(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1270(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1271(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1272(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1174(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1180(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1273(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1274(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1270(depth + 1, max_depth, buf, rng);
        Self::fragment_1271(depth + 1, max_depth, buf, rng);
        Self::fragment_1272(depth + 1, max_depth, buf, rng);
        Self::fragment_1273(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1276(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1278(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1280(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1282(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1284(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1286(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1288(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1290(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1292(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1294(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1295(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1296(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1297(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1298(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_1276(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1278(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1280(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1282(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1284(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1286(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1288(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1290(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1292(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1294(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1299(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1300(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1531(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1533(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1535(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1301(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1302(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1305(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1448(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1453(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1307(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1048(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1057(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1060(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1070(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1309(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1311(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_600(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_603(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1313(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "T" */
        buf.push(84);
    }
    fn fragment_1315(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "F" */
        buf.push(70);
    }
    fn fragment_1318(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1319(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1320(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1338(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1344(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1321(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1324(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1326(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1328(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1330(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1332(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1334(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1336(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1338(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1324(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1326(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1328(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1330(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1332(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1334(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1336(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1339(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1324(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1326(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1328(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1330(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1332(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1334(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1336(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1340(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1341(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1342(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1343(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1324(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1326(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1328(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1330(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1332(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1334(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1336(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1344(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1339(depth + 1, max_depth, buf, rng);
        Self::fragment_1340(depth + 1, max_depth, buf, rng);
        Self::fragment_1341(depth + 1, max_depth, buf, rng);
        Self::fragment_1342(depth + 1, max_depth, buf, rng);
        Self::fragment_1343(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1345(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1346(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1347(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1348(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1349(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1313(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1315(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1350(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1353(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1355(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1357(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1359(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1111(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1113(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1115(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1117(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1119(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1360(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1111(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1113(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1115(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1117(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1119(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1361(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1362(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1363(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1360(depth + 1, max_depth, buf, rng);
        Self::fragment_1361(depth + 1, max_depth, buf, rng);
        Self::fragment_1362(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1365(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1353(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1355(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1357(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1359(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1363(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1366(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1353(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1355(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1357(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1359(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1363(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1367(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1368(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1353(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1355(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1357(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1359(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1363(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1369(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1366(depth + 1, max_depth, buf, rng);
        Self::fragment_1367(depth + 1, max_depth, buf, rng);
        Self::fragment_1368(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1370(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1371(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1372(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1373(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1374(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1365(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1369(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1375(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1378(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1380(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1382(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1384(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1386(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1388(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1390(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1392(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1393(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..8) {
            0 => Self::fragment_1378(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1380(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1382(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1384(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1386(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1388(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1390(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1392(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1394(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1395(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1396(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1397(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2181(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2183(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2188(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2191(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1400(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1393(depth + 1, max_depth, buf, rng);
        Self::fragment_1394(depth + 1, max_depth, buf, rng);
        Self::fragment_1395(depth + 1, max_depth, buf, rng);
        Self::fragment_1396(depth + 1, max_depth, buf, rng);
        Self::fragment_1397(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1401(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1393(depth + 1, max_depth, buf, rng);
        Self::fragment_1394(depth + 1, max_depth, buf, rng);
        Self::fragment_1395(depth + 1, max_depth, buf, rng);
        Self::fragment_1396(depth + 1, max_depth, buf, rng);
        Self::fragment_1397(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1402(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1403(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1404(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1405(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1393(depth + 1, max_depth, buf, rng);
        Self::fragment_1394(depth + 1, max_depth, buf, rng);
        Self::fragment_1395(depth + 1, max_depth, buf, rng);
        Self::fragment_1396(depth + 1, max_depth, buf, rng);
        Self::fragment_1397(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1406(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1401(depth + 1, max_depth, buf, rng);
        Self::fragment_1402(depth + 1, max_depth, buf, rng);
        Self::fragment_1403(depth + 1, max_depth, buf, rng);
        Self::fragment_1404(depth + 1, max_depth, buf, rng);
        Self::fragment_1405(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1408(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1410(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1412(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1414(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_600(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_603(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1416(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1417(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1418(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1129(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1133(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1419(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1417(depth + 1, max_depth, buf, rng);
        Self::fragment_1418(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1421(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1439(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1444(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1422(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1439(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1444(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1423(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1424(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1421(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1425(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1425(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1422(depth + 1, max_depth, buf, rng);
        Self::fragment_1423(depth + 1, max_depth, buf, rng);
        Self::fragment_1424(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1427(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_1429(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1431(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_520(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_522(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1433(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1434(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1427(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1429(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1431(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1433(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1435(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1416(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1419(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1436(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1437(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1408(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1410(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1412(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1414(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1438(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1439(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1434(depth + 1, max_depth, buf, rng);
        Self::fragment_1435(depth + 1, max_depth, buf, rng);
        Self::fragment_1436(depth + 1, max_depth, buf, rng);
        Self::fragment_1437(depth + 1, max_depth, buf, rng);
        Self::fragment_1438(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1440(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1427(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1429(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1431(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1433(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1441(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1442(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1408(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1410(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1412(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1414(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1443(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1444(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1440(depth + 1, max_depth, buf, rng);
        Self::fragment_1441(depth + 1, max_depth, buf, rng);
        Self::fragment_1442(depth + 1, max_depth, buf, rng);
        Self::fragment_1443(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1445(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1421(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1425(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1446(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1931(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1934(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1447(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1448(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1445(depth + 1, max_depth, buf, rng);
        Self::fragment_1446(depth + 1, max_depth, buf, rng);
        Self::fragment_1447(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1449(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1421(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1425(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1450(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1931(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1934(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1451(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1827(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1829(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1831(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1452(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1453(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1449(depth + 1, max_depth, buf, rng);
        Self::fragment_1450(depth + 1, max_depth, buf, rng);
        Self::fragment_1451(depth + 1, max_depth, buf, rng);
        Self::fragment_1452(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1454(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1455(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1456(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1457(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1459(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1460(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1461(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1462(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1463(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2555(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2557(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2559(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2561(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1464(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1466(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1467(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1468(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1469(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1470(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1923(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1927(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1471(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1474(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1476(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1477(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1478(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1479(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1480(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1477(depth + 1, max_depth, buf, rng);
        Self::fragment_1478(depth + 1, max_depth, buf, rng);
        Self::fragment_1479(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1482(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1174(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1180(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1484(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2234(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2236(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1485(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1486(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1487(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1488(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1489(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1482(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1484(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1490(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1500(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1508(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1510(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1514(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1518(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1522(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1501(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1508(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1510(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1514(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1518(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1522(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1502(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1503(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1504(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1505(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1500(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1506(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1506(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1501(depth + 1, max_depth, buf, rng);
        Self::fragment_1502(depth + 1, max_depth, buf, rng);
        Self::fragment_1503(depth + 1, max_depth, buf, rng);
        Self::fragment_1504(depth + 1, max_depth, buf, rng);
        Self::fragment_1505(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1508(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1510(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1511(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1512(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_1513(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1514(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1511(depth + 1, max_depth, buf, rng);
        Self::fragment_1512(depth + 1, max_depth, buf, rng);
        Self::fragment_1513(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1515(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1516(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_1517(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1129(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1133(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1518(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1515(depth + 1, max_depth, buf, rng);
        Self::fragment_1516(depth + 1, max_depth, buf, rng);
        Self::fragment_1517(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1519(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1520(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_1521(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_600(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_603(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1522(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1519(depth + 1, max_depth, buf, rng);
        Self::fragment_1520(depth + 1, max_depth, buf, rng);
        Self::fragment_1521(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1523(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1524(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1525(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1526(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1527(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1500(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1506(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1528(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1531(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1533(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1535(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_600(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_603(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1536(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1537(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1538(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1539(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1540(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_703(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_705(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_707(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_709(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_711(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_713(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_715(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_724(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_733(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1541(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1544(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1546(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_1548(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1550(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1551(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1552(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1553(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1544(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1546(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1548(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1550(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1554(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1556(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1557(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1558(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1559(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1174(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1180(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1560(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1563(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_1565(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1566(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1567(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1568(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1569(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1563(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1565(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1570(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1573(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1575(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1577(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1579(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1581(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1583(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1585(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1586(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1573(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1575(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1577(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1579(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1581(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1583(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1587(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1588(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2181(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2183(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2188(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2191(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1589(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1586(depth + 1, max_depth, buf, rng);
        Self::fragment_1587(depth + 1, max_depth, buf, rng);
        Self::fragment_1588(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1591(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1585(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1589(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1592(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1585(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1589(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1593(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1594(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1585(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1589(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1595(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1592(depth + 1, max_depth, buf, rng);
        Self::fragment_1593(depth + 1, max_depth, buf, rng);
        Self::fragment_1594(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1597(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1474(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1476(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1480(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1599(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1454(depth + 1, max_depth, buf, rng);
        Self::fragment_1455(depth + 1, max_depth, buf, rng);
        Self::fragment_1456(depth + 1, max_depth, buf, rng);
        Self::fragment_1457(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1600(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1474(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1476(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1480(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1601(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1454(depth + 1, max_depth, buf, rng);
        Self::fragment_1455(depth + 1, max_depth, buf, rng);
        Self::fragment_1456(depth + 1, max_depth, buf, rng);
        Self::fragment_1457(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1602(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1600(depth + 1, max_depth, buf, rng);
        Self::fragment_1601(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1603(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1454(depth + 1, max_depth, buf, rng);
        Self::fragment_1455(depth + 1, max_depth, buf, rng);
        Self::fragment_1456(depth + 1, max_depth, buf, rng);
        Self::fragment_1457(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1604(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1474(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1476(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1480(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1605(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1603(depth + 1, max_depth, buf, rng);
        Self::fragment_1604(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1607(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        buf.push(42);
    }
    fn fragment_1609(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1611(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1612(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\"" */
        buf.push(34);
    }
    fn fragment_1613(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1614(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "\"" */
        buf.push(34);
    }
    fn fragment_1615(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1612(depth + 1, max_depth, buf, rng);
        Self::fragment_1613(depth + 1, max_depth, buf, rng);
        Self::fragment_1614(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1617(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1607(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1609(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1611(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1615(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1618(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1607(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1609(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1611(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1615(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1619(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1620(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1607(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1609(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1611(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1615(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1621(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1618(depth + 1, max_depth, buf, rng);
        Self::fragment_1619(depth + 1, max_depth, buf, rng);
        Self::fragment_1620(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1626(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1627(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1628(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1629(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1630(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1632(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1633(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1634(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_703(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_705(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_707(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_709(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_711(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_713(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_715(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_724(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_733(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1635(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1639(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1640(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1641(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1642(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1643(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_1644(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1174(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1180(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1645(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1649(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1650(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1651(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1652(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1653(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1656(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1657(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1658(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1656(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1659(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1659(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1657(depth + 1, max_depth, buf, rng);
        Self::fragment_1658(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1662(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1666(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1669(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1673(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1678(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1684(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1687(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1663(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1666(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1597(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1599(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1602(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1605(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1667(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1597(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1599(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1602(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1605(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1668(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1669(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1667(depth + 1, max_depth, buf, rng);
        Self::fragment_1668(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1670(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1597(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1599(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1602(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1605(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1671(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1672(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1673(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1670(depth + 1, max_depth, buf, rng);
        Self::fragment_1671(depth + 1, max_depth, buf, rng);
        Self::fragment_1672(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1674(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1597(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1599(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1602(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1605(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1675(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1676(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1677(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1678(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1674(depth + 1, max_depth, buf, rng);
        Self::fragment_1675(depth + 1, max_depth, buf, rng);
        Self::fragment_1676(depth + 1, max_depth, buf, rng);
        Self::fragment_1677(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1679(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1597(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1599(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1602(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1605(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1680(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1681(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1682(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1683(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..68) {
            0 => Self::fragment_2333(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2335(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2337(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2339(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2341(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2343(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2345(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_2347(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_2349(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_2351(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_2353(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_2355(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_2357(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_2359(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_2361(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_2363(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_2365(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_2367(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_2369(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_2371(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_2373(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_2375(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_2377(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_2379(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_2381(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_2383(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_2385(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_2387(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_2389(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_2391(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_2393(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_2395(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_2397(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_2399(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_2401(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_2403(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_2405(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_2407(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_2409(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_2411(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_2413(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_2415(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_2417(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_2419(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_2421(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_2423(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_2425(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_2427(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_2429(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_2431(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_2433(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_2435(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_2437(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_2439(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_2441(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_2443(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_2445(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_2447(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_2449(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_2451(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_2453(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_2455(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_2457(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_2459(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_2461(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_2463(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_2465(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_2467(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1684(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1679(depth + 1, max_depth, buf, rng);
        Self::fragment_1680(depth + 1, max_depth, buf, rng);
        Self::fragment_1681(depth + 1, max_depth, buf, rng);
        Self::fragment_1682(depth + 1, max_depth, buf, rng);
        Self::fragment_1683(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1685(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1597(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1599(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1602(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1605(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1686(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1656(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1659(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1687(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1685(depth + 1, max_depth, buf, rng);
        Self::fragment_1686(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1689(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1691(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1693(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1695(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1697(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1699(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1701(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1703(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1705(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1707(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1709(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1711(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1713(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1715(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1717(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1719(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1721(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1723(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1725(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1727(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1729(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1731(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1733(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1735(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1737(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1739(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1741(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1743(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1745(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1747(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1749(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1751(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1753(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1755(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1757(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1759(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1761(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1763(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1765(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1767(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1769(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1771(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1773(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1775(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1777(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1779(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1781(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1783(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1785(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1787(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1789(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1791(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1793(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1795(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1797(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1799(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1801(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1803(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1805(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1807(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1809(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1811(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1813(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1815(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1817(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1819(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1821(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1823(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1825(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1827(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1829(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1831(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1832(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1833(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_520(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_522(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1834(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1835(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1832(depth + 1, max_depth, buf, rng);
        Self::fragment_1833(depth + 1, max_depth, buf, rng);
        Self::fragment_1834(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1837(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1839(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1841(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1843(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1845(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1847(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1848(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1849(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_1850(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_301(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_303(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_305(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_307(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_309(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_311(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_313(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1851(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1848(depth + 1, max_depth, buf, rng);
        Self::fragment_1849(depth + 1, max_depth, buf, rng);
        Self::fragment_1850(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1852(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1853(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_1854(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_315(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_318(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1855(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1852(depth + 1, max_depth, buf, rng);
        Self::fragment_1853(depth + 1, max_depth, buf, rng);
        Self::fragment_1854(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1856(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1857(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1839(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1841(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1843(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1845(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1847(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1851(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1855(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1861(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1862(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1863(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1864(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1865(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1869(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1870(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1871(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1617(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1621(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1872(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1874(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1875(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1876(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_703(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_705(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_707(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_709(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_711(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_713(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_715(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_724(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_733(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1877(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1879(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1880(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1881(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1617(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1621(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1882(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1885(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1607(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1609(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1611(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1615(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1887(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_703(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_705(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_707(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_709(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_711(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_713(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_715(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_724(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_733(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1888(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1889(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1890(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1885(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1887(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1891(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1893(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1894(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1895(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1607(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1609(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1611(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1615(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1896(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1898(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1899(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1900(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..9) {
            0 => Self::fragment_703(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_705(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_707(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_709(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_711(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_713(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_715(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_724(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_733(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1901(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1903(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1904(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1905(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1906(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2469(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2472(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1907(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1910(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1912(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1914(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1915(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_814(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_816(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_818(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_820(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_822(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_824(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_826(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_828(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_830(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_832(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_834(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_836(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_838(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_840(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_842(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_844(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_846(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_848(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_850(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_854(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_856(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_858(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_860(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_862(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_864(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1916(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_814(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_816(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_818(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_820(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_822(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_824(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_826(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_828(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_830(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_832(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_834(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_836(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_838(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_840(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_842(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_844(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_846(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_848(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_850(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_854(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_856(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_858(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_860(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_862(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_864(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1917(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1915(depth + 1, max_depth, buf, rng);
        Self::fragment_1916(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1918(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1910(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1912(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1914(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1917(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1919(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_1920(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2181(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2183(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2188(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2191(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1923(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1918(depth + 1, max_depth, buf, rng);
        Self::fragment_1919(depth + 1, max_depth, buf, rng);
        Self::fragment_1920(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1924(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1918(depth + 1, max_depth, buf, rng);
        Self::fragment_1919(depth + 1, max_depth, buf, rng);
        Self::fragment_1920(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1925(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1926(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1918(depth + 1, max_depth, buf, rng);
        Self::fragment_1919(depth + 1, max_depth, buf, rng);
        Self::fragment_1920(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1927(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1924(depth + 1, max_depth, buf, rng);
        Self::fragment_1925(depth + 1, max_depth, buf, rng);
        Self::fragment_1926(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1928(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1929(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1416(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1419(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1930(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1931(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1928(depth + 1, max_depth, buf, rng);
        Self::fragment_1929(depth + 1, max_depth, buf, rng);
        Self::fragment_1930(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1932(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1933(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1934(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1932(depth + 1, max_depth, buf, rng);
        Self::fragment_1933(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1942(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "<" */
        buf.push(60);
    }
    fn fragment_1943(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1174(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1180(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1944(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ">" */
        buf.push(62);
    }
    fn fragment_1946(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1947(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1948(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1942(depth + 1, max_depth, buf, rng);
        Self::fragment_1943(depth + 1, max_depth, buf, rng);
        Self::fragment_1944(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1949(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1956(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_1958(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_1959(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1960(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_1961(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1962(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1956(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1958(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1963(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1966(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1968(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1970(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1972(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1974(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1976(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1978(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1980(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1982(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..8) {
            0 => Self::fragment_1966(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1968(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1970(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1972(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1974(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1976(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1978(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1980(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1983(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..8) {
            0 => Self::fragment_1966(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1968(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1970(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1972(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1974(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1976(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1978(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1980(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1984(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_1985(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1982(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1986(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_1986(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1983(depth + 1, max_depth, buf, rng);
        Self::fragment_1984(depth + 1, max_depth, buf, rng);
        Self::fragment_1985(depth + 1, max_depth, buf, rng);
    }
    fn fragment_1990(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1992(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1994(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1996(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_1998(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2000(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2002(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2004(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2006(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2008(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2010(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2012(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2014(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2016(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2018(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2020(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2022(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2024(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2202(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2206(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2028(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2030(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2031(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2032(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2033(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2034(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2028(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2030(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2035(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2043(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2044(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2045(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2046(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2047(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2051(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2052(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2053(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2054(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2055(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2061(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2062(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2063(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2064(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2065(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2072(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2074(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2076(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2077(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2078(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2079(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2080(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2072(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2074(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2076(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2081(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2086(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2088(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2089(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2090(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2091(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2092(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2086(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2088(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2093(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2098(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2100(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2101(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2102(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2103(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2104(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2098(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2100(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2105(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2109(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2110(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2111(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2112(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1313(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1315(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2113(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2118(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2120(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2122(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2123(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2124(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2125(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2126(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2118(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2120(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2122(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2127(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2133(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2134(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2135(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2136(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2137(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2141(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2142(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2143(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2144(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2154(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2156(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2158(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2160(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2145(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2148(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2150(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2152(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2154(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2148(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2150(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2152(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2156(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2148(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2150(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2152(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2158(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2160(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2148(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2150(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2152(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2162(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2164(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2165(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2166(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1839(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1841(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1843(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1845(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1847(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1851(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1855(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2167(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2165(depth + 1, max_depth, buf, rng);
        Self::fragment_2166(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2169(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2162(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2164(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2167(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2170(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2162(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2164(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2167(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2171(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2172(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2162(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2164(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2167(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2173(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2170(depth + 1, max_depth, buf, rng);
        Self::fragment_2171(depth + 1, max_depth, buf, rng);
        Self::fragment_2172(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2174(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2175(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_1276(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1278(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1280(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1282(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1284(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1286(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1288(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1290(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1292(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1294(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2176(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_2177(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1531(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1533(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1535(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2178(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2181(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2183(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2184(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2185(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2186(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_2187(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2188(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2184(depth + 1, max_depth, buf, rng);
        Self::fragment_2185(depth + 1, max_depth, buf, rng);
        Self::fragment_2186(depth + 1, max_depth, buf, rng);
        Self::fragment_2187(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2189(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2190(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2191(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2189(depth + 1, max_depth, buf, rng);
        Self::fragment_2190(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2192(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2193(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2194(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2195(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2196(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2197(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2199(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2200(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_2201(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2202(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2199(depth + 1, max_depth, buf, rng);
        Self::fragment_2200(depth + 1, max_depth, buf, rng);
        Self::fragment_2201(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2203(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2204(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_2205(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2206(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2203(depth + 1, max_depth, buf, rng);
        Self::fragment_2204(depth + 1, max_depth, buf, rng);
        Self::fragment_2205(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2208(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2210(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2212(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2214(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2215(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2216(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_2217(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2218(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2215(depth + 1, max_depth, buf, rng);
        Self::fragment_2216(depth + 1, max_depth, buf, rng);
        Self::fragment_2217(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2219(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2220(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2208(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2210(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2221(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_2222(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2212(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2214(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2218(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2223(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2227(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2228(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2229(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2230(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1174(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1180(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2231(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2234(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2236(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1121(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1125(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2239(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2240(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2241(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2242(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1313(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1315(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2243(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2249(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2250(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2251(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2252(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2253(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2259(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2260(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ":" */
        buf.push(58);
    }
    fn fragment_2261(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2262(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2263(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2266(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2268(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2270(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2272(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2266(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2268(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2270(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2273(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2266(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2268(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2270(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2274(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2275(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2266(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2268(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2270(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2276(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2273(depth + 1, max_depth, buf, rng);
        Self::fragment_2274(depth + 1, max_depth, buf, rng);
        Self::fragment_2275(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2277(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2278(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2272(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2276(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2279(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2282(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2284(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_600(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_603(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_606(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2285(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2286(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2287(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2282(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2284(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2288(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2291(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2293(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2295(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2291(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2293(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2296(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2291(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2293(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2297(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2298(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2291(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2293(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2299(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2296(depth + 1, max_depth, buf, rng);
        Self::fragment_2297(depth + 1, max_depth, buf, rng);
        Self::fragment_2298(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2300(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2301(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2302(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2295(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2299(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2303(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2306(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2308(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2309(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2310(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2311(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2306(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2308(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2312(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2324(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2326(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2327(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2328(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2329(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2324(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2326(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2330(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2333(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1224(depth + 1, max_depth, buf, rng);
        Self::fragment_1225(depth + 1, max_depth, buf, rng);
        Self::fragment_1226(depth + 1, max_depth, buf, rng);
        Self::fragment_1227(depth + 1, max_depth, buf, rng);
        Self::fragment_1228(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2335(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1188(depth + 1, max_depth, buf, rng);
        Self::fragment_1189(depth + 1, max_depth, buf, rng);
        Self::fragment_1190(depth + 1, max_depth, buf, rng);
        Self::fragment_1191(depth + 1, max_depth, buf, rng);
        Self::fragment_1192(depth + 1, max_depth, buf, rng);
        Self::fragment_1193(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2337(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1195(depth + 1, max_depth, buf, rng);
        Self::fragment_1196(depth + 1, max_depth, buf, rng);
        Self::fragment_1197(depth + 1, max_depth, buf, rng);
        Self::fragment_1198(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2339(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1200(depth + 1, max_depth, buf, rng);
        Self::fragment_1201(depth + 1, max_depth, buf, rng);
        Self::fragment_1202(depth + 1, max_depth, buf, rng);
        Self::fragment_1203(depth + 1, max_depth, buf, rng);
        Self::fragment_1204(depth + 1, max_depth, buf, rng);
        Self::fragment_1205(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2341(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1207(depth + 1, max_depth, buf, rng);
        Self::fragment_1208(depth + 1, max_depth, buf, rng);
        Self::fragment_1209(depth + 1, max_depth, buf, rng);
        Self::fragment_1210(depth + 1, max_depth, buf, rng);
        Self::fragment_1211(depth + 1, max_depth, buf, rng);
        Self::fragment_1212(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2343(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1243(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1248(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2345(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1255(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1260(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2347(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_1269(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1274(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2349(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1295(depth + 1, max_depth, buf, rng);
        Self::fragment_1296(depth + 1, max_depth, buf, rng);
        Self::fragment_1297(depth + 1, max_depth, buf, rng);
        Self::fragment_1298(depth + 1, max_depth, buf, rng);
        Self::fragment_1299(depth + 1, max_depth, buf, rng);
        Self::fragment_1300(depth + 1, max_depth, buf, rng);
        Self::fragment_1301(depth + 1, max_depth, buf, rng);
        Self::fragment_1302(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2351(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1318(depth + 1, max_depth, buf, rng);
        Self::fragment_1319(depth + 1, max_depth, buf, rng);
        Self::fragment_1320(depth + 1, max_depth, buf, rng);
        Self::fragment_1321(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2353(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1345(depth + 1, max_depth, buf, rng);
        Self::fragment_1346(depth + 1, max_depth, buf, rng);
        Self::fragment_1347(depth + 1, max_depth, buf, rng);
        Self::fragment_1348(depth + 1, max_depth, buf, rng);
        Self::fragment_1349(depth + 1, max_depth, buf, rng);
        Self::fragment_1350(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2355(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1370(depth + 1, max_depth, buf, rng);
        Self::fragment_1371(depth + 1, max_depth, buf, rng);
        Self::fragment_1372(depth + 1, max_depth, buf, rng);
        Self::fragment_1373(depth + 1, max_depth, buf, rng);
        Self::fragment_1374(depth + 1, max_depth, buf, rng);
        Self::fragment_1375(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2357(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1459(depth + 1, max_depth, buf, rng);
        Self::fragment_1460(depth + 1, max_depth, buf, rng);
        Self::fragment_1461(depth + 1, max_depth, buf, rng);
        Self::fragment_1462(depth + 1, max_depth, buf, rng);
        Self::fragment_1463(depth + 1, max_depth, buf, rng);
        Self::fragment_1464(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2359(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1466(depth + 1, max_depth, buf, rng);
        Self::fragment_1467(depth + 1, max_depth, buf, rng);
        Self::fragment_1468(depth + 1, max_depth, buf, rng);
        Self::fragment_1469(depth + 1, max_depth, buf, rng);
        Self::fragment_1470(depth + 1, max_depth, buf, rng);
        Self::fragment_1471(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2361(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1474(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1476(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1480(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2363(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1485(depth + 1, max_depth, buf, rng);
        Self::fragment_1486(depth + 1, max_depth, buf, rng);
        Self::fragment_1487(depth + 1, max_depth, buf, rng);
        Self::fragment_1488(depth + 1, max_depth, buf, rng);
        Self::fragment_1489(depth + 1, max_depth, buf, rng);
        Self::fragment_1490(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2365(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1523(depth + 1, max_depth, buf, rng);
        Self::fragment_1524(depth + 1, max_depth, buf, rng);
        Self::fragment_1525(depth + 1, max_depth, buf, rng);
        Self::fragment_1526(depth + 1, max_depth, buf, rng);
        Self::fragment_1527(depth + 1, max_depth, buf, rng);
        Self::fragment_1528(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2367(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1536(depth + 1, max_depth, buf, rng);
        Self::fragment_1537(depth + 1, max_depth, buf, rng);
        Self::fragment_1538(depth + 1, max_depth, buf, rng);
        Self::fragment_1539(depth + 1, max_depth, buf, rng);
        Self::fragment_1540(depth + 1, max_depth, buf, rng);
        Self::fragment_1541(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2369(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1551(depth + 1, max_depth, buf, rng);
        Self::fragment_1552(depth + 1, max_depth, buf, rng);
        Self::fragment_1553(depth + 1, max_depth, buf, rng);
        Self::fragment_1554(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2371(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1556(depth + 1, max_depth, buf, rng);
        Self::fragment_1557(depth + 1, max_depth, buf, rng);
        Self::fragment_1558(depth + 1, max_depth, buf, rng);
        Self::fragment_1559(depth + 1, max_depth, buf, rng);
        Self::fragment_1560(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2373(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1566(depth + 1, max_depth, buf, rng);
        Self::fragment_1567(depth + 1, max_depth, buf, rng);
        Self::fragment_1568(depth + 1, max_depth, buf, rng);
        Self::fragment_1569(depth + 1, max_depth, buf, rng);
        Self::fragment_1570(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2375(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1626(depth + 1, max_depth, buf, rng);
        Self::fragment_1627(depth + 1, max_depth, buf, rng);
        Self::fragment_1628(depth + 1, max_depth, buf, rng);
        Self::fragment_1629(depth + 1, max_depth, buf, rng);
        Self::fragment_1630(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2377(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1632(depth + 1, max_depth, buf, rng);
        Self::fragment_1633(depth + 1, max_depth, buf, rng);
        Self::fragment_1634(depth + 1, max_depth, buf, rng);
        Self::fragment_1635(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2379(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1639(depth + 1, max_depth, buf, rng);
        Self::fragment_1640(depth + 1, max_depth, buf, rng);
        Self::fragment_1641(depth + 1, max_depth, buf, rng);
        Self::fragment_1642(depth + 1, max_depth, buf, rng);
        Self::fragment_1643(depth + 1, max_depth, buf, rng);
        Self::fragment_1644(depth + 1, max_depth, buf, rng);
        Self::fragment_1645(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2381(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1649(depth + 1, max_depth, buf, rng);
        Self::fragment_1650(depth + 1, max_depth, buf, rng);
        Self::fragment_1651(depth + 1, max_depth, buf, rng);
        Self::fragment_1652(depth + 1, max_depth, buf, rng);
        Self::fragment_1653(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2383(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1861(depth + 1, max_depth, buf, rng);
        Self::fragment_1862(depth + 1, max_depth, buf, rng);
        Self::fragment_1863(depth + 1, max_depth, buf, rng);
        Self::fragment_1864(depth + 1, max_depth, buf, rng);
        Self::fragment_1865(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2385(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1903(depth + 1, max_depth, buf, rng);
        Self::fragment_1904(depth + 1, max_depth, buf, rng);
        Self::fragment_1905(depth + 1, max_depth, buf, rng);
        Self::fragment_1906(depth + 1, max_depth, buf, rng);
        Self::fragment_1907(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2387(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1869(depth + 1, max_depth, buf, rng);
        Self::fragment_1870(depth + 1, max_depth, buf, rng);
        Self::fragment_1871(depth + 1, max_depth, buf, rng);
        Self::fragment_1872(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2389(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1874(depth + 1, max_depth, buf, rng);
        Self::fragment_1875(depth + 1, max_depth, buf, rng);
        Self::fragment_1876(depth + 1, max_depth, buf, rng);
        Self::fragment_1877(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2391(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1879(depth + 1, max_depth, buf, rng);
        Self::fragment_1880(depth + 1, max_depth, buf, rng);
        Self::fragment_1881(depth + 1, max_depth, buf, rng);
        Self::fragment_1882(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2393(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1888(depth + 1, max_depth, buf, rng);
        Self::fragment_1889(depth + 1, max_depth, buf, rng);
        Self::fragment_1890(depth + 1, max_depth, buf, rng);
        Self::fragment_1891(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2395(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1893(depth + 1, max_depth, buf, rng);
        Self::fragment_1894(depth + 1, max_depth, buf, rng);
        Self::fragment_1895(depth + 1, max_depth, buf, rng);
        Self::fragment_1896(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2397(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1898(depth + 1, max_depth, buf, rng);
        Self::fragment_1899(depth + 1, max_depth, buf, rng);
        Self::fragment_1900(depth + 1, max_depth, buf, rng);
        Self::fragment_1901(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2399(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1946(depth + 1, max_depth, buf, rng);
        Self::fragment_1947(depth + 1, max_depth, buf, rng);
        Self::fragment_1948(depth + 1, max_depth, buf, rng);
        Self::fragment_1949(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2401(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1959(depth + 1, max_depth, buf, rng);
        Self::fragment_1960(depth + 1, max_depth, buf, rng);
        Self::fragment_1961(depth + 1, max_depth, buf, rng);
        Self::fragment_1962(depth + 1, max_depth, buf, rng);
        Self::fragment_1963(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2403(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2031(depth + 1, max_depth, buf, rng);
        Self::fragment_2032(depth + 1, max_depth, buf, rng);
        Self::fragment_2033(depth + 1, max_depth, buf, rng);
        Self::fragment_2034(depth + 1, max_depth, buf, rng);
        Self::fragment_2035(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2405(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2043(depth + 1, max_depth, buf, rng);
        Self::fragment_2044(depth + 1, max_depth, buf, rng);
        Self::fragment_2045(depth + 1, max_depth, buf, rng);
        Self::fragment_2046(depth + 1, max_depth, buf, rng);
        Self::fragment_2047(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2407(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2051(depth + 1, max_depth, buf, rng);
        Self::fragment_2052(depth + 1, max_depth, buf, rng);
        Self::fragment_2053(depth + 1, max_depth, buf, rng);
        Self::fragment_2054(depth + 1, max_depth, buf, rng);
        Self::fragment_2055(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2409(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2061(depth + 1, max_depth, buf, rng);
        Self::fragment_2062(depth + 1, max_depth, buf, rng);
        Self::fragment_2063(depth + 1, max_depth, buf, rng);
        Self::fragment_2064(depth + 1, max_depth, buf, rng);
        Self::fragment_2065(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2411(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2077(depth + 1, max_depth, buf, rng);
        Self::fragment_2078(depth + 1, max_depth, buf, rng);
        Self::fragment_2079(depth + 1, max_depth, buf, rng);
        Self::fragment_2080(depth + 1, max_depth, buf, rng);
        Self::fragment_2081(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2413(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2089(depth + 1, max_depth, buf, rng);
        Self::fragment_2090(depth + 1, max_depth, buf, rng);
        Self::fragment_2091(depth + 1, max_depth, buf, rng);
        Self::fragment_2092(depth + 1, max_depth, buf, rng);
        Self::fragment_2093(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2415(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2101(depth + 1, max_depth, buf, rng);
        Self::fragment_2102(depth + 1, max_depth, buf, rng);
        Self::fragment_2103(depth + 1, max_depth, buf, rng);
        Self::fragment_2104(depth + 1, max_depth, buf, rng);
        Self::fragment_2105(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2417(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2109(depth + 1, max_depth, buf, rng);
        Self::fragment_2110(depth + 1, max_depth, buf, rng);
        Self::fragment_2111(depth + 1, max_depth, buf, rng);
        Self::fragment_2112(depth + 1, max_depth, buf, rng);
        Self::fragment_2113(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2419(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2123(depth + 1, max_depth, buf, rng);
        Self::fragment_2124(depth + 1, max_depth, buf, rng);
        Self::fragment_2125(depth + 1, max_depth, buf, rng);
        Self::fragment_2126(depth + 1, max_depth, buf, rng);
        Self::fragment_2127(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2421(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2133(depth + 1, max_depth, buf, rng);
        Self::fragment_2134(depth + 1, max_depth, buf, rng);
        Self::fragment_2135(depth + 1, max_depth, buf, rng);
        Self::fragment_2136(depth + 1, max_depth, buf, rng);
        Self::fragment_2137(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2423(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2141(depth + 1, max_depth, buf, rng);
        Self::fragment_2142(depth + 1, max_depth, buf, rng);
        Self::fragment_2143(depth + 1, max_depth, buf, rng);
        Self::fragment_2144(depth + 1, max_depth, buf, rng);
        Self::fragment_2145(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2425(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2174(depth + 1, max_depth, buf, rng);
        Self::fragment_2175(depth + 1, max_depth, buf, rng);
        Self::fragment_2176(depth + 1, max_depth, buf, rng);
        Self::fragment_2177(depth + 1, max_depth, buf, rng);
        Self::fragment_2178(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2427(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2219(depth + 1, max_depth, buf, rng);
        Self::fragment_2220(depth + 1, max_depth, buf, rng);
        Self::fragment_2221(depth + 1, max_depth, buf, rng);
        Self::fragment_2222(depth + 1, max_depth, buf, rng);
        Self::fragment_2223(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2429(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2227(depth + 1, max_depth, buf, rng);
        Self::fragment_2228(depth + 1, max_depth, buf, rng);
        Self::fragment_2229(depth + 1, max_depth, buf, rng);
        Self::fragment_2230(depth + 1, max_depth, buf, rng);
        Self::fragment_2231(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2431(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2239(depth + 1, max_depth, buf, rng);
        Self::fragment_2240(depth + 1, max_depth, buf, rng);
        Self::fragment_2241(depth + 1, max_depth, buf, rng);
        Self::fragment_2242(depth + 1, max_depth, buf, rng);
        Self::fragment_2243(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2433(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2249(depth + 1, max_depth, buf, rng);
        Self::fragment_2250(depth + 1, max_depth, buf, rng);
        Self::fragment_2251(depth + 1, max_depth, buf, rng);
        Self::fragment_2252(depth + 1, max_depth, buf, rng);
        Self::fragment_2253(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2435(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2259(depth + 1, max_depth, buf, rng);
        Self::fragment_2260(depth + 1, max_depth, buf, rng);
        Self::fragment_2261(depth + 1, max_depth, buf, rng);
        Self::fragment_2262(depth + 1, max_depth, buf, rng);
        Self::fragment_2263(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2437(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2277(depth + 1, max_depth, buf, rng);
        Self::fragment_2278(depth + 1, max_depth, buf, rng);
        Self::fragment_2279(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2439(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2285(depth + 1, max_depth, buf, rng);
        Self::fragment_2286(depth + 1, max_depth, buf, rng);
        Self::fragment_2287(depth + 1, max_depth, buf, rng);
        Self::fragment_2288(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2441(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2300(depth + 1, max_depth, buf, rng);
        Self::fragment_2301(depth + 1, max_depth, buf, rng);
        Self::fragment_2302(depth + 1, max_depth, buf, rng);
        Self::fragment_2303(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2443(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2309(depth + 1, max_depth, buf, rng);
        Self::fragment_2310(depth + 1, max_depth, buf, rng);
        Self::fragment_2311(depth + 1, max_depth, buf, rng);
        Self::fragment_2312(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2445(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2327(depth + 1, max_depth, buf, rng);
        Self::fragment_2328(depth + 1, max_depth, buf, rng);
        Self::fragment_2329(depth + 1, max_depth, buf, rng);
        Self::fragment_2330(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2447(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2497(depth + 1, max_depth, buf, rng);
        Self::fragment_2498(depth + 1, max_depth, buf, rng);
        Self::fragment_2499(depth + 1, max_depth, buf, rng);
        Self::fragment_2500(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2449(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2523(depth + 1, max_depth, buf, rng);
        Self::fragment_2524(depth + 1, max_depth, buf, rng);
        Self::fragment_2525(depth + 1, max_depth, buf, rng);
        Self::fragment_2526(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2451(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2532(depth + 1, max_depth, buf, rng);
        Self::fragment_2533(depth + 1, max_depth, buf, rng);
        Self::fragment_2534(depth + 1, max_depth, buf, rng);
        Self::fragment_2535(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2453(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2541(depth + 1, max_depth, buf, rng);
        Self::fragment_2542(depth + 1, max_depth, buf, rng);
        Self::fragment_2543(depth + 1, max_depth, buf, rng);
        Self::fragment_2544(depth + 1, max_depth, buf, rng);
        Self::fragment_2545(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2455(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2548(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2553(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2457(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_2563(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2565(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2568(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2459(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2601(depth + 1, max_depth, buf, rng);
        Self::fragment_2602(depth + 1, max_depth, buf, rng);
        Self::fragment_2603(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2461(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2588(depth + 1, max_depth, buf, rng);
        Self::fragment_2589(depth + 1, max_depth, buf, rng);
        Self::fragment_2590(depth + 1, max_depth, buf, rng);
        Self::fragment_2591(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2463(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2625(depth + 1, max_depth, buf, rng);
        Self::fragment_2626(depth + 1, max_depth, buf, rng);
        Self::fragment_2627(depth + 1, max_depth, buf, rng);
        Self::fragment_2628(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2465(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2778(depth + 1, max_depth, buf, rng);
        Self::fragment_2779(depth + 1, max_depth, buf, rng);
        Self::fragment_2780(depth + 1, max_depth, buf, rng);
        Self::fragment_2781(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2467(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2192(depth + 1, max_depth, buf, rng);
        Self::fragment_2193(depth + 1, max_depth, buf, rng);
        Self::fragment_2194(depth + 1, max_depth, buf, rng);
        Self::fragment_2195(depth + 1, max_depth, buf, rng);
        Self::fragment_2196(depth + 1, max_depth, buf, rng);
        Self::fragment_2197(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2469(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2470(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2471(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2472(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2470(depth + 1, max_depth, buf, rng);
        Self::fragment_2471(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2474(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2476(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2478(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2480(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2482(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2484(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2486(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2487(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_2474(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2476(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2478(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2480(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2482(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2484(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_2486(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2488(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* ";" */
        buf.push(59);
    }
    fn fragment_2489(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2181(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2183(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2188(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2191(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2492(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2487(depth + 1, max_depth, buf, rng);
        Self::fragment_2488(depth + 1, max_depth, buf, rng);
        Self::fragment_2489(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2493(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2487(depth + 1, max_depth, buf, rng);
        Self::fragment_2488(depth + 1, max_depth, buf, rng);
        Self::fragment_2489(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2494(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2495(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2487(depth + 1, max_depth, buf, rng);
        Self::fragment_2488(depth + 1, max_depth, buf, rng);
        Self::fragment_2489(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2496(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2493(depth + 1, max_depth, buf, rng);
        Self::fragment_2494(depth + 1, max_depth, buf, rng);
        Self::fragment_2495(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2497(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2498(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2499(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2492(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2496(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2500(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2502(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2503(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2504(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2507(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2509(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2510(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2511(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2512(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2510(depth + 1, max_depth, buf, rng);
        Self::fragment_2511(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2513(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2502(depth + 1, max_depth, buf, rng);
        Self::fragment_2503(depth + 1, max_depth, buf, rng);
        Self::fragment_2504(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2514(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_2515(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2516(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2513(depth + 1, max_depth, buf, rng);
        Self::fragment_2514(depth + 1, max_depth, buf, rng);
        Self::fragment_2515(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2518(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2507(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2509(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2512(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2516(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2519(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2507(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2509(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2512(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2516(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2520(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2521(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2507(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2509(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2512(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2516(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2522(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2519(depth + 1, max_depth, buf, rng);
        Self::fragment_2520(depth + 1, max_depth, buf, rng);
        Self::fragment_2521(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2523(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2524(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2525(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2518(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2522(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2526(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2529(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2531(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2532(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2533(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2534(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2529(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2531(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2535(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2538(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..69) {
            0 => Self::fragment_1689(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1691(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1693(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1695(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1697(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1699(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1701(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_1703(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_1705(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_1707(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_1709(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_1711(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_1713(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_1715(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_1717(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_1719(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_1721(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_1723(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_1725(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_1727(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_1729(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_1731(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_1733(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_1735(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_1737(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_1739(depth + 1, max_depth, buf, rng),
            26 => Self::fragment_1741(depth + 1, max_depth, buf, rng),
            27 => Self::fragment_1743(depth + 1, max_depth, buf, rng),
            28 => Self::fragment_1745(depth + 1, max_depth, buf, rng),
            29 => Self::fragment_1747(depth + 1, max_depth, buf, rng),
            30 => Self::fragment_1749(depth + 1, max_depth, buf, rng),
            31 => Self::fragment_1751(depth + 1, max_depth, buf, rng),
            32 => Self::fragment_1753(depth + 1, max_depth, buf, rng),
            33 => Self::fragment_1755(depth + 1, max_depth, buf, rng),
            34 => Self::fragment_1757(depth + 1, max_depth, buf, rng),
            35 => Self::fragment_1759(depth + 1, max_depth, buf, rng),
            36 => Self::fragment_1761(depth + 1, max_depth, buf, rng),
            37 => Self::fragment_1763(depth + 1, max_depth, buf, rng),
            38 => Self::fragment_1765(depth + 1, max_depth, buf, rng),
            39 => Self::fragment_1767(depth + 1, max_depth, buf, rng),
            40 => Self::fragment_1769(depth + 1, max_depth, buf, rng),
            41 => Self::fragment_1771(depth + 1, max_depth, buf, rng),
            42 => Self::fragment_1773(depth + 1, max_depth, buf, rng),
            43 => Self::fragment_1775(depth + 1, max_depth, buf, rng),
            44 => Self::fragment_1777(depth + 1, max_depth, buf, rng),
            45 => Self::fragment_1779(depth + 1, max_depth, buf, rng),
            46 => Self::fragment_1781(depth + 1, max_depth, buf, rng),
            47 => Self::fragment_1783(depth + 1, max_depth, buf, rng),
            48 => Self::fragment_1785(depth + 1, max_depth, buf, rng),
            49 => Self::fragment_1787(depth + 1, max_depth, buf, rng),
            50 => Self::fragment_1789(depth + 1, max_depth, buf, rng),
            51 => Self::fragment_1791(depth + 1, max_depth, buf, rng),
            52 => Self::fragment_1793(depth + 1, max_depth, buf, rng),
            53 => Self::fragment_1795(depth + 1, max_depth, buf, rng),
            54 => Self::fragment_1797(depth + 1, max_depth, buf, rng),
            55 => Self::fragment_1799(depth + 1, max_depth, buf, rng),
            56 => Self::fragment_1801(depth + 1, max_depth, buf, rng),
            57 => Self::fragment_1803(depth + 1, max_depth, buf, rng),
            58 => Self::fragment_1805(depth + 1, max_depth, buf, rng),
            59 => Self::fragment_1807(depth + 1, max_depth, buf, rng),
            60 => Self::fragment_1809(depth + 1, max_depth, buf, rng),
            61 => Self::fragment_1811(depth + 1, max_depth, buf, rng),
            62 => Self::fragment_1813(depth + 1, max_depth, buf, rng),
            63 => Self::fragment_1815(depth + 1, max_depth, buf, rng),
            64 => Self::fragment_1817(depth + 1, max_depth, buf, rng),
            65 => Self::fragment_1819(depth + 1, max_depth, buf, rng),
            66 => Self::fragment_1821(depth + 1, max_depth, buf, rng),
            67 => Self::fragment_1823(depth + 1, max_depth, buf, rng),
            68 => Self::fragment_1825(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2540(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_698(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_701(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2541(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2542(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2543(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2538(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2540(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2544(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2545(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2548(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2549(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2550(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2551(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2555(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2557(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2559(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2561(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2552(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2553(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2549(depth + 1, max_depth, buf, rng);
        Self::fragment_2550(depth + 1, max_depth, buf, rng);
        Self::fragment_2551(depth + 1, max_depth, buf, rng);
        Self::fragment_2552(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2555(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1573(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1575(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1577(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1579(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1581(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1583(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2557(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1573(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1575(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1577(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1579(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1581(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1583(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2559(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2561(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_1573(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1575(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1577(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1579(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1581(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1583(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2563(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2565(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2566(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2567(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..18) {
            0 => Self::fragment_524(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_526(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_528(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_530(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_532(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_534(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_536(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_538(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_540(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_542(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_544(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_546(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_548(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_550(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_552(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_554(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_556(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_558(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2568(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2566(depth + 1, max_depth, buf, rng);
        Self::fragment_2567(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2570(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2572(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2573(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2574(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1839(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1841(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1843(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1845(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1847(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1851(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1855(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2575(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2573(depth + 1, max_depth, buf, rng);
        Self::fragment_2574(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2577(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2579(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2581(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2583(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_2570(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2572(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2575(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2577(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2579(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2581(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2584(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_2570(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2572(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2575(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2577(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2579(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2581(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2585(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2586(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_2570(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2572(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2575(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2577(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2579(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2581(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2587(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2584(depth + 1, max_depth, buf, rng);
        Self::fragment_2585(depth + 1, max_depth, buf, rng);
        Self::fragment_2586(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2588(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2589(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2590(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2583(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2587(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2591(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2594(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2596(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2598(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2600(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2601(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2602(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2594(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2596(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2598(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2600(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2603(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2614(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2616(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2618(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2620(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2622(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2624(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_923(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_933(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_946(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2625(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2626(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2627(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_2614(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2616(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2618(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2620(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_2622(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_2624(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2628(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2759(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1839(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1841(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1843(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1845(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1847(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1851(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1855(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2760(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2761(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2759(depth + 1, max_depth, buf, rng);
        Self::fragment_2760(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2762(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1839(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1841(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1843(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1845(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1847(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1851(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1855(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2763(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2764(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2762(depth + 1, max_depth, buf, rng);
        Self::fragment_2763(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2765(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1839(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1841(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1843(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1845(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1847(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1851(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1855(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2766(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2767(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2765(depth + 1, max_depth, buf, rng);
        Self::fragment_2766(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2768(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_1839(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1841(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1843(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1845(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1847(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_1851(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_1855(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2769(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_2770(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_1111(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1113(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1115(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1117(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_1119(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2771(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2768(depth + 1, max_depth, buf, rng);
        Self::fragment_2769(depth + 1, max_depth, buf, rng);
        Self::fragment_2770(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2773(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2761(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2764(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2767(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2771(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2774(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_2761(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2764(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_2767(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_2771(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2775(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "," */
        buf.push(44);
    }
    fn fragment_2776(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2773(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2777(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2777(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_2774(depth + 1, max_depth, buf, rng);
        Self::fragment_2775(depth + 1, max_depth, buf, rng);
        Self::fragment_2776(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2778(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2779(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2780(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_2773(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_2777(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2781(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2789(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_2790(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2791(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_1169(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1174(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1180(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2792(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* " " */
        buf.push(32);
    }
    fn fragment_2793(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2794(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1856(depth + 1, max_depth, buf, rng);
        Self::fragment_1857(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2795(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..7) {
            0 => Self::fragment_899(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_901(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_903(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_905(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_907(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_909(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_2796(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_2798(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_101(depth + 1, max_depth, buf, rng);
        Self::fragment_2789(depth + 1, max_depth, buf, rng);
        Self::fragment_2790(depth + 1, max_depth, buf, rng);
        Self::fragment_2791(depth + 1, max_depth, buf, rng);
        Self::fragment_2792(depth + 1, max_depth, buf, rng);
        Self::fragment_2793(depth + 1, max_depth, buf, rng);
        Self::fragment_2794(depth + 1, max_depth, buf, rng);
        Self::fragment_2795(depth + 1, max_depth, buf, rng);
        Self::fragment_2796(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2799(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_1662(depth + 1, max_depth, buf, rng);
        Self::fragment_1663(depth + 1, max_depth, buf, rng);
    }
    fn fragment_2800(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_1305(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_1307(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_1309(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_1311(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
}
