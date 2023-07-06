#![allow(unused)]
use rand::Rng;
use std::cell::Cell;

pub struct GrammarGenerator;

pub static TERMINALS: [&'static str; 86] = [
    "http://example.com",
    "http://example.com/example/sub",
    "http://example.com/example/sub?a=b",
    "http://example.com/example/sub?a=b&c=12345",
    "http://example.com/example/sub?a=b&c=12345#asdf-asdf",
    "http://example.com/example/sub#asdf-asdf",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "+",
    "/",
    "=",
    "==",
    "-",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "%",
    ".",
    "example.com",
    "sub.example.com",
    "subsub.sub.example.com",
    "a.b.c.d.f.g.h.i.j.k.example.com",
    "&",
    "?",
    "http://",
    "https://",
    "ftp://",
    "file://",
    "://",
];

impl GrammarGenerator {
    pub fn terminals() -> &'static [&'static str] {
        return &TERMINALS;
    }

    pub fn generate_into(out: &mut Vec<u8>, max_depth: Option<usize>, rng: &mut impl Rng) {
        out.clear();
        Self::fragment_1(0, max_depth.unwrap_or(1024 as usize), out, rng);
    }

    pub fn generate_new(max_depth: Option<usize>, rng: &mut impl Rng) -> Vec<u8> {
        let mut out = Vec::new();
        Self::generate_into(&mut out, max_depth, rng);
        out
    }
    fn fragment_1(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_967(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_972(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_978(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_15(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_17(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_19(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_21(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_23(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_25(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_63(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
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
            _ => unreachable!(),
        }
    }
    fn fragment_65(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_777(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_779(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_781(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_783(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_785(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_787(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_789(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_791(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_793(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_795(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_797(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_799(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_801(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_803(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_805(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_807(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_809(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_811(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_813(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_90(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_92(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_94(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2" */
        buf.push(50);
    }
    fn fragment_96(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3" */
        buf.push(51);
    }
    fn fragment_98(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_100(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5" */
        buf.push(53);
    }
    fn fragment_102(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "6" */
        buf.push(54);
    }
    fn fragment_104(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "7" */
        buf.push(55);
    }
    fn fragment_106(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_108(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9" */
        buf.push(57);
    }
    fn fragment_257(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "0" */
        buf.push(48);
    }
    fn fragment_259(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "1" */
        buf.push(49);
    }
    fn fragment_261(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "2" */
        buf.push(50);
    }
    fn fragment_263(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "3" */
        buf.push(51);
    }
    fn fragment_265(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "4" */
        buf.push(52);
    }
    fn fragment_267(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "5" */
        buf.push(53);
    }
    fn fragment_269(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "6" */
        buf.push(54);
    }
    fn fragment_271(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "7" */
        buf.push(55);
    }
    fn fragment_273(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "8" */
        buf.push(56);
    }
    fn fragment_275(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "9" */
        buf.push(57);
    }
    fn fragment_277(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "A" */
        buf.push(65);
    }
    fn fragment_279(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "B" */
        buf.push(66);
    }
    fn fragment_281(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "C" */
        buf.push(67);
    }
    fn fragment_283(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "D" */
        buf.push(68);
    }
    fn fragment_285(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "E" */
        buf.push(69);
    }
    fn fragment_287(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "F" */
        buf.push(70);
    }
    fn fragment_289(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "a" */
        buf.push(97);
    }
    fn fragment_291(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "b" */
        buf.push(98);
    }
    fn fragment_293(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "c" */
        buf.push(99);
    }
    fn fragment_295(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "d" */
        buf.push(100);
    }
    fn fragment_297(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e" */
        buf.push(101);
    }
    fn fragment_299(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "f" */
        buf.push(102);
    }
    fn fragment_385(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_90(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_92(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_94(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_96(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_98(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_100(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_102(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_104(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_106(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_108(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_387(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "+" */
        buf.push(43);
    }
    fn fragment_389(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_391(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_393(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_395(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_63(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_65(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_385(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_387(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_389(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_396(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_63(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_65(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_385(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_387(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_389(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_397(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_391(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_393(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_398(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_396(depth + 1, max_depth, buf, rng);
        Self::fragment_397(depth + 1, max_depth, buf, rng);
    }
    fn fragment_399(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_63(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_65(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_385(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_387(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_389(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_400(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_395(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_398(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_401(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_401(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_399(depth + 1, max_depth, buf, rng);
        Self::fragment_400(depth + 1, max_depth, buf, rng);
    }
    fn fragment_470(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
    }
    fn fragment_472(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_473(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_474(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_470(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_472(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_475(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_482(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_475(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_473(depth + 1, max_depth, buf, rng);
        Self::fragment_474(depth + 1, max_depth, buf, rng);
    }
    fn fragment_476(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_477(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_478(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_479(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_480(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_484(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_486(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_488(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_490(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_481(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_470(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_472(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_475(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_482(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_482(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_476(depth + 1, max_depth, buf, rng);
        Self::fragment_477(depth + 1, max_depth, buf, rng);
        Self::fragment_478(depth + 1, max_depth, buf, rng);
        Self::fragment_479(depth + 1, max_depth, buf, rng);
        Self::fragment_480(depth + 1, max_depth, buf, rng);
        Self::fragment_481(depth + 1, max_depth, buf, rng);
    }
    fn fragment_484(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
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
            _ => unreachable!(),
        }
    }
    fn fragment_486(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_777(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_779(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_781(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_783(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_785(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_787(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_789(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_791(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_793(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_795(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_797(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_799(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_801(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_803(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_805(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_807(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_809(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_811(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_813(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_488(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_90(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_92(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_94(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_96(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_98(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_100(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_102(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_104(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_106(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_108(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_490(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_491(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
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
            _ => unreachable!(),
        }
    }
    fn fragment_492(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_470(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_472(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_475(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_482(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_493(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_491(depth + 1, max_depth, buf, rng);
        Self::fragment_492(depth + 1, max_depth, buf, rng);
    }
    fn fragment_494(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_777(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_779(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_781(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_783(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_785(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_787(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_789(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_791(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_793(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_795(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_797(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_799(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_801(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_803(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_805(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_807(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_809(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_811(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_813(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_495(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_470(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_472(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_475(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_482(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_496(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_494(depth + 1, max_depth, buf, rng);
        Self::fragment_495(depth + 1, max_depth, buf, rng);
    }
    fn fragment_609(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "a" */
        buf.push(97);
    }
    fn fragment_611(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "b" */
        buf.push(98);
    }
    fn fragment_613(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "c" */
        buf.push(99);
    }
    fn fragment_615(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "d" */
        buf.push(100);
    }
    fn fragment_617(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "e" */
        buf.push(101);
    }
    fn fragment_619(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "f" */
        buf.push(102);
    }
    fn fragment_621(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "g" */
        buf.push(103);
    }
    fn fragment_623(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "h" */
        buf.push(104);
    }
    fn fragment_625(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "i" */
        buf.push(105);
    }
    fn fragment_627(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "j" */
        buf.push(106);
    }
    fn fragment_629(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "k" */
        buf.push(107);
    }
    fn fragment_631(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "l" */
        buf.push(108);
    }
    fn fragment_633(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "m" */
        buf.push(109);
    }
    fn fragment_635(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "n" */
        buf.push(110);
    }
    fn fragment_637(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "o" */
        buf.push(111);
    }
    fn fragment_639(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "p" */
        buf.push(112);
    }
    fn fragment_641(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "q" */
        buf.push(113);
    }
    fn fragment_643(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "r" */
        buf.push(114);
    }
    fn fragment_645(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "s" */
        buf.push(115);
    }
    fn fragment_647(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "t" */
        buf.push(116);
    }
    fn fragment_649(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "u" */
        buf.push(117);
    }
    fn fragment_651(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "v" */
        buf.push(118);
    }
    fn fragment_653(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "w" */
        buf.push(119);
    }
    fn fragment_655(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "x" */
        buf.push(120);
    }
    fn fragment_657(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "y" */
        buf.push(121);
    }
    fn fragment_659(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "z" */
        buf.push(122);
    }
    fn fragment_777(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "A" */
        buf.push(65);
    }
    fn fragment_779(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "B" */
        buf.push(66);
    }
    fn fragment_781(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "C" */
        buf.push(67);
    }
    fn fragment_783(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "D" */
        buf.push(68);
    }
    fn fragment_785(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "E" */
        buf.push(69);
    }
    fn fragment_787(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "F" */
        buf.push(70);
    }
    fn fragment_789(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "G" */
        buf.push(71);
    }
    fn fragment_791(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "H" */
        buf.push(72);
    }
    fn fragment_793(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "I" */
        buf.push(73);
    }
    fn fragment_795(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "J" */
        buf.push(74);
    }
    fn fragment_797(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "K" */
        buf.push(75);
    }
    fn fragment_799(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "L" */
        buf.push(76);
    }
    fn fragment_801(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "M" */
        buf.push(77);
    }
    fn fragment_803(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "N" */
        buf.push(78);
    }
    fn fragment_805(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "O" */
        buf.push(79);
    }
    fn fragment_807(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "P" */
        buf.push(80);
    }
    fn fragment_809(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Q" */
        buf.push(81);
    }
    fn fragment_811(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "R" */
        buf.push(82);
    }
    fn fragment_813(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "S" */
        buf.push(83);
    }
    fn fragment_815(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "T" */
        buf.push(84);
    }
    fn fragment_817(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "U" */
        buf.push(85);
    }
    fn fragment_819(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "V" */
        buf.push(86);
    }
    fn fragment_821(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "W" */
        buf.push(87);
    }
    fn fragment_823(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "X" */
        buf.push(88);
    }
    fn fragment_825(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Y" */
        buf.push(89);
    }
    fn fragment_827(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "Z" */
        buf.push(90);
    }
    fn fragment_829(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
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
            _ => unreachable!(),
        }
    }
    fn fragment_831(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..26) {
            0 => Self::fragment_777(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_779(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_781(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_783(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_785(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_787(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_789(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_791(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_793(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_795(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_797(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_799(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_801(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_803(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_805(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_807(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_809(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_811(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_813(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_815(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_817(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_819(depth + 1, max_depth, buf, rng),
            22 => Self::fragment_821(depth + 1, max_depth, buf, rng),
            23 => Self::fragment_823(depth + 1, max_depth, buf, rng),
            24 => Self::fragment_825(depth + 1, max_depth, buf, rng),
            25 => Self::fragment_827(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_833(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..10) {
            0 => Self::fragment_90(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_92(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_94(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_96(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_98(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_100(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_102(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_104(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_106(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_108(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_835(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "-" */
        buf.push(45);
    }
    fn fragment_837(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_838(depth + 1, max_depth, buf, rng);
        Self::fragment_839(depth + 1, max_depth, buf, rng);
        Self::fragment_840(depth + 1, max_depth, buf, rng);
    }
    fn fragment_838(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "%" */
        buf.push(37);
    }
    fn fragment_839(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_257(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_259(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_261(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_263(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_265(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_267(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_269(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_271(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_273(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_275(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_277(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_279(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_281(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_283(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_285(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_287(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_289(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_291(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_293(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_840(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..22) {
            0 => Self::fragment_257(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_259(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_261(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_263(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_265(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_267(depth + 1, max_depth, buf, rng),
            6 => Self::fragment_269(depth + 1, max_depth, buf, rng),
            7 => Self::fragment_271(depth + 1, max_depth, buf, rng),
            8 => Self::fragment_273(depth + 1, max_depth, buf, rng),
            9 => Self::fragment_275(depth + 1, max_depth, buf, rng),
            10 => Self::fragment_277(depth + 1, max_depth, buf, rng),
            11 => Self::fragment_279(depth + 1, max_depth, buf, rng),
            12 => Self::fragment_281(depth + 1, max_depth, buf, rng),
            13 => Self::fragment_283(depth + 1, max_depth, buf, rng),
            14 => Self::fragment_285(depth + 1, max_depth, buf, rng),
            15 => Self::fragment_287(depth + 1, max_depth, buf, rng),
            16 => Self::fragment_289(depth + 1, max_depth, buf, rng),
            17 => Self::fragment_291(depth + 1, max_depth, buf, rng),
            18 => Self::fragment_293(depth + 1, max_depth, buf, rng),
            19 => Self::fragment_295(depth + 1, max_depth, buf, rng),
            20 => Self::fragment_297(depth + 1, max_depth, buf, rng),
            21 => Self::fragment_299(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_843(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_844(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_845(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_846(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_847(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_848(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_849(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_850(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_851(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_852(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_844(depth + 1, max_depth, buf, rng);
        Self::fragment_845(depth + 1, max_depth, buf, rng);
        Self::fragment_846(depth + 1, max_depth, buf, rng);
        Self::fragment_847(depth + 1, max_depth, buf, rng);
        Self::fragment_848(depth + 1, max_depth, buf, rng);
        Self::fragment_849(depth + 1, max_depth, buf, rng);
        Self::fragment_850(depth + 1, max_depth, buf, rng);
        Self::fragment_851(depth + 1, max_depth, buf, rng);
    }
    fn fragment_853(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_854(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_855(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_853(depth + 1, max_depth, buf, rng);
        Self::fragment_854(depth + 1, max_depth, buf, rng);
    }
    fn fragment_856(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_857(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_858(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_859(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_860(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_861(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_862(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_863(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_829(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_831(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_833(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_835(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_837(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_864(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_865(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_856(depth + 1, max_depth, buf, rng);
        Self::fragment_857(depth + 1, max_depth, buf, rng);
        Self::fragment_858(depth + 1, max_depth, buf, rng);
        Self::fragment_859(depth + 1, max_depth, buf, rng);
        Self::fragment_860(depth + 1, max_depth, buf, rng);
        Self::fragment_861(depth + 1, max_depth, buf, rng);
        Self::fragment_862(depth + 1, max_depth, buf, rng);
        Self::fragment_863(depth + 1, max_depth, buf, rng);
        Self::fragment_864(depth + 1, max_depth, buf, rng);
    }
    fn fragment_900(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_901(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_902(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "." */
        buf.push(46);
    }
    fn fragment_903(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_904(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_901(depth + 1, max_depth, buf, rng);
        Self::fragment_902(depth + 1, max_depth, buf, rng);
        Self::fragment_903(depth + 1, max_depth, buf, rng);
    }
    fn fragment_906(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_908(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_910(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_912(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_914(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_900(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_904(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_916(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_917(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_918(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_919(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_916(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_920(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_920(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_917(depth + 1, max_depth, buf, rng);
        Self::fragment_918(depth + 1, max_depth, buf, rng);
        Self::fragment_919(depth + 1, max_depth, buf, rng);
    }
    fn fragment_924(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_925(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_926(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_927(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_928(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_928(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_925(depth + 1, max_depth, buf, rng);
        Self::fragment_926(depth + 1, max_depth, buf, rng);
        Self::fragment_927(depth + 1, max_depth, buf, rng);
    }
    fn fragment_929(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_930(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "=" */
        buf.push(61);
    }
    fn fragment_931(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_934(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_936(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_934(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..3) {
            0 => Self::fragment_395(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_398(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_401(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_936(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_843(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_852(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_855(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_865(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_937(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_938(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_939(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_937(depth + 1, max_depth, buf, rng);
        Self::fragment_938(depth + 1, max_depth, buf, rng);
    }
    fn fragment_940(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_941(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_942(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_943(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_944(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_940(depth + 1, max_depth, buf, rng);
        Self::fragment_941(depth + 1, max_depth, buf, rng);
        Self::fragment_942(depth + 1, max_depth, buf, rng);
        Self::fragment_943(depth + 1, max_depth, buf, rng);
    }
    fn fragment_945(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_946(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_947(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_948(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_949(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "&" */
        buf.push(38);
    }
    fn fragment_950(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_929(depth + 1, max_depth, buf, rng);
        Self::fragment_930(depth + 1, max_depth, buf, rng);
        Self::fragment_931(depth + 1, max_depth, buf, rng);
    }
    fn fragment_951(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_945(depth + 1, max_depth, buf, rng);
        Self::fragment_946(depth + 1, max_depth, buf, rng);
        Self::fragment_947(depth + 1, max_depth, buf, rng);
        Self::fragment_948(depth + 1, max_depth, buf, rng);
        Self::fragment_949(depth + 1, max_depth, buf, rng);
        Self::fragment_950(depth + 1, max_depth, buf, rng);
    }
    fn fragment_952(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "?" */
        buf.push(63);
    }
    fn fragment_953(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_924(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_928(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_954(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_952(depth + 1, max_depth, buf, rng);
        Self::fragment_953(depth + 1, max_depth, buf, rng);
    }
    fn fragment_956(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_958(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_960(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_962(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
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
    fn fragment_963(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_493(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_496(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_964(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "://" */
        unsafe {
            let old_size = buf.len();
            let new_size = old_size + 3;

            if new_size > buf.capacity() {
                buf.reserve(new_size - old_size);
            }

            std::ptr::copy_nonoverlapping(
                [58, 47, 47].as_ptr(),
                buf.as_mut_ptr().offset(old_size as isize),
                3,
            );
            buf.set_len(new_size);
        }
    }
    fn fragment_965(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_963(depth + 1, max_depth, buf, rng);
        Self::fragment_964(depth + 1, max_depth, buf, rng);
    }
    fn fragment_967(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..6) {
            0 => Self::fragment_15(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_17(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_19(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_21(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_23(depth + 1, max_depth, buf, rng),
            5 => Self::fragment_25(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_968(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_956(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_958(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_960(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_962(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_965(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_969(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_970(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_971(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_916(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_920(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_972(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_968(depth + 1, max_depth, buf, rng);
        Self::fragment_969(depth + 1, max_depth, buf, rng);
        Self::fragment_970(depth + 1, max_depth, buf, rng);
        Self::fragment_971(depth + 1, max_depth, buf, rng);
    }
    fn fragment_973(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_956(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_958(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_960(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_962(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_965(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_974(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..5) {
            0 => Self::fragment_906(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_908(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_910(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_912(depth + 1, max_depth, buf, rng),
            4 => Self::fragment_914(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_975(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        /* "/" */
        buf.push(47);
    }
    fn fragment_976(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..2) {
            0 => Self::fragment_916(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_920(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_977(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        match rng.gen_range(0..4) {
            0 => Self::fragment_939(depth + 1, max_depth, buf, rng),
            1 => Self::fragment_944(depth + 1, max_depth, buf, rng),
            2 => Self::fragment_951(depth + 1, max_depth, buf, rng),
            3 => Self::fragment_954(depth + 1, max_depth, buf, rng),
            _ => unreachable!(),
        }
    }
    fn fragment_978(depth: usize, max_depth: usize, buf: &mut Vec<u8>, rng: &mut impl Rng) {
        if depth >= max_depth {
            return;
        }
        Self::fragment_973(depth + 1, max_depth, buf, rng);
        Self::fragment_974(depth + 1, max_depth, buf, rng);
        Self::fragment_975(depth + 1, max_depth, buf, rng);
        Self::fragment_976(depth + 1, max_depth, buf, rng);
        Self::fragment_977(depth + 1, max_depth, buf, rng);
    }
}
