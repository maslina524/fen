use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Stream {
    memory: Vec<u8>,
    byte_idx: usize,
    bit_idx: usize,
}

impl Stream {
    pub fn new(memory: Vec<u8>) -> Self {
        Self { memory, byte_idx: 0, bit_idx: 0 }
    }

    pub fn align_to_byte(&mut self) {
        if self.bit_idx != 0 {
            self.bit_idx = 0;
            self.byte_idx += 1;
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.bit_idx != 0 {
            self.bit_idx = 0;
            self.byte_idx += 1;
        }
        let byte = self.memory[self.byte_idx];
        self.byte_idx += 1;
        byte
    }

    pub fn read_bytes(&mut self, count: usize) -> u64 {
        let mut bytes = 0u64;
        for i in 0..count {
            bytes |= (self.read_byte() as u64) << (8 * i);
        }
        bytes
    }

    pub fn read_bit(&mut self) -> u8 {
        let bit = (self.memory[self.byte_idx] >> self.bit_idx) & 1;
        self.bit_idx += 1;
        if self.bit_idx > 7 {
            self.bit_idx = 0;
            self.byte_idx += 1;
        }
        bit
    }

    pub fn read_bits(&mut self, count: usize) -> u64 {
        let mut bits = 0;
        for i in 0..count {
            bits |= (self.read_bit() as u64) << i;
        }
        bits
    }
}

pub struct WriteStream {
    memory: Vec<u8>,
    byte: u8,
    bit_idx: usize
}

impl WriteStream {
    pub fn new() -> Self {
        Self { memory: Vec::new(), byte: 0, bit_idx: 0 }
    }

    pub fn write_bit(&mut self, bit: u8) {
        if bit == 1 {
            self.byte |= 1 << self.bit_idx
        }

        self.bit_idx += 1;
        if self.bit_idx == 8 {
            self.memory.push(self.byte);
            self.byte = 0;
            self.bit_idx = 0;
        } 
    }

    pub fn write_bits(&mut self, value: u32, count: usize, msb_first: bool) {
        if msb_first {
            for i in count - 1..-1 {
                self.write_bit(((value >> i) & 1) as u8);
            }
        } else {
            for i in 0..count {
                self.write_bit(((value >> i) & 1) as u8);
            }
        }
    }

    pub fn flush(&mut self) {
        if self.bit_idx > 0 {
            self.memory.push(self.byte);
            self.byte = 0;
            self.bit_idx = 0;
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.memory
    }
}