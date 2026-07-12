use alloc::vec::Vec;

use crate::zlib::stream::Stream;

fn inflate(mut stream: Stream) -> Vec<u8> {
    let mut bfinal = 0;
    let mut inflated = Vec::new();

    while bfinal == 0 {
        bfinal = stream.read_bit();
        let btype = stream.read_bits(2);

        match btype {
            0b00 => inflate_block_no_compression(stream, &mut inflated),
            0b01 => inflate_block_fixed_huffman(stream, &mut inflated),
            0b10 => inflate_block_dynamic_huffman(stream, &mut inflated),
        }
    }

    inflated
}

pub fn decompress(bytes: Vec<u8>, buf: &mut Vec<u8>) -> u32 {
    let mut stream = Stream::new(bytes);

    let cmf = stream.read_byte();
    let cm = cmf & 0b1111;
    let cinfo = cmf >> 4;
    let flg = stream.read_byte();
    let fdict = (flg >> 5) & 1;

    let checksum = stream.read_bytes(4) as u32;
    checksum
} 