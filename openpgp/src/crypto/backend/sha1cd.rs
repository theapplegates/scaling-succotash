//! SHA-1 collision detection support.

use std::io::Write;
use crate::Result;
use crate::crypto::hash::Digest;
use sha1collisiondetection::Sha1CD;

/// SHA-1 with collision detection.
#[derive(Clone)]
pub struct Sha1 {
    hasher: Sha1CD,
}

impl Sha1 {
    /// Creates a new SHA-1 hasher with collision detection.
    pub fn new() -> Self {
        Self {
            hasher: Sha1CD::default(),
        }
    }

    /// Updates the hasher with the given data.
    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    /// Finalizes the hash and returns the digest.
    pub fn finalize(self) -> Result<[u8; 20]> {
        let digest_result = self.hasher.finalize_cd();
        match digest_result {
            Ok(digest) => {
                let mut result = [0u8; 20];
                result.copy_from_slice(&digest);
                Ok(result)
            }
            Err(_collision) => {
                Err(crate::Error::InvalidArgument(
                    "SHA-1 collision detected".into(),
                ).into())
            }
        }
    }
}

impl Default for Sha1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Digest for Sha1 {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    fn digest(&mut self, digest: &mut [u8]) -> Result<()> {
        // Create a clone of the current hasher state
        let hasher_clone = self.hasher.clone();
        let hash_result = hasher_clone.finalize_cd();
        match hash_result {
            Ok(hash) => {
                let copy_len = digest.len().min(hash.len());
                digest[..copy_len].copy_from_slice(&hash[..copy_len]);
                
                // Reset the hasher
                self.hasher = Sha1CD::default();
                Ok(())
            }
            Err(_collision) => {
                Err(crate::Error::InvalidArgument(
                    "SHA-1 collision detected".into(),
                ).into())
            }
        }
    }
}

impl Write for Sha1 {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Creates a new SHA-1 hasher with collision detection.
pub fn build() -> Box<dyn Digest> {
    Box::new(Sha1::new())
}