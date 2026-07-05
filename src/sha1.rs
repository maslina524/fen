use alloc::format;
use alloc::vec::Vec;

/// reference: https://github.com/pcaro90/Python-SHA1/blob/master/SHA1.py
pub struct Sha1 {
    bytes: [u8; 20]
}

impl Sha1 {
    pub fn new() -> Self {
        Self { bytes: [0u8; 20] }
    }

    pub fn encrypt(&mut self, content: &[u8]) {
        let mut ret = [0u8; 20];

        let stream = self.padding(Vec::new());
        let stream = self.prepare(&stream);

        self.bytes = ret
    }

    fn padding(&self, mut stream: Vec<u8>) -> Vec<u8> {
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
}