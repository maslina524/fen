use alloc::boxed::Box;

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