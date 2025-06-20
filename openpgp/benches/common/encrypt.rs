use std::sync::OnceLock;

use sequoia_openpgp as openpgp;
use openpgp::cert::Cert;
use openpgp::parse::Parse;
use openpgp::policy::StandardPolicy;
use openpgp::serialize::stream::{
    Armorer, Encryptor, LiteralWriter, Message, Signer,
};

use std::io::Write;

/// Encrypt with password, using a minimal writer stack.
pub fn encrypt_with_password(
    bytes: &[u8],
    password: &str,
) -> openpgp::Result<Vec<u8>> {
    let mut sink = vec![];
    let message =
        Encryptor::with_passwords(Message::new(&mut sink), Some(password))
            .build()?;
    let mut w = LiteralWriter::new(message).build()?;
    w.write_all(bytes)?;
    w.finalize()?;
    Ok(sink)
}

/// Encrypt ignoring revocation or expiration.
/// Uses a minimal writer stack.
pub fn encrypt_to_cert(
    bytes: &[u8],
    cert: &Cert,
) -> openpgp::Result<Vec<u8>> {
    let mut sink = vec![];
    let p = &StandardPolicy::new();
    let recipients = cert
        .keys()
        .with_policy(p, None)
        .supported()
        .for_transport_encryption()
        .for_storage_encryption();
    let message =
        Encryptor::for_recipients(Message::new(&mut sink), recipients)
            .build()?;
    let mut w = LiteralWriter::new(message).build()?;
    w.write_all(bytes)?;
    w.finalize()?;
    Ok(sink)
}

/// Sign ignoring revocation or expiration.
pub fn sign(bytes: &[u8], sender: &Cert) -> openpgp::Result<Vec<u8>> {
    let mut sink = vec![];

    let p = &StandardPolicy::new();
    let signing_keypair = sender
        .keys()
        .with_policy(p, None)
        .secret()
        .for_signing()
        .next()
        .unwrap()
        .key()
        .clone()
        .into_keypair()?;

    let message = Message::new(&mut sink);
    let message = Signer::new(message, signing_keypair)?.build()?;
    let mut w = LiteralWriter::new(message).build()?;
    w.write_all(bytes)?;
    w.finalize()?;
    Ok(sink)
}

/// Encrypt and sign, ignoring revocation or expiration.
/// Uses a realistic writer stack with padding and armor.
pub fn encrypt_to_cert_and_sign(
    bytes: &[u8],
    sender: &Cert,
    recipient: &Cert,
) -> openpgp::Result<Vec<u8>> {
    let mut sink = vec![];

    let p = &StandardPolicy::new();
    let signing_keypair = sender
        .keys()
        .with_policy(p, None)
        .secret()
        .for_signing()
        .next()
        .unwrap()
        .key()
        .clone()
        .into_keypair()?;

    let recipients = recipient
        .keys()
        .with_policy(p, None)
        .supported()
        .for_transport_encryption()
        .for_storage_encryption();

    let message = Message::new(&mut sink);
    let message = Armorer::new(message).build()?;
    let message = Encryptor::for_recipients(message, recipients).build()?;
    let message = Signer::new(message, signing_keypair)?
        //.add_intended_recipient(&recipient)
        .build()?;
    let mut w = LiteralWriter::new(message).build()?;
    w.write_all(bytes)?;
    w.finalize()?;
    Ok(sink)
}

fn zeros_1_mb() -> &'static [u8] {
    static ZEROS_1_MB: OnceLock<Vec<u8>> = OnceLock::new();
    ZEROS_1_MB.get_or_init(|| vec![0; 1024 * 1024])
}

fn zeros_10_mb() -> &'static [u8] {
    static ZEROS_10_MB: OnceLock<Vec<u8>> = OnceLock::new();
    ZEROS_10_MB.get_or_init(|| vec![0; 10 * 1024 * 1024])
}

/// Encrypt a very short, medium and very long message.
pub fn messages() -> impl Iterator<Item = &'static [u8]> {
    [b"Hello world.", zeros_1_mb(), zeros_10_mb()]
        .into_iter()
}

/// Returns the sender key.
pub fn sender() -> &'static Cert {
    static CERT: OnceLock<Cert> = OnceLock::new();
    CERT.get_or_init(|| {
        Cert::from_bytes(
            &include_bytes!("../../tests/data/keys/sender.pgp")[..])
            .unwrap()
    })
}

/// Returns the recipient key.
pub fn recipient() -> &'static Cert {
    static CERT: OnceLock<Cert> = OnceLock::new();
    CERT.get_or_init(|| {
        Cert::from_bytes(
            &include_bytes!("../../tests/data/keys/recipient.pgp")[..])
            .unwrap()
    })
}
