pub fn bitwise_hamming_dis(s1: &[u8], s2: &[u8]) -> usize {
    assert!(s1.len() == s2.len());

    let mut res = 0;
    let len = s1.len();

    for i in 0..len {
        for j in 0..8 {
            res += (((s1[i] >> j) & 1) ^ ((s2[i] >> j) & 1)) as usize;
        }
    }
    res
}
