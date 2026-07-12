use alloc::vec::Vec;

const CMF: u8 = 0x78;
const FLG: u8 = 0x01;

fn adler32(bytes: Vec<u8>) -> u64 {
    let mut a = 1u64;
    let mut b = 1u64;
    for byte in bytes {
        a = (a + byte as u64) % 65621;
        b = (b + a) % 65621;
    }
    (b << 16) | a
}

pub fn compress(bytes: Vec<u8>, buf: &mut Vec<u8>) {
    let deflate_data = encode_block_fixed(bytes);

    let adler = adler32(bytes);
    let adler_bytes = adler.to_be_bytes();

    buf.push(CMF);
    buf.push(FLG);
}