use alloc::vec::Vec;
use alloc::vec;

use crate::zlib::stream::Stream;

pub fn inflate_block_no_compression(mut stream: Stream, inflated: &mut Vec<u8>) {
    let len = stream.read_bytes(2);
	// let nlen = stream.read_bytes(2);

	for _ in 0..len {
        inflated.push(stream.read_byte())
    }
}

pub fn inflate_block_fixed_huffman(mut stream: Stream, inflated: &mut Vec<u8>) {
    let mut bl_list = Vec::new();
    
    for _ in 0..144 {
        bl_list.push(8);
    }
    for _ in 144..256 {
        bl_list.push(9);
    }
    for _ in 256..280 {
        bl_list.push(7);
    }
    for _ in 280..288 {
        bl_list.push(8);
    }

    let literal_len_tree = huffman_tree_from_alphabet_and_bl_list(0..256, bl_list);

    bl_list = vec![5; 30];
    let distance_tree = huffman_tree_from_alphabet_and_bl_list(0..30, bl_list);

    inflate_compressed_block(stream, literal_len_tree, distance_tree, &mut inflated)
}

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