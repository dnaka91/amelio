//! Functionality for hashing passwords.
//!
//! One-way-hashes allow to turn a plaintext password into a hashed version that is unusable without
//! having the original plain password. It protects stolen passwords from being used because only
//! its hashed version is stored and this version can not be turned back into the original one.

use anyhow::Result;

/// A hasher can hash and verify passwords with a _one-way-hash_.
pub trait Hasher {
    /// Turn a plain passwords into a hashed one.
    fn hash(&self, password: &str) -> Result<String>;
    /// Verify a plain password against a previously hashed password.
    fn verify(&self, password: &str, hash: &str) -> Result<bool>;
}

/// Main implementation of [`Hasher`].
struct HasherImpl;

impl Hasher for HasherImpl {
    fn hash(&self, password: &str) -> Result<String> {
        let cost = if cfg!(test) { 4 } else { bcrypt::DEFAULT_COST };
        bcrypt::hash(password, cost).map_err(Into::into)
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool> {
        bcrypt::verify(password, hash).map_err(Into::into)
    }
}

/// Create a new hasher.
pub fn new_hasher() -> impl Hasher {
    HasherImpl
}
