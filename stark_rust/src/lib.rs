// src/lib.rs - Library root file
pub mod prover;
pub mod verifier;
pub mod types;
pub mod crypto;
pub mod computation;

pub use prover::STARKProver;
pub use verifier::STARKVerifier;
pub use types::{Proof, ProofTrace, VerificationResult};
