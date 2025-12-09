// src/types.rs - Data structures for proofs and verification
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single step in the computation trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceStep {
    pub step: usize,
    pub operation: String,
    pub input: u64,
    pub output: u64,
    pub depth: usize,
}

/// The complete computation trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofTrace {
    pub steps: Vec<TraceStep>,
    pub inputs: HashMap<String, u64>,
    pub outputs: HashMap<String, u64>,
}

impl ProofTrace {
    /// Create a new proof trace
    pub fn new() -> Self {
        ProofTrace {
            steps: Vec::new(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    /// Add a step to the trace
    pub fn add_step(&mut self, step: TraceStep) {
        self.steps.push(step);
    }

    /// Set an input value
    pub fn set_input(&mut self, name: impl Into<String>, value: u64) {
        self.inputs.insert(name.into(), value);
    }

    /// Set an output value
    pub fn set_output(&mut self, name: impl Into<String>, value: u64) {
        self.outputs.insert(name.into(), value);
    }
}

impl Default for ProofTrace {
    fn default() -> Self {
        Self::new()
    }
}

/// A STARK Proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub version: String,
    pub computation: String,
    pub result: u64,
    pub trace_commitment: String,
    pub constraint_evaluations: Vec<u64>,
    pub challenge: String,
    pub fri_layers: Vec<String>,
    pub timestamp: u64,
    pub security_bits: u32,
}

impl Proof {
    /// Create a new proof
    pub fn new(
        computation: impl Into<String>,
        result: u64,
        trace_commitment: String,
        constraint_evaluations: Vec<u64>,
        challenge: String,
        fri_layers: Vec<String>,
        security_bits: u32,
    ) -> Self {
        Proof {
            version: "1.0".to_string(),
            computation: computation.into(),
            result,
            trace_commitment,
            constraint_evaluations,
            challenge,
            fri_layers,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            security_bits,
        }
    }
}

/// Verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub message: String,
    pub checks_passed: Vec<String>,
    pub checks_failed: Vec<String>,
}

impl VerificationResult {
    /// Create a valid result
    pub fn valid(message: impl Into<String>, checks_passed: Vec<String>) -> Self {
        VerificationResult {
            valid: true,
            message: message.into(),
            checks_passed,
            checks_failed: Vec::new(),
        }
    }

    /// Create an invalid result
    pub fn invalid(message: impl Into<String>, checks_failed: Vec<String>) -> Self {
        VerificationResult {
            valid: false,
            message: message.into(),
            checks_passed: Vec::new(),
            checks_failed,
        }
    }

    /// Print the result nicely
    pub fn print_report(&self) {
        println!("\n{}", "=".repeat(60));
        println!("STARK PROOF VERIFICATION REPORT");
        println!("{}", "=".repeat(60));
        println!("\nStatus: {}", self.message);
        println!("Overall Valid: {}", if self.valid { "YES" } else { "NO" });

        if !self.checks_passed.is_empty() {
            println!("\nChecks Passed ({}):", self.checks_passed.len());
            for check in &self.checks_passed {
                println!("  ✓ {}", check);
            }
        }

        if !self.checks_failed.is_empty() {
            println!("\nChecks Failed ({}):", self.checks_failed.len());
            for check in &self.checks_failed {
                println!("  ✗ {}", check);
            }
        }

        println!("\n{}\n", "=".repeat(60));
    }
}
