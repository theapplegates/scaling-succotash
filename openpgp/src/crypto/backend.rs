//! Concrete implementation of the crypto primitives used by the rest of the
//! crypto API.

pub(crate) mod interface;
pub(crate) mod sha1cd;

// Nettle is the default backend, but on Windows targets we instead
// enable CNG for running the tests in non-leaf crates that depend on
// sequoia-openpgp.  This creates a conflict, and makes `cargo test`
// fail.  To mitigate this, only enable the Nettle backend if we are
// not compiling the tests and have a different backend selected.
//
// Note: If you add a new crypto backend, add it to the expression,
// and also synchronize the expression to `build.rs`.
#[cfg(all(feature = "crypto-openssl",
          not(all(feature = "__implicit-crypto-backend-for-tests",
                  any(feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-fuzzing",
                      feature = "crypto-openssl")))))]
mod nettle;
#[cfg(all(feature = "crypto-openssl",
          not(all(feature = "__implicit-crypto-backend-for-tests",
                  any(feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-fuzzing",
                      feature = "crypto-openssl")))))]
pub use self::nettle::*;
#[cfg(all(feature = "crypto-openssl",
          not(all(feature = "__implicit-crypto-backend-for-tests",
                  any(feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl")))))]
pub use self::nettle::Backend;

// Nettle is the default backend, but on Windows targets we instead
// enable CNG for running the tests in non-leaf crates that depend on
// sequoia-openpgp.  This creates a conflict, and makes `cargo test`
// fail.  To mitigate this, only enable the CNG backend if we are
// not compiling the tests and have a different backend selected.
//
// Note: If you add a new crypto backend, add it to the expression,
// and also synchronize the expression to `build.rs`.
#[cfg(all(feature = "crypto-openssl",
          not(all(feature = "__implicit-crypto-backend-for-tests",
                  any(feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-fuzzing",
                      feature = "crypto-openssl")))))]
mod cng;
#[cfg(all(feature = "crypto-openssl",
          not(all(feature = "__implicit-crypto-backend-for-tests",
                  any(feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-fuzzing",
                      feature = "crypto-openssl")))))]
pub use self::cng::*;
#[cfg(all(feature = "crypto-openssl",
          not(all(feature = "__implicit-crypto-backend-for-tests",
                  any(feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-openssl",
                      feature = "crypto-fuzzing",
                      feature = "crypto-openssl")))))]
pub use self::cng::Backend;

#[cfg(feature = "crypto-openssl")]
mod rust;
#[cfg(feature = "crypto-openssl")]
pub use self::rust::*;
#[cfg(feature = "crypto-openssl")]
pub use self::rust::Backend;

#[cfg(feature = "crypto-openssl")]
mod openssl;
#[cfg(feature = "crypto-openssl")]
pub use self::openssl::*;
#[cfg(feature = "crypto-openssl")]
pub use self::openssl::Backend;

#[cfg(any(feature = "crypto-openssl", feature = "crypto-openssl"))]
mod botan;
#[cfg(any(feature = "crypto-openssl", feature = "crypto-openssl"))]
pub use self::botan::*;
#[cfg(feature = "crypto-openssl")]
pub use self::botan::Backend;

#[cfg(feature = "crypto-fuzzing")]
mod fuzzing;
#[cfg(feature = "crypto-fuzzing")]
pub use self::fuzzing::*;
#[cfg(feature = "crypto-fuzzing")]
pub use self::fuzzing::Backend;
