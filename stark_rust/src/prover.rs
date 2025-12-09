// src/prover.rs - STARK Proof Generation
use crate::types::{Proof, ProofTrace};
use crate::crypto::{hash_integers, hash_string, generate_challenge};

/// STARK Prover
pub struct STARKProver {
    security_level: u32,
}

impl STARKProver {
    /// Create a new prover
    pub fn new(security_level: u32) -> Self {
        STARKProver { security_level }
    }

    /// Generate a STARK proof
    pub fn prove(
        &self,
        computation_name: impl Into<String>,
        result: u64,
        trace: &ProofTrace,
    ) -> Proof {
        let computation = computation_name.into();

        // Step 1: Commit to the trace
        let trace_commitment = self.commit_to_trace(trace);

        // Step 2: Evaluate constraints
        let constraint_evaluations = self.evaluate_constraints(trace);

        // Step 3: Generate challenge
        let challenge = generate_challenge(&trace_commitment, self.security_level);

        // Step 4: Create FRI layers
        let fri_layers = self.create_fri_layers(&constraint_evaluations, &challenge);

        // Create the proof
        Proof::new(
            computation,
            result,
            trace_commitment,
            constraint_evaluations,
            challenge,
            fri_layers,
            self.security_level,
        )
    }

    /// Commit to the trace by hashing it
    fn commit_to_trace(&self, trace: &ProofTrace) -> String {
        let trace_json = serde_json::to_string(&trace)
            .expect("Failed to serialize trace");
        hash_string(&trace_json)
    }

    /// Evaluate constraint polynomials on the trace
    fn evaluate_constraints(&self, trace: &ProofTrace) -> Vec<u64> {
        trace
            .steps
            .iter()
            .map(|step| {
                // Simple constraint: output should be consistent
                step.output % (1u64 << self.security_level.min(32))
            })
            .collect()
    }

    /// Create FRI (Fast Reed-Solomon Interactive) proof layers
    fn create_fri_layers(&self, evaluations: &[u64], challenge: &str) -> Vec<String> {
        let mut layers = Vec::new();
        let mut current_evals = evaluations.to_vec();

        // Generate 3 FRI layers
        for layer_idx in 0..3 {
            if current_evals.is_empty() {
                break;
            }

            // Hash the current evaluations with the challenge
            let layer_data = format!("{}{}{}", 
                current_evals.iter()
                    .take(5)
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                challenge,
                layer_idx
            );

            let layer_hash = hash_string(&layer_data);
            layers.push(layer_hash);

            // Halve the evaluations for the next layer (simulating folding)
            current_evals = current_evals
                .iter()
                .step_by(2)
                .copied()
                .collect();
        }

        layers
    }

    /// Save proof to a JSON file
    pub fn save_proof(&self, proof: &Proof, filename: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&proof)?;
        std::fs::write(filename, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::computation::fibonacci_with_trace;

    #[test]
    fn test_prove_fibonacci() {
        let prover = STARKProver::new(128);
        let (result, trace) = fibonacci_with_trace(10);
        let proof = prover.prove("fibonacci", result, &trace);

        assert_eq!(proof.result, 55);
        assert_eq!(proof.computation, "fibonacci");
        assert_eq!(proof.security_bits, 128);
        assert!(!proof.trace_commitment.is_empty());
        assert!(!proof.constraint_evaluations.is_empty());
        assert!(!proof.fri_layers.is_empty());
    }

    #[test]
    fn test_trace_commitment() {
        let prover = STARKProver::new(128);
        let (_, trace) = fibonacci_with_trace(5);
        let commitment = prover.commit_to_trace(&trace);

        // Should be a valid SHA-256 hash (64 hex characters)
        assert_eq!(commitment.len(), 64);
        assert!(commitment.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
