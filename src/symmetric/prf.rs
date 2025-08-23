use rand::Rng;
use serde::{Serialize, de::DeserializeOwned};

/// Trait to model a pseudorandom function (PRF)
pub trait Pseudorandom {
    type Key: Send + Sync + Serialize + DeserializeOwned;
    type Output;

    /// Sample a random key for the PRF
    fn key_gen<R: Rng>(rng: &mut R) -> Self::Key;

    /// Apply the PRF to an epoch and an index
    fn apply(key: &Self::Key, epoch: u32, index: u64) -> Self::Output;

    /// Function to check internal consistency of any given parameters
    /// For testing only, and expected to panic if something is wrong.
    fn internal_consistency_check();
}

pub mod sha;
pub mod shake_to_field;
