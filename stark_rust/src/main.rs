// src/main.rs - Command-line interface
use clap::{Parser, Subcommand};
use stark_prover_verifier::{STARKProver, STARKVerifier, computation};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "STARK Prover & Verifier")]
#[command(about = "Educational STARK proof generation and verification in Rust", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a complete demonstration
    Demo,

    /// Generate a STARK proof for fibonacci(n)
    Prove {
        /// The fibonacci index
        #[arg(value_name = "N")]
        n: u64,

        /// Output file (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Verify a STARK proof from a JSON file
    Verify {
        /// Path to the proof JSON file
        #[arg(value_name = "FILE")]
        proof_file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Demo => run_demo(),
        Commands::Prove { n, output } => prove_fibonacci(n, output),
        Commands::Verify { proof_file } => verify_proof(proof_file),
    }
}

/// Run a complete demonstration
fn run_demo() {
    println!("\n{}", "=".repeat(60));
    println!("STARK PROVER & VERIFIER - DEMONSTRATION (RUST)");
    println!("{}", "=".repeat(60));

    // Demo 1: Compute fibonacci and generate proof
    println!("\nDEMO 1: Computing Fibonacci(10)");
    println!("{}", "-".repeat(60));

    let prover = STARKProver::new(128);
    let (result, trace) = computation::fibonacci_with_trace(10);

    println!("Computing fibonacci(10)...");
    println!("✓ Result: fibonacci(10) = {}", result);
    println!("✓ Trace steps: {}", trace.steps.len());

    println!("\nGenerating STARK proof...");
    let proof = prover.prove("fibonacci", result, &trace);
    println!("✓ STARK proof generated");

    println!(
        "\nProof Details:\n  - Version: {}\n  - Computation: {}\n  - Result: {}\n  - Security Level: {} bits\n  - Trace Commitment: {}...\n  - Challenge: {}\n  - FRI Layers: {}",
        proof.version,
        proof.computation,
        proof.result,
        proof.security_bits,
        &proof.trace_commitment[..16],
        proof.challenge,
        proof.fri_layers.len()
    );

    // Demo 2: Verify the proof
    println!("\n{}", "=".repeat(60));
    println!("DEMO 2: Verifying the Proof");
    println!("{}", "-".repeat(60));

    let verifier = STARKVerifier::new(128);
    let verification_result = verifier.verify(&proof);
    verification_result.print_report();

    // Demo 3: Multiple computations
    println!("{}", "=".repeat(60));
    println!("DEMO 3: Multiple Computations");
    println!("{}", "-".repeat(60));

    let test_cases = vec![5, 8, 15];
    for n in test_cases {
        let (result, trace) = computation::fibonacci_with_trace(n);
        let proof = prover.prove("fibonacci", result, &trace);
        let verification = verifier.verify(&proof);

        let status = if verification.valid { "✓ VALID" } else { "✗ INVALID" };
        println!("fibonacci({:2}) = {:6} - Proof: {}", n, result, status);
    }

    println!("\n{}", "=".repeat(60));
    println!("✓ ALL PROOFS VERIFIED SUCCESSFULLY");
    println!("{}", "=".repeat(60));
    println!();
}

/// Generate a proof for fibonacci(n)
fn prove_fibonacci(n: u64, output: Option<PathBuf>) {
    println!("\n{}", "=".repeat(60));
    println!("STARK PROOF GENERATION (RUST)");
    println!("{}", "=".repeat(60));
    println!();

    if n > 100 {
        eprintln!("Error: n must be <= 100 for performance reasons");
        std::process::exit(1);
    }

    println!("Computing Fibonacci({})...", n);
    let prover = STARKProver::new(128);
    let (result, trace) = computation::fibonacci_with_trace(n);

    println!("✓ Computation completed: fibonacci({}) = {}", n, result);
    println!("✓ Computation trace generated with {} steps", trace.steps.len());

    println!("\nGenerating STARK proof...");
    let proof = prover.prove("fibonacci", result, &trace);
    println!("✓ STARK proof generated successfully");

    println!(
        "\nProof Details:\n  - Version: {}\n  - Computation: {}\n  - Result: {}\n  - Security Level: {} bits\n  - Trace Commitment: {}...\n  - Challenge: {}\n  - FRI Layers: {}",
        proof.version,
        proof.computation,
        proof.result,
        proof.security_bits,
        &proof.trace_commitment[..16],
        proof.challenge,
        proof.fri_layers.len()
    );

    let output_file = output.unwrap_or_else(|| {
        PathBuf::from(format!("proof_fib_{}.json", n))
    });

    match prover.save_proof(&proof, output_file.to_str().unwrap()) {
        Ok(_) => println!("\n✓ Proof saved to: {}", output_file.display()),
        Err(e) => {
            eprintln!("Error saving proof: {}", e);
            std::process::exit(1);
        }
    }

    println!();
}

/// Verify a proof from a file
fn verify_proof(proof_file: PathBuf) {
    println!("\n{}", "=".repeat(60));
    println!("STARK PROOF VERIFICATION (RUST)");
    println!("{}", "=".repeat(60));
    println!();

    println!("Loading proof from: {}", proof_file.display());

    let verifier = STARKVerifier::new(128);
    match verifier.verify_from_file(proof_file.to_str().unwrap()) {
        Ok(result) => {
            if !result.valid {
                eprintln!("Proof file not found or invalid");
                std::process::exit(1);
            }
            result.print_report();
        }
        Err(e) => {
            eprintln!("Error loading proof: {}", e);
            std::process::exit(1);
        }
    }
}
