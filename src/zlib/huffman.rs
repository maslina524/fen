use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::vec;

use crate::zlib::stream::Stream;

#[derive(Debug, Clone)]
pub struct HuffmanNode {
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
    symb: Option<char>,
}

impl HuffmanNode {
    pub fn new() -> Self {
        Self { left: None, right: None, symb: None }
    }
}

#[derive(Debug, Clone)]
pub struct HuffmanTree {
    root: HuffmanNode,
}

impl HuffmanTree {
    pub fn new() -> Self {
        Self { root: HuffmanNode::new() }
    }

    pub fn from_alphabet_and_bl_list(alphabet: Vec<u8>, bl_list: Vec<u8>) -> Self {
        let max_bits = bl_list.iter().max().unwrap();

        let mut bl_count = vec![0; bl_list.len()];
        for i in 0..max_bits + 1 {
            for bl in &bl_list {
                if *bl == i && i != 0 {
                    bl_count[i as usize] += 1;
                }
            }
        }

        let mut next_code = vec![0, 0];
        for bit_len in 2..max_bits + 1 {
            let bit_len = bit_len as usize;
            next_code.push(
                (next_code[bit_len - 1] + bl_count[bit_len - 1]) << 1
            );
        }

        let mut tree = Self::new();
        for (symb, bit_len) in alphabet.iter().zip(bl_list.iter()) {
            let bit_len = *bit_len as usize;
            if bit_len != 0 {
                tree.insert(next_code[bit_len], bit_len, char::from_u32(*symb as u32).unwrap());
                next_code[bit_len] += 1;
            }
        }
        tree
    }

    pub fn insert(&mut self, code: u64, n: usize, symb: char) {
        let mut curr = &mut self.root;

        for i in 0..n {
            let bit = (code >> (n - i - 1) & 1) as u8;
            let next = if bit == 1 {
                &mut curr.right
            } else {
                &mut curr.left
            };

            if next.is_none() {
                *next = Some(Box::new(HuffmanNode::new()));
            }

            curr = next.as_mut().unwrap();
        }

        curr.symb = Some(symb);
    }
}

pub fn decode_symb(mut stream: Stream, tree: HuffmanTree) -> Option<char> {
    let mut curr = &tree.root;

    while curr.left.is_some() || curr.right.is_some() {
        let bit = stream.read_bit();
        if bit == 1 {
            curr = curr.right.as_ref().unwrap();
        } else {
            curr = curr.left.as_ref().unwrap();
        }
    }

    curr.symb
}