//! Traits defining the interface that all crypto backends must implement.

use std::borrow::Cow;
use crate::{Result};
use crate::crypto::mpi::{MPI, ProtectedMPI};
use crate::crypto::mem::Protected;
use crate::crypto::symmetric::{Context, BlockCipherMode};
use crate::crypto::SessionKey;
use crate::types::{PublicKeyAlgorithm, Curve, SymmetricAlgorithm};

/// Backend trait providing basic cryptographic operations.
pub trait Backend {
    /// Returns a string describing the crypto backend.
    fn backend() -> String;

    /// Fills the given buffer with random bytes.
    fn random(buf: &mut [u8]) -> Result<()>;
}

/// Trait for asymmetric cryptographic operations.
pub trait Asymmetric {
    /// Returns whether the backend supports the given public key algorithm.
    fn supports_algo(algo: PublicKeyAlgorithm) -> bool;

    /// Returns whether the backend supports the given curve.
    fn supports_curve(curve: &Curve) -> bool;

    // X25519 operations
    fn x25519_generate_key() -> Result<(Protected, [u8; 32])>;
    fn x25519_derive_public(secret: &Protected) -> Result<[u8; 32]>;
    fn x25519_shared_point(secret: &Protected, public: &[u8; 32]) -> Result<Protected>;

    // X448 operations
    fn x448_generate_key() -> Result<(Protected, [u8; 56])>;
    fn x448_derive_public(secret: &Protected) -> Result<[u8; 56]>;
    fn x448_shared_point(secret: &Protected, public: &[u8; 56]) -> Result<Protected>;

    // Ed25519 operations
    fn ed25519_generate_key() -> Result<(Protected, [u8; 32])>;
    fn ed25519_derive_public(secret: &Protected) -> Result<[u8; 32]>;
    fn ed25519_sign(secret: &Protected, public: &[u8; 32], digest: &[u8]) -> Result<[u8; 64]>;
    fn ed25519_verify(public: &[u8; 32], digest: &[u8], signature: &[u8; 64]) -> Result<bool>;

    // Ed448 operations
    fn ed448_generate_key() -> Result<(Protected, [u8; 57])>;
    fn ed448_derive_public(secret: &Protected) -> Result<[u8; 57]>;
    fn ed448_sign(secret: &Protected, public: &[u8; 57], digest: &[u8]) -> Result<[u8; 114]>;
    fn ed448_verify(public: &[u8; 57], digest: &[u8], signature: &[u8; 114]) -> Result<bool>;

    // ML-DSA operations
    fn mldsa65_generate_key() -> Result<(Protected, Box<[u8; 1952]>)>;
    fn mldsa65_sign(secret: &Protected, digest: &[u8]) -> Result<Box<[u8; 3309]>>;
    fn mldsa65_verify(public: &[u8; 1952], digest: &[u8], signature: &[u8; 3309]) -> Result<bool>;

    fn mldsa87_generate_key() -> Result<(Protected, Box<[u8; 2592]>)>;
    fn mldsa87_sign(secret: &Protected, digest: &[u8]) -> Result<Box<[u8; 4627]>>;
    fn mldsa87_verify(public: &[u8; 2592], digest: &[u8], signature: &[u8; 4627]) -> Result<bool>;

    // SLH-DSA operations
    fn slhdsa128s_generate_key() -> Result<(Protected, [u8; 32])>;
    fn slhdsa128s_sign(secret: &Protected, digest: &[u8]) -> Result<Box<[u8; 7856]>>;
    fn slhdsa128s_verify(public: &[u8; 32], digest: &[u8], signature: &[u8; 7856]) -> Result<bool>;

    fn slhdsa128f_generate_key() -> Result<(Protected, [u8; 32])>;
    fn slhdsa128f_sign(secret: &Protected, digest: &[u8]) -> Result<Box<[u8; 17088]>>;
    fn slhdsa128f_verify(public: &[u8; 32], digest: &[u8], signature: &[u8; 17088]) -> Result<bool>;

    fn slhdsa256s_generate_key() -> Result<(Protected, Box<[u8; 64]>)>;
    fn slhdsa256s_sign(secret: &Protected, digest: &[u8]) -> Result<Box<[u8; 29792]>>;
    fn slhdsa256s_verify(public: &[u8; 64], digest: &[u8], signature: &[u8; 29792]) -> Result<bool>;

    // ML-KEM operations
    fn mlkem768_generate_key() -> Result<(Protected, Box<[u8; 1184]>)>;
    fn mlkem768_encapsulate(public: &[u8; 1184]) -> Result<(Box<[u8; 1088]>, Protected)>;
    fn mlkem768_decapsulate(secret: &Protected, ciphertext: &[u8; 1088]) -> Result<Protected>;

    fn mlkem1024_generate_key() -> Result<(Protected, Box<[u8; 1568]>)>;
    fn mlkem1024_encapsulate(public: &[u8; 1568]) -> Result<(Box<[u8; 1568]>, Protected)>;
    fn mlkem1024_decapsulate(secret: &Protected, ciphertext: &[u8; 1568]) -> Result<Protected>;

    // DSA operations
    fn dsa_generate_key(p_bits: usize) -> Result<(MPI, MPI, MPI, MPI, ProtectedMPI)>;
    fn dsa_sign(x: &ProtectedMPI, p: &MPI, q: &MPI, g: &MPI, y: &MPI, digest: &[u8]) -> Result<(MPI, MPI)>;
    fn dsa_verify(p: &MPI, q: &MPI, g: &MPI, y: &MPI, digest: &[u8], r: &MPI, s: &MPI) -> Result<bool>;

    // Legacy functions - needed for compatibility but not post-quantum
    fn elgamal_generate_key(p_bits: usize) -> Result<(MPI, MPI, MPI, ProtectedMPI)>;
    fn x25519_clamp_secret(secret: &mut [u8]);
}

/// Trait for symmetric cryptographic operations.
pub trait Symmetric {
    /// Returns whether the backend supports the given symmetric algorithm.
    fn supports_algo(algo: SymmetricAlgorithm) -> bool;

    /// Creates an encryptor for the given algorithm and mode.
    fn encryptor_impl(
        algo: SymmetricAlgorithm,
        mode: BlockCipherMode,
        key: &Protected,
        iv: Cow<'_, [u8]>,
    ) -> Result<Box<dyn Context>>;

    /// Creates a decryptor for the given algorithm and mode.
    fn decryptor_impl(
        algo: SymmetricAlgorithm,
        mode: BlockCipherMode,
        key: &Protected,
        iv: Cow<'_, [u8]>,
    ) -> Result<Box<dyn Context>>;

    /// Creates an encryptor for the given algorithm and mode (wrapper).
    fn encryptor(
        algo: SymmetricAlgorithm,
        mode: BlockCipherMode,
        key: &Protected,
        iv: Option<&[u8]>,
    ) -> Result<Box<dyn Context>> {
        let empty: &[u8] = &[];
        let iv_cow = match iv {
            Some(iv) => Cow::Borrowed(iv),
            None => Cow::Borrowed(empty),
        };
        Self::encryptor_impl(algo, mode, key, iv_cow)
    }

    /// Creates a decryptor for the given algorithm and mode (wrapper).
    fn decryptor(
        algo: SymmetricAlgorithm,
        mode: BlockCipherMode,
        key: &Protected,
        iv: Option<&[u8]>,
    ) -> Result<Box<dyn Context>> {
        let empty: &[u8] = &[];
        let iv_cow = match iv {
            Some(iv) => Cow::Borrowed(iv),
            None => Cow::Borrowed(empty),
        };
        Self::decryptor_impl(algo, mode, key, iv_cow)
    }
}

/// Trait for key derivation functions.
pub trait Kdf {
    /// HKDF using SHA-256.
    fn hkdf_sha256(
        ikm: &SessionKey,
        salt: Option<&[u8]>,
        info: &[u8],
        okm: &mut SessionKey,
    ) -> Result<()>;

    /// HKDF using SHA-512.
    fn hkdf_sha512(
        ikm: &SessionKey,
        salt: Option<&[u8]>,
        info: &[u8],
        okm: &mut SessionKey,
    ) -> Result<()>;
}