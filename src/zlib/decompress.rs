use alloc::vec::Vec;
use alloc::vec;

use crate::zlib::huffman::{HuffmanTree, decode_symb};
use crate::zlib::stream::Stream;

const LENGTH_BASE: [u32; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LENGTH_EXTRA_BITS: [u32; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DISTANCE_BASE: [u32; 30] = [1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577];
const DISTANCE_EXTRA_BITS: [u32; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

pub fn inflate_block_no_compression(mut stream: Stream, inflated: &mut Vec<u8>) {
    let len = stream.read_bytes(2);
	// let nlen = stream.read_bytes(2);

	for _ in 0..len {
        inflated.push(stream.read_byte())
    }
}

pub fn inflate_compressed_block(mut stream: Stream, literal_len_tree: HuffmanTree, distance_tree: HuffmanTree, inflated: &mut Vec<u8>) {
    loop {
        let symb = decode_symb(&mut stream, &literal_len_tree).unwrap() as u32;

        if symb < 256 {
            inflated.push(symb as u8);
        } else if symb == 256 {
            return;
        } else {
            let i = (symb - 257) as usize;
            let len = DISTANCE_BASE[i] + stream.read_bits(LENGTH_EXTRA_BITS[i] as usize) as u32;
            let symb = decode_symb(&mut stream, &distance_tree).unwrap() as usize;
            let distance = DISTANCE_BASE[symb] + stream.read_bits(DISTANCE_EXTRA_BITS[symb] as usize) as u32;
            
            for _ in 0..len {
                inflated.push(inflated[inflated.len() - distance as usize]);
            }
        }
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

    let literal_len_tree = HuffmanTree::from_alphabet_and_bl_list(Vec::from_iter(0..256), bl_list);

    bl_list = vec![5; 30];
    let distance_tree = HuffmanTree::from_alphabet_and_bl_list(Vec::from_iter(0..30), bl_list);

    inflate_compressed_block(stream, literal_len_tree, distance_tree, inflated)
}

fn inflate(mut stream: Stream) -> Vec<u8> {
    let mut bfinal = 0;
    let mut inflated = Vec::new();

    while bfinal == 0 {
        bfinal = stream.read_bit();
        let btype = stream.read_bits(2);

        match btype {
            0b00 => inflate_block_no_compression(stream, &mut inflated),
            0b01 => inflate_block_fixed_huffman(stream.clone(), &mut inflated),
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