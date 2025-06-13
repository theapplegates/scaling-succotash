use sequoia_openpgp as openpgp;
use openpgp::cert::prelude::*;
use openpgp::cert::CipherSuite;

// Add this:
use openssl; // (You may need to run `cargo add openssl` if it is not in your Cargo.toml)

fn main() -> openpgp::Result<()> {
    println!("OpenSSL version: {}", openssl::version::version());

    let (cert, _rev) = CertBuilder::general_purpose(Some("Paul <me@paulapplegate.com>"))
        .set_cipher_suite(CipherSuite::MLDSA65)
        .set_profile(openpgp::Profile::RFC9580)?
        .generate()?;
    println!("PQC cert: {:?}", cert);
    Ok(())
}