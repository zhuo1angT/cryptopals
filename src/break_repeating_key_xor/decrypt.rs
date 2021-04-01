pub fn bit_level_hamming_dis(s1: &str, s2: &str) -> usize {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    assert!(s1.len() == s2.len());

    let mut res = 0;
    let len = s1.len();

    for i in 0..len {
        for j in 0..8 {
            if (((s1[i] >> j) & 1) != 0) ^ (((s2[i] >> j) & 1) != 0) {
                res += 1;
            }
        }
    }
    res
}
