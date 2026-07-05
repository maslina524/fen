use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// reference: https://github.com/pcaro90/Python-SHA1/blob/master/SHA1.py
pub struct Sha1 {
    bytes: [u32; 5]
}

impl Sha1 {
    pub fn new() -> Self {
        Self {
            bytes: [
                0x67452301,
                0xEFCDAB89,
                0x98BADCFE,
                0x10325476,
                0xC3D2E1F0,
            ]
        }
    }

    pub fn encrypt(&mut self, content: &[u8]) {
        let stream = self.padding(content);
        let stream = self.prepare(&stream);

        for block in stream {
            self.process_block(block);
        }
    }

    fn padding(&self, stream: &[u8]) -> Vec<u8> {
        let mut stream = Vec::from(stream);

        let l = stream.len();
        let hex_str = format!("{:0>16x}", l * 8);
        let mut hl = [0u8; 8];
        for (j, i) in (0..16).step_by(2).enumerate() {
            let hex_byte = &hex_str[i..i+2];
            let int = u8::from_str_radix(hex_byte, 16).unwrap();
            hl[j] = int;
        }

        let l0 = (56 - l) % 64;
        let l0 = if l0 == 0 { 64 } else { l0 };

        stream.push(0x80);
        stream.extend(core::iter::repeat(0).take(l0 - 1));
        stream.extend(hl);

        stream
    }

    fn prepare(&self, stream: &[u8]) -> Vec<[u32; 16]> {
        let mut blocks = Vec::new();
        let n_blocks = stream.len() / 64;
        for i in 0..n_blocks {
            let mut words = [0u32; 16];

            for j in 0..16 {
                let mut word = 0u32;
                for k in 0..4 {
                    word = (word << 8) | stream[i*64 + j*4 + k] as u32;
                }

                words[j] = word;
            }

            blocks.push(words);
        }

        blocks
    }

    fn rotl(&self, n: u32, x: u32, w: u32) -> u32 {
        (x << n) | (x >> (w - n)) 
    }

    fn process_block(&mut self, block: [u32; 16]) {
        const MASK: u32 = u32::MAX;

        let mut w = Vec::from(block);
        for t in 16..80 {
            let value = self.rotl(
                1, 
                w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16], 
                32
            ) & MASK;
            w.push(value);
        }

        let [mut a, mut b, mut c, mut d, mut e] = self.bytes;

        for t in 0..80 {
            let (k, f) = if t <= 19 {
                (0x5a827999, (b & c) ^ (!b & d))
            } else if t <= 39 {
                (0x6ed9eba1, b ^ c ^ d)
            } else if t <= 59 {
                (0x8f1bbcdc, (b & c) ^ (b & d) ^ (c & d))
            } else {
                (0xca62c1d6, b ^ c ^ d)
            };

            let big_t = (self.rotl(5,a, 32) + f + e + k + w[t]) & MASK;
            e = d;
            d = c;
            c = self.rotl(30, b, 32) & MASK;
            b = a;
            a = big_t;
        }

        self.bytes[0] = (a + self.bytes[0]) & MASK;
        self.bytes[1] = (b + self.bytes[1]) & MASK;
        self.bytes[2] = (c + self.bytes[2]) & MASK;
        self.bytes[3] = (d + self.bytes[3]) & MASK;
        self.bytes[4] = (e + self.bytes[4]) & MASK;
    }

    pub fn hex(&self) -> String {
        let [a, b, c, d, e] = self.bytes;
        format!("{a:08x}{b:08x}{c:08x}{d:08x}{e:08x}")
    }
}