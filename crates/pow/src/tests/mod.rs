use crate::pow::verify_pow;

#[test]
fn test_verify_proof_of_work_0() {
    let digest: [u8; 32] = [
        0x63, 0x08, 0xb3, 0x8a, 0xe2, 0x84, 0x1c, 0x18, 0xfb, 0x8c, 0x06, 0xc9, 0xac, 0xc9, 0xbc,
        0xd5, 0x5d, 0x35, 0xfa, 0xb3, 0xc1, 0x11, 0x98, 0xda, 0x5f, 0x6f, 0xe4, 0x16, 0x66, 0x99,
        0x3b, 0x16,
    ];
    let nonce: u64 = 0xd65397f;
    let n_bits: u8 = 0x1e;
    verify_pow(digest, n_bits, nonce).unwrap();
}

#[test]
#[should_panic]
fn test_verify_proof_of_work_1() {
    let digest: [u8; 32] = [
        0x63, 0x08, 0xb3, 0x8a, 0xe2, 0x84, 0x1c, 0x18, 0xfb, 0x8c, 0x06, 0xc9, 0xac, 0xc9, 0xbc,
        0xd5, 0x5d, 0x35, 0xfa, 0xb3, 0xc1, 0x11, 0x98, 0xda, 0x5f, 0x6f, 0xe4, 0x16, 0x66, 0x99,
        0x3b, 0x16,
    ];
    let nonce: u64 = 0xd65397f + 1;
    let n_bits: u8 = 0x1e;
    verify_pow(digest, n_bits, nonce).unwrap();
}
