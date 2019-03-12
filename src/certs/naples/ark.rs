#![allow(clippy::unreadable_literal)]

use std::num::NonZeroU128;
use endicon::Endianness;
use codicon::Decoder;
use super::super::*;
use super::*;

#[test]
fn decode() {
    let ark = Certificate::decode(&mut &ARK[..], Kind::Ca).unwrap();
    assert_eq!(ark, Certificate {
        version: 1,
        firmware: None,
        sigs: [None, None],
        key: PublicKey {
            usage: Usage::AmdRootKey,
            algo: SigAlgo::RsaSha256.into(),
            key: Key::Rsa(RsaKey {
                pubexp: to4096(&ARK[0x040..][..256]),
                modulus: to4096(&ARK[0x140..][..256]),
            }),
            id: NonZeroU128::new(122178821951678173525318614033703090459),
        },
    });
    assert_eq!(ark.sigs, [
        Some(Signature {
            usage: Usage::AmdRootKey,
            algo: SigAlgo::RsaSha256,
            sig: to4096(&ARK[0x240..][..256]),
            id: NonZeroU128::new(122178821951678173525318614033703090459),
        }),
        None
    ]);
}

#[test]
fn encode() {
    let ark = Certificate::decode(&mut &ARK[..], Kind::Ca).unwrap();

    let output = ark.encode_buf(()).unwrap();
    assert_eq!(ARK.len(), output.len());
    assert_eq!(ARK.to_vec(), output);

    let output = ark.body().unwrap();
    assert_eq!(CA_SIG_OFFSET, output.len());
    assert_eq!(ARK[..CA_SIG_OFFSET].to_vec(), output);
}

#[test]
fn verify() {
    let ark = Certificate::decode(&mut ARK, Kind::Ca).unwrap();
    (&ark, &ark).verify().unwrap();
}

#[test]
fn create() {
    let (ark, _) = Certificate::new(Usage::AmdRootKey).unwrap();
    let buf = ark.encode_buf(()).unwrap();

    let id = u128::decode(&mut &buf[4..], Endianness::Little).unwrap();
    let id = NonZeroU128::new(id);

    assert_eq!(ark, Certificate {
        version: 1,
        firmware: None,
        sigs: [None, None],
        key: PublicKey {
            usage: Usage::AmdRootKey,
            algo: SigAlgo::RsaSha256.into(),
            key: Key::Rsa(RsaKey {
                pubexp: to4096(&buf[0x040..0x140]),
                modulus: to4096(&buf[0x140..0x240]),
            }),
            id: id,
        },
    });

    assert_eq!(ark.sigs, [
        Some(Signature {
            usage: Usage::AmdRootKey,
            algo: SigAlgo::RsaSha256,
            sig: to4096(&buf[0x240..0x340]),
            id: id,
        }),
        None
    ]);

    assert!((&ark, &ark).verify().is_ok());
}
