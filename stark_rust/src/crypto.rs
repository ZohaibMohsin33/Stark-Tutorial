// src/crypto.rs - Cryptographic utilities
use sha2::{Sha256, Digest};

/// Hash a value using SHA-256
pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Hash a string
pub fn hash_string(data: &str) -> String {
    hash_bytes(data.as_bytes())
}

/// Hash a vector of integers
pub fn hash_integers(values: &[u64]) -> String {
    let data = values.iter()
        .map(|v| v.to_le_bytes().to_vec())
        .fold(Vec::new(), |mut acc, v| {
            acc.extend(v);
            acc
        });
    hash_bytes(&data)
}

/// Generate a challenge from commitment and security parameter
pub fn generate_challenge(commitment: &str, security_bits: u32) -> String {
    let challenge_input = format!("{}{}", commitment, security_bits);
    let hash = hash_string(&challenge_input);
    hash[..16].to_string()
}

/// Verify challenge consistency
pub fn verify_challenge(commitment: &str, security_bits: u32, provided_challenge: &str) -> bool {
    let expected_challenge = generate_challenge(commitment, security_bits);
    expected_challenge == provided_challenge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let hash = hash_string("test");
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
    }

    #[test]
    fn test_generate_challenge() {
        let commitment = "abc123";
        let challenge = generate_challenge(commitment, 128);
        assert_eq!(challenge.len(), 16);
    }

    #[test]
    fn test_verify_challenge() {
        let commitment = "test_commitment";
        let challenge = generate_challenge(commitment, 128);
        assert!(verify_challenge(commitment, 128, &challenge));
    }
}
