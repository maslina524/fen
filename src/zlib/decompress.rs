use alloc::vec::Vec;

use crate::zlib::stream::Stream;

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