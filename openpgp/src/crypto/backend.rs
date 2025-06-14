//! Concrete implementation of the crypto primitives used by the rest of the
//! crypto API.

pub(crate) mod interface;
pub(crate) mod sha1cd;

// OpenSSL backend with post-quantum crypto support
#[cfg(feature = "crypto-openssl")]
mod openssl;
#[cfg(feature = "crypto-openssl")]
pub use self::openssl::*;
#[cfg(feature = "crypto-openssl")]
pub use self::openssl::Backend;