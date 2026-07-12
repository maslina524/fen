use crate::zlib::stream::WriteStream;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::vec;

const LENGTH_BASE: [u16; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LENGTH_EXTRA_BITS: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DISTANCE_BASE: [u16; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DISTANCE_EXTRA_BITS: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

#[derive(Clone, Copy)]
enum Token {
    Literal(u8),
    Pair(u16, u16),
}

fn compute_codes(bl_list: &[u8]) -> BTreeMap<u16, (u32, u8)> {
    let max_bits = bl_list.iter().max().copied().unwrap_or(0) as usize;
    let mut bl_count = vec![0; max_bits + 1];
    for &bl in bl_list {
        if bl > 0 {
            bl_count[bl as usize] += 1;
        }
    }

    let mut next_code = vec![0u32; max_bits + 1];
    next_code[1] = 0;
    for bits in 2..=max_bits {
        next_code[bits] = (next_code[bits - 1] + bl_count[bits - 1]) << 1;
    }

    let mut codes = BTreeMap::new();
    for (symbol, &bl) in bl_list.iter().enumerate() {
        if bl > 0 {
            let code = next_code[bl as usize];
            codes.insert(symbol as u16, (code, bl));
            next_code[bl as usize] += 1;
        }
    }
    codes
}

fn get_fixed_literal_length_codes() -> BTreeMap<u16, (u32, u8)> {
    let mut bl = Vec::with_capacity(288);
    for _ in 0..144 {
        bl.push(8);
    }
    for _ in 144..256 {
        bl.push(9);
    }
    for _ in 256..280 {
        bl.push(7);
    }
    for _ in 280..288 {
        bl.push(8);
    }
    bl.truncate(286);
    compute_codes(&bl)
}

fn get_fixed_distance_codes() -> BTreeMap<u16, (u32, u8)> {
    let bl = vec![5; 30];
    compute_codes(&bl)
}

fn find_match(data: &[u8], pos: usize, window_size: usize) -> (u16, u16) {
    let max_len = core::cmp::min(258, data.len() - pos);
    if max_len < 3 {
        return (0, 0);
    }
    let start = if pos > window_size { pos - window_size } else { 0 };
    let mut best_len = 0;
    let mut best_dist = 0;
    for j in start..pos {
        let mut l = 0;
        while l < max_len && data[pos + l] == data[j + l] {
            l += 1;
        }
        if l >= 3 && l > best_len {
            best_len = l;
            best_dist = pos - j;
            if best_len == max_len {
                break;
            }
        }
    }
    (best_len as u16, best_dist as u16)
}

fn encode_block_fixed(data: &[u8]) -> Vec<u8> {
    let lit_codes = get_fixed_literal_length_codes();
    let dist_codes = get_fixed_distance_codes();

    let mut out = WriteStream::new();

    out.write_bits(1, 1, false);
    out.write_bits(0b01, 2, false);

    let mut tokens = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let (length, distance) = find_match(data, i, 32768);
        if length >= 3 {
            tokens.push(Token::Pair(length, distance));
            i += length as usize;
        } else {
            tokens.push(Token::Literal(data[i]));
            i += 1;
        }
    }

    for token in tokens {
        match token {
            Token::Literal(lit) => {
                let (code, bits) = lit_codes[&(lit as u16)];
                out.write_bits(code, bits as usize, true);
            }
            Token::Pair(length, distance) => {
                let mut idx_len = 0;
                while idx_len < LENGTH_BASE.len() - 1 && length >= LENGTH_BASE[idx_len + 1] {
                    idx_len += 1;
                }
                let symbol = 257 + idx_len;
                let (code, bits) = lit_codes[&(symbol as u16)];
                out.write_bits(code, bits as usize, true);
                let extra = length - LENGTH_BASE[idx_len];
                if LENGTH_EXTRA_BITS[idx_len] > 0 {
                    out.write_bits(extra as u32, LENGTH_EXTRA_BITS[idx_len] as usize, false);
                }

                let mut idx_dist = 0;
                while idx_dist < DISTANCE_BASE.len() - 1 && distance >= DISTANCE_BASE[idx_dist + 1] {
                    idx_dist += 1;
                }
                let (code, bits) = dist_codes[&(idx_dist as u16)];
                out.write_bits(code, bits as usize, true);
                let extra = distance - DISTANCE_BASE[idx_dist];
                if DISTANCE_EXTRA_BITS[idx_dist] > 0 {
                    out.write_bits(extra as u32, DISTANCE_EXTRA_BITS[idx_dist] as usize, false);
                }
            }
        }
    }

    let (code_end, bits_end) = lit_codes[&256];
    out.write_bits(code_end, bits_end as usize, true);

    out.flush();
    out.get_bytes().to_vec()
}

fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let mut a = 1u32;
    let mut b = 0u32;
    for &byte in data {
        a = (a + byte as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}

pub fn compress(data: &[u8], buf: &mut Vec<u8>) {
    let cmf = 0x78;
    let flg = 0x01;

    let deflate_data = encode_block_fixed(data);

    let adler = adler32(data);
    let adler_bytes = adler.to_be_bytes();

    buf.push(cmf);
    buf.push(flg);
    buf.extend_from_slice(&deflate_data);
    buf.extend_from_slice(&adler_bytes);
}