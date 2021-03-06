//! Ed25519 signatures.
//!
//! Edwards Digital Signature Algorithm (EdDSA) over Curve25519 as specified in
//! RFC 8032: <https://tools.ietf.org/html/rfc8032>
//!
//! This crate doesn't contain an implementation of Ed25519, but instead
//! contains an [`ed25519::Signature`][`Signature`] type which other crates can
//! use in conjunction with the [`signature::Signer`] and
//! [`signature::Verifier`] traits defined in the [`signature`] crate.
//!
//! These traits allow crates which produce and consume Ed25519 signatures
//! to be written abstractly in such a way that different signing_key/verifier
//! providers can be plugged in, enabling support for using different
//! Ed25519 implementations, including HSMs or Cloud KMS services.
//!
//! ## Minimum Supported Rust Version
//!
//! Rust **1.41** or higher.
//!
//! Minimum supported Rust version may be changed in the future, but such
//! changes will be accompanied with a minor version bump.
//!
//! # Using Ed25519 generically over algorithm implementations/providers
//!
//! By using the `ed25519` crate, you can write code which signs and verifies
//! messages using the Ed25519 signature algorithm generically over any
//! supported Ed25519 implementation (see the next section for available
//! providers).
//!
//! This allows consumers of your code to plug in whatever implementation they
//! want to use without having to add all potential Ed25519 libraries you'd
//! like to support as optional dependencies.
//!
//! ## Example
//!
//! ```
//! use ed25519::signature::{Signer, Verifier};
//!
//! pub struct HelloSigner<S>
//! where
//!     S: Signer<ed25519::Signature>
//! {
//!     pub signing_key: S
//! }
//!
//! impl<S> HelloSigner<S>
//! where
//!     S: Signer<ed25519::Signature>
//! {
//!     pub fn sign(&self, person: &str) -> ed25519::Signature {
//!         // NOTE: use `try_sign` if you'd like to be able to handle
//!         // errors from external signing services/devices (e.g. HSM/KMS)
//!         // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
//!         self.signing_key.sign(format_message(person).as_bytes())
//!     }
//! }
//!
//! pub struct HelloVerifier<V> {
//!     pub verifier: V
//! }
//!
//! impl<V> HelloVerifier<V>
//! where
//!     V: Verifier<ed25519::Signature>
//! {
//!     pub fn verify(
//!         &self,
//!         person: &str,
//!         signature: &ed25519::Signature
//!     ) -> Result<(), ed25519::Error> {
//!         self.verifier.verify(format_message(person).as_bytes(), signature)
//!     }
//! }
//!
//! fn format_message(person: &str) -> String {
//!     format!("Hello, {}!", person)
//! }
//! ```
//!
//! ## Using above example with `ed25519-dalek`
//!
//! The [`ed25519-dalek`] crate natively supports the [`ed25519::Signature`][`Signature`]
//! type defined in this crate along with the the [`signature::Signer`] and
//! [`signature::Verifier`] traits.
//!
//! Below is an example of how a hypothetical consumer of the code above can
//! instantiate and use the previously defined `HelloSigner` and `HelloVerifier`
//! types with [`ed25519-dalek`] as the signing/verification provider:
//!
//! ```
//! use ed25519_dalek::{Signer, Verifier, Signature};
//! #
//! # pub struct HelloSigner<S>
//! # where
//! #     S: Signer<Signature>
//! # {
//! #     pub signing_key: S
//! # }
//! #
//! # impl<S> HelloSigner<S>
//! # where
//! #     S: Signer<Signature>
//! # {
//! #     pub fn sign(&self, person: &str) -> Signature {
//! #         // NOTE: use `try_sign` if you'd like to be able to handle
//! #         // errors from external signing services/devices (e.g. HSM/KMS)
//! #         // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
//! #         self.signing_key.sign(format_message(person).as_bytes())
//! #     }
//! # }
//! #
//! # pub struct HelloVerifier<V> {
//! #     pub verify_key: V
//! # }
//! #
//! # impl<V> HelloVerifier<V>
//! # where
//! #     V: Verifier<Signature>
//! # {
//! #     pub fn verify(
//! #         &self,
//! #         person: &str,
//! #         signature: &Signature
//! #     ) -> Result<(), ed25519::Error> {
//! #         self.verify_key.verify(format_message(person).as_bytes(), signature)
//! #     }
//! # }
//! #
//! # fn format_message(person: &str) -> String {
//! #     format!("Hello, {}!", person)
//! # }
//! use rand_core::OsRng; // Requires the `std` feature of `rand_core`
//!
//! /// `HelloSigner` defined above instantiated with `ed25519-dalek` as
//! /// the signing provider.
//! pub type DalekHelloSigner = HelloSigner<ed25519_dalek::Keypair>;
//!
//! let signing_key = ed25519_dalek::Keypair::generate(&mut OsRng);
//! let signer = DalekHelloSigner { signing_key };
//! let person = "Joe"; // Message to sign
//! let signature = signer.sign(person);
//!
//! /// `HelloVerifier` defined above instantiated with `ed25519-dalek`
//! /// as the signature verification provider.
//! pub type DalekHelloVerifier = HelloVerifier<ed25519_dalek::PublicKey>;
//!
//! let verify_key: ed25519_dalek::PublicKey = signer.signing_key.public;
//! let verifier = DalekHelloVerifier { verify_key };
//! assert!(verifier.verify(person, &signature).is_ok());
//! ```
//!
//! # Available Ed25519 providers
//!
//! The following libraries natively support the types and traits from the
//! `ed25519` crate:
//!
//! - [`ed25519-dalek`] - mature pure Rust implementation of Ed25519
//! - [`yubihsm`] - host-side client library for YubiHSM2 devices from Yubico
//!
//! The [Signatory] project provides wrappers for several notable crates which
//! produce/verify Ed25519 signatures:
//!
//! - [`signatory-ring`] - wrapper for [*ring*]
//! - [`signatory-sodiumoxide`] - wrapper for [`sodiumoxide`]
//!
//! [`ed25519-dalek`]: https://docs.rs/ed25519-dalek
//! [*ring*]: https://github.com/briansmith/ring
//! [Signatory]: https://github.com/iqlusioninc/signatory/blob/develop/README.md
//! [`signatory-ring`]: https://docs.rs/signatory-ring/
//! [`signatory-sodiumoxide`]: https://docs.rs/signatory-sodiumoxide/
//! [`sodiumoxide`]: https://github.com/sodiumoxide/sodiumoxide
//! [`yubihsm`]: https://github.com/iqlusioninc/yubihsm.rs/blob/develop/README.md

#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png",
    html_root_url = "https://docs.rs/ed25519/1.0.2"
)]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    intra_doc_link_resolution_failure
)]

#[cfg(feature = "serde")]
use serde::{de, ser, Deserialize, Serialize};

#[cfg(all(test, feature = "std"))]
extern crate std;

pub use signature::{self, Error};

use core::{
    convert::{TryFrom, TryInto},
    fmt::{self, Debug},
};

/// Length of an Ed25519 signature
pub const SIGNATURE_LENGTH: usize = 64;

/// Ed25519 signature.
#[derive(Copy, Clone)]
pub struct Signature([u8; SIGNATURE_LENGTH]);

impl Signature {
    /// Create a new signature from a byte array
    pub fn new(bytes: [u8; SIGNATURE_LENGTH]) -> Self {
        Self::from(bytes)
    }

    /// Return the inner byte array
    pub fn to_bytes(&self) -> [u8; SIGNATURE_LENGTH] {
        self.0
    }
}

impl signature::Signature for Signature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        bytes.try_into()
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

// can't derive `Debug`, `PartialEq`, or `Eq` below because core array types
// only have  trait implementations for lengths 0..=32
impl Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ed25519::Signature({:?})", &self.0[..])
    }
}

// TODO(tarcieri): derive `Eq` after const generics are available
impl Eq for Signature {}

// TODO(tarcieri): derive `PartialEq` after const generics are available
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl From<[u8; SIGNATURE_LENGTH]> for Signature {
    fn from(bytes: [u8; SIGNATURE_LENGTH]) -> Signature {
        Signature(bytes)
    }
}

impl<'a> TryFrom<&'a [u8]> for Signature {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Error> {
        // TODO(tarcieri): use TryInto when const generics are available
        if bytes.len() != SIGNATURE_LENGTH {
            return Err(Error::new());
        }

        // Perform a partial reduction check on the signature's `s` scalar.
        // When properly reduced, at least the three highest bits of the scalar
        // will be unset so as to fit within the order of ~2^(252.5).
        //
        // This doesn't ensure that `s` is fully reduced (which would require a
        // full reduction check in the event that the 4th most significant bit
        // is set), however it will catch a number of invalid signatures
        // relatively inexpensively.
        if bytes[SIGNATURE_LENGTH - 1] & 0b1110_0000 != 0 {
            return Err(Error::new());
        }

        let mut arr = [0u8; SIGNATURE_LENGTH];
        arr.copy_from_slice(bytes);
        Ok(Signature(arr))
    }
}

#[cfg(feature = "serde")]
impl Serialize for Signature {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use ser::SerializeTuple;

        let mut seq = serializer.serialize_tuple(SIGNATURE_LENGTH)?;

        for byte in &self.0[..] {
            seq.serialize_element(byte)?;
        }

        seq.end()
    }
}

// serde lacks support for deserializing arrays larger than 32-bytes
// see: <https://github.com/serde-rs/serde/issues/631>
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ByteArrayVisitor;

        impl<'de> de::Visitor<'de> for ByteArrayVisitor {
            type Value = [u8; SIGNATURE_LENGTH];

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("bytestring of length 64")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<[u8; SIGNATURE_LENGTH], A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                use de::Error;
                let mut arr = [0u8; SIGNATURE_LENGTH];

                for (i, byte) in arr.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| Error::invalid_length(i, &self))?;
                }

                Ok(arr)
            }
        }

        deserializer
            .deserialize_tuple(SIGNATURE_LENGTH, ByteArrayVisitor)
            .map(|bytes| bytes.into())
    }
}

#[cfg(all(test, feature = "serde", feature = "std"))]
mod tests {
    use super::*;
    use signature::Signature as _;
    use std::{convert::TryFrom, vec::Vec};

    const EXAMPLE_SIGNATURE: [u8; SIGNATURE_LENGTH] = [
        63, 62, 61, 60, 59, 58, 57, 56, 55, 54, 53, 52, 51, 50, 49, 48, 47, 46, 45, 44, 43, 42, 41,
        40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18,
        17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
    ];

    #[test]
    fn test_serialize() {
        let signature = Signature::try_from(&EXAMPLE_SIGNATURE[..]).unwrap();
        let encoded_signature: Vec<u8> = bincode::serialize(&signature).unwrap();
        assert_eq!(&EXAMPLE_SIGNATURE[..], &encoded_signature[..]);
    }

    #[test]
    fn test_deserialize() {
        let signature = bincode::deserialize::<Signature>(&EXAMPLE_SIGNATURE).unwrap();
        assert_eq!(&EXAMPLE_SIGNATURE[..], signature.as_bytes());
    }
}
