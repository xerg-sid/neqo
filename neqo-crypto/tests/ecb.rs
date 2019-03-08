#![deny(warnings)]

use neqo_crypto::constants::*;
use neqo_crypto::ecb::ecb;
use neqo_crypto::{init_db, SymKey, SymKeyTarget};

#[test]
fn aes_ecb() {
    init_db("./db");
    let key = SymKey::import(
        SymKeyTarget::Ecb(TLS_AES_128_GCM_SHA256),
        &[
            0x94, 0xb9, 0x45, 0x2d, 0x2b, 0x3c, 0x7c, 0x7f, 0x6d, 0xa7, 0xfd, 0xd8, 0x59, 0x35,
            0x37, 0xfd,
        ],
    )
    .expect("can make key");
    let input = &[
        0xc4, 0xc2, 0xa2, 0x30, 0x3d, 0x29, 0x7e, 0x3c, 0x51, 0x9b, 0xf6, 0xb2, 0x23, 0x86, 0xe3,
        0xd0,
    ];
    let mask = ecb(&key, input).expect("should produce a mask");
    const EXPECTED: &[u8] = &[
        0x75, 0xf7, 0xec, 0x8b, 0x62, 0x20, 0xac, 0x7f, 0x30, 0x57, 0xec, 0x8e, 0x38, 0x25, 0xe2,
        0x8f,
    ];
    assert_eq!(mask, EXPECTED);
}
