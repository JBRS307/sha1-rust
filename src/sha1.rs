use std::iter::zip;

mod method1;
mod method2;

/// SHA1 computation method as specified in RFC 3174
#[allow(dead_code)]
enum Sha1Method {
    First,
    Second,
}

pub struct Sha1Digest {
    bytes: [u8; 20],
    hex_digest: String,
}

impl Sha1Digest {
    pub fn hash(msg_bytes: Vec<u8>) -> Self {
        let padded_bytes = pad_message(msg_bytes);
        let blocks = split_msg_to_blocks(padded_bytes);

        let bytes = method1::process_msg(blocks);
        let hex_digest = hex_digest(&bytes);

        Self { bytes, hex_digest }
    }

    pub fn bytes(&self) -> &[u8; 20] {
        &self.bytes
    }

    pub fn hex_digest(&self) -> &str {
        &self.hex_digest
    }
}

fn pad_message(mut bytes: Vec<u8>) -> Vec<u8> {
    let original_length = bytes.len() as u64;
    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0);
    }
    let length_bytes = (8 * original_length).to_be_bytes();
    bytes.extend(length_bytes);
    assert_eq!(bytes.len() % 64, 0);
    bytes
}

fn split_msg_to_blocks(bytes: Vec<u8>) -> Vec<[u32; 16]> {
    assert_eq!(bytes.len() % 64, 0);

    let mut blocks = vec![];
    for i in (0..bytes.len()).step_by(64) {
        let mut block = [0u32; 16];
        for (k, j) in zip(0usize..16, (i..i + 64).step_by(4)) {
            let word = u32::from_be_bytes([bytes[j], bytes[j + 1], bytes[j + 2], bytes[j + 3]]);
            block[k] = word;
        }
        blocks.push(block);
    }
    blocks
}

fn f(t: usize, b: u32, c: u32, d: u32) -> u32 {
    match t {
        0..=19 => (b & c) | (!b & d),
        20..=39 => b ^ c ^ d,
        40..=59 => (b & c) | (b & d) | (c & d),
        60..=79 => b ^ c ^ d,
        _ => unreachable!(),
    }
}

fn k(t: usize) -> u32 {
    match t {
        0..=19 => 0x5a827999,
        20..=39 => 0x6ed9eba1,
        40..=59 => 0x8f1bbcdc,
        60..=79 => 0xca62c1d6,
        _ => unreachable!(),
    }
}

fn h_arr() -> [u32; 5] {
    const H0: u32 = 0x67452301;
    const H1: u32 = 0xefcdab89;
    const H2: u32 = 0x98badcfe;
    const H3: u32 = 0x10325476;
    const H4: u32 = 0xc3d2e1f0;
    [H0, H1, H2, H3, H4]
}

fn hex_digest(bytes: &[u8; 20]) -> String {
    let mut digest = String::with_capacity(40);
    for byte in bytes {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}
