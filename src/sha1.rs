pub struct Sha1Digest {
    bytes: [u8; 20],
    hex_digest: String,
}

impl Sha1Digest {
    pub fn hash(msg_bytes: Vec<u8>) -> Self {
        let padded_bytes = pad_message(msg_bytes);
        println!("{}", padded_bytes.len());
        println!("{padded_bytes:?}");

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
