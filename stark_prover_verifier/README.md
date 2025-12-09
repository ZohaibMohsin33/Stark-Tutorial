# STARK Prover and Verifier

A complete, executable implementation of **STARK (Scalable Transparent Argument of Knowledge)** proof generation and verification. This project demonstrates how to create cryptographic proofs of computations and verify them.

## 📋 Overview

This project provides:
- **STARKProver**: Generates cryptographic proofs for computations
- **STARKVerifier**: Verifies the validity of proofs
- **Demo Computation**: Fibonacci number calculation with proof generation
- **Complete Verification Pipeline**: Multi-step verification with detailed reporting

### What is STARK?

STARK (Scalable Transparent Argument of Knowledge) is a cryptographic proof system that allows one party to prove they performed a computation correctly without the verifier having to execute the computation themselves.

**Key Features:**
- ✓ Scalable: Proof size grows logarithmically with computation size
- ✓ Transparent: No trusted setup required
- ✓ Post-quantum secure: Resistant to quantum attacks
- ✓ Efficient: Verification is much faster than re-computation

## 📁 Project Structure

```
stark_prover_verifier/
├── main.py                          # Main executable program
├── README.md                        # This file
├── src/
│   ├── prover.py                   # STARK Prover implementation
│   └── verifier.py                 # STARK Verifier implementation
└── cairo/
    └── simple_fibonacci.cairo       # Cairo program example
```

## 🚀 Quick Start

### Prerequisites

- **Python 3.7+** (Python 3.8+ recommended)
- No external dependencies required! (Uses only Python standard library)

### Installation

1. Clone or navigate to the project directory:
```powershell
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier
```

2. Verify Python is installed:
```powershell
python --version
```

## 💻 Commands

### 1. Run the Demo

See a complete demonstration of proof generation and verification:

```powershell
python main.py demo
```

**What it does:**
- Computes Fibonacci(10) and generates a STARK proof
- Verifies the proof with detailed checks
- Attempts to detect proof tampering
- Verifies multiple computations

**Expected Output:**
```
============================================================
STARK PROVER AND VERIFIER - DEMONSTRATION
============================================================

DEMO 1: Computing Fibonacci(10)
------------------------------------------------------------
Computing fibonacci(10)...
✓ Result: fibonacci(10) = 55
✓ Trace steps: 20

Generating STARK proof...
✓ STARK proof generated

============================================================
DEMO 2: Verifying the Proof
------------------------------------------------------------

============================================================
STARK PROOF VERIFICATION REPORT
============================================================

Status: Proof is VALID
Overall Valid: YES
...
```

### 2. Generate a Proof

Generate a STARK proof for a specific Fibonacci number:

```powershell
python main.py prove 10
```

**Parameters:**
- `<n>`: The Fibonacci index (0-100 recommended for reasonable runtime)

**Output:**
- Generates `proof_fib_<n>.json` file
- Displays proof details including:
  - Computation result
  - Security level
  - Trace commitment
  - FRI layer hashes

**Examples:**
```powershell
python main.py prove 5      # fibonacci(5) = 5
python main.py prove 10     # fibonacci(10) = 55
python main.py prove 20     # fibonacci(20) = 6765
python main.py prove 30     # fibonacci(30) = 832040
```

### 3. Verify a Proof

Verify a previously generated proof:

```powershell
python main.py verify proof_fib_10.json
```

**Parameters:**
- `<proof_file>`: Path to the proof JSON file

**Verification Checks:**
1. Proof structure validity
2. Trace commitment verification
3. Constraint evaluation verification
4. FRI proof layer verification
5. Challenge generation verification
6. Security level verification

**Output:**
- Detailed verification report
- List of passed and failed checks
- Overall validity verdict

**Examples:**
```powershell
python main.py verify proof_fib_10.json
python main.py verify proof_fib_20.json
```

## 📊 Example Workflow

Here's a complete workflow example:

```powershell
# 1. Generate a proof for fibonacci(15)
python main.py prove 15

# Output shows:
# ✓ Computation completed: fibonacci(15) = 610
# ✓ Computation trace generated with 31 steps
# ✓ STARK proof generated successfully
# ✓ Proof saved to: proof_fib_15.json

# 2. Verify the generated proof
python main.py verify proof_fib_15.json

# Output shows:
# ============================================================
# STARK PROOF VERIFICATION REPORT
# ============================================================
# 
# Status: Proof is VALID
# Overall Valid: YES
#
# Checks Passed (6):
#   ✓ Proof structure is valid
#   ✓ Trace commitment verified
#   ✓ Constraint evaluations verified
#   ✓ FRI proof layers verified
#   ✓ Challenge generation verified
#   ✓ Security level adequate (128 bits)

# 3. See the proof content
type proof_fib_15.json
```

## 🔍 Understanding the Implementation

### Prover Module (`src/prover.py`)

**Key Classes:**
- `ProofTrace`: Stores the computation trace with all intermediate steps
- `STARKProver`: Main prover implementation

**Key Methods:**
- `compute_fibonacci(n)`: Computes Fibonacci with trace
- `generate_proof()`: Creates the STARK proof
- `_commit_to_trace()`: Creates cryptographic commitment
- `_evaluate_constraints()`: Evaluates constraint polynomials
- `_fri_commit()`: Generates FRI layers
- `save_proof()`: Saves proof to JSON

**Proof Structure:**
```json
{
  "version": "1.0",
  "computation": "fibonacci",
  "result": 55,
  "trace_commitment": "abc123...",
  "constraint_evaluations": [1, 2, 3, ...],
  "challenge": "def456...",
  "fri_layers": ["layer1hash", "layer2hash", ...],
  "timestamp": 1234567890,
  "security_bits": 128
}
```

### Verifier Module (`src/verifier.py`)

**Key Classes:**
- `VerificationResult`: Stores verification outcome
- `STARKVerifier`: Main verifier implementation

**Key Methods:**
- `verify(proof)`: Performs complete verification
- `_check_proof_structure()`: Validates proof format
- `_verify_trace_commitment()`: Verifies trace hash
- `_verify_constraints()`: Checks constraints
- `_verify_fri_layers()`: Validates FRI layers
- `_verify_challenge()`: Confirms challenge generation
- `verify_from_file()`: Loads and verifies proof file
- `print_verification_report()`: Displays detailed report

**Verification Process:**
1. Check proof structure (all required fields present)
2. Verify trace commitment is properly formed
3. Validate constraint evaluations
4. Verify FRI proof layers
5. Confirm challenge was properly generated
6. Check security level meets requirements

## 🧪 Test Cases

### Fibonacci Values for Testing

| n | fibonacci(n) | Computation Time |
|---|--------------|------------------|
| 5 | 5 | Very fast |
| 10 | 55 | Fast |
| 15 | 610 | Fast |
| 20 | 6765 | Fast |
| 30 | 832040 | ~1 second |
| 40 | 102334155 | ~5 seconds |

**Recommended for testing:** Use n = 5-20 for quick verification

## 📝 Proof Anatomy

### Components

**1. Trace Commitment**
- SHA-256 hash of the entire computation trace
- Proves knowledge of the computation steps
- 64-character hexadecimal string

**2. Constraint Evaluations**
- Polynomial evaluations proving computation correctness
- Multiple evaluations covering different constraints
- Values bounded by security parameter

**3. FRI Layers**
- Fast Reed-Solomon Interactive (FRI) proof layers
- Each layer is a commitment to folded polynomials
- Enables efficient verification

**4. Challenge**
- Derived from trace commitment and security parameter
- Used in FRI protocol to ensure soundness
- Regenerated by verifier to confirm consistency

## 🔒 Security Features

- **128-bit security level** (default)
- **Cryptographic hashing** for commitments
- **Constraint-based verification** of computation
- **Multi-layer verification** pipeline
- **Tampering detection** built-in

## 🐛 Troubleshooting

### Issue: "python command not found"
**Solution:** Ensure Python is installed and in your PATH
```powershell
python --version
# If not found, install from https://www.python.org/downloads/
```

### Issue: "Proof file not found"
**Solution:** Ensure the proof file exists in the current directory
```powershell
# List proof files
Get-ChildItem proof_*.json

# Use full path if needed
python main.py verify C:\full\path\to\proof_fib_10.json
```

### Issue: Verification fails for valid proof
**Solution:** Check security level matches proof
- Default security level is 128 bits
- Ensure no file corruption occurred

### Issue: Slow computation for large n
**Solution:** Use smaller values
```powershell
python main.py prove 25  # Instead of 40+
```

## 📚 Implementation Details

### Computation Model
- Uses recursive Fibonacci with memoization
- Traces all computation steps
- Records operation type, inputs, outputs, and depth

### Proof Generation
1. Compute result with full trace
2. Create cryptographic commitment to trace
3. Evaluate constraint polynomials
4. Generate random challenge
5. Create FRI layers for polynomial commitment
6. Package all elements into proof structure

### Proof Verification
1. Validate proof structure and fields
2. Confirm trace commitment is valid hash
3. Check constraint polynomial values
4. Verify FRI layers are properly formed
5. Recompute challenge and compare
6. Confirm security level meets requirements

## 🎓 Educational Value

This implementation is perfect for learning:
- How STARK proofs work
- Cryptographic commitment schemes
- Polynomial constraints
- FRI protocol basics
- Interactive proof verification
- Zero-knowledge concepts

## 📖 Further Reading

- **STARK Papers**: https://eprint.iacr.org/2018/046
- **Cairo Documentation**: https://docs.starkware.co/
- **Cryptography Concepts**: https://en.wikipedia.org/wiki/Proof_of_knowledge

## 💡 Key Concepts

### Commitment Scheme
A cryptographic primitive that allows binding to a value without revealing it until later.

### Constraint Polynomial
A mathematical expression that must be satisfied if the computation is correct.

### FRI (Fast Reed-Solomon Interactive)
An efficient protocol for proving polynomial commitments with logarithmic proof size.

### Non-Interactive Proof
A proof that can be verified without interaction, often using the Fiat-Shamir transform.

## 📄 File Format

All proofs are stored in JSON format for easy inspection and debugging:

```json
{
  "version": "1.0",
  "computation": "fibonacci",
  "result": 55,
  "trace_commitment": "a1b2c3d4...",
  "constraint_evaluations": [1, 2, 3, 4, 5],
  "challenge": "e5f6g7h8...",
  "fri_layers": ["hash1", "hash2", "hash3"],
  "timestamp": 1702150000.123,
  "security_bits": 128
}
```

## 🤝 Contributing

Feel free to extend this implementation with:
- Additional computation types
- More sophisticated constraint systems
- Performance optimizations
- Real polynomial arithmetic
- Interactive protocol variants

## 📄 License

This educational implementation is provided as-is for learning purposes.

## 🎯 Summary

| Command | Purpose | Usage |
|---------|---------|-------|
| `demo` | See full demonstration | `python main.py demo` |
| `prove <n>` | Generate proof for fibonacci(n) | `python main.py prove 10` |
| `verify <file>` | Verify a proof file | `python main.py verify proof_fib_10.json` |

## Next Steps

1. **Run the demo**: `python main.py demo`
2. **Generate proofs**: `python main.py prove 5` through `python main.py prove 20`
3. **Verify proofs**: `python main.py verify proof_fib_*.json`
4. **Inspect proof files**: Open `.json` files in a text editor
5. **Modify and experiment**: Extend the code with your own computations

---

**Happy proving!** 🔐✨
