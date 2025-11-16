use crate::sha1::Sha1Digest;

mod sha1;

fn main() {
    let msg = "a".repeat(60);
    let msg_bytes = msg.as_bytes().to_vec();
    Sha1Digest::hash(msg_bytes);
}
