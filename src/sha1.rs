pub struct Sha1Digest {
    bytes: [u8; 20],
    hex_digest: String,
}

impl Sha1Digest {
    pub fn hash(msg_bytes: Vec<u8>) -> Self {
        let padded_bytes = pad_message(msg_bytes);

        todo!()
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

