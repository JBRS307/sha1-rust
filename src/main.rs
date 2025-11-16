use crate::sha1::Sha1Digest;

mod sha1;

fn main() {
    let msg = "abcdef";
    let msg_bytes = msg.as_bytes().to_vec();
    let hash = Sha1Digest::hash(msg_bytes);
    println!("{:?}", hash.bytes());
    println!("{}", hash.hex_digest());

    let msg = "a".repeat(70);
    println!("{msg}");
    let msg_bytes = msg.as_bytes().to_vec();
    let hash = Sha1Digest::hash(msg_bytes);
    println!("{:?}", hash.bytes());
    println!("{}", hash.hex_digest());
}
