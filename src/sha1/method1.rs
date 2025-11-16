use crate::sha1::{f, h_arr, k, rotate_left};

pub(super) fn process_msg(blocks: Vec<[u32; 16]>) -> [u8; 20] {
    let [mut h0, mut h1, mut h2, mut h3, mut h4] = h_arr();
    for block in blocks {
        process_block(&block, &mut h0, &mut h1, &mut h2, &mut h3, &mut h4);
    }

    let mut output = [0u8; 20];
    let h0_bytes = h0.to_be_bytes();
    let h1_bytes = h1.to_be_bytes();
    let h2_bytes = h2.to_be_bytes();
    let h3_bytes = h3.to_be_bytes();
    let h4_bytes = h4.to_be_bytes();

    output[0..4].copy_from_slice(&h0_bytes);
    output[4..8].copy_from_slice(&h1_bytes);
    output[8..12].copy_from_slice(&h2_bytes);
    output[12..16].copy_from_slice(&h3_bytes);
    output[16..20].copy_from_slice(&h4_bytes);
    output
}

fn process_block(
    block: &[u32; 16],
    h0: &mut u32,
    h1: &mut u32,
    h2: &mut u32,
    h3: &mut u32,
    h4: &mut u32,
) {
    let mut w = [0u32; 80];
    w[0..16].copy_from_slice(block);

    for t in 16..=79usize {
        let n = w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16];
        w[t] = rotate_left(n, 1);
    }

    let mut a = *h0;
    let mut b = *h1;
    let mut c = *h2;
    let mut d = *h3;
    let mut e = *h4;

    for (t, word) in w.iter().enumerate() {
        let temp = u32::overflowing_add(rotate_left(a, 5), f(t, b, c, d))
            .0
            .overflowing_add(e)
            .0
            .overflowing_add(*word)
            .0
            .overflowing_add(k(t))
            .0;
        e = d;
        d = c;
        c = rotate_left(b, 30);
        b = a;
        a = temp;
    }

    *h0 = u32::overflowing_add(*h0, a).0;
    *h1 = u32::overflowing_add(*h1, b).0;
    *h2 = u32::overflowing_add(*h2, c).0;
    *h3 = u32::overflowing_add(*h3, d).0;
    *h4 = u32::overflowing_add(*h4, e).0;
}
