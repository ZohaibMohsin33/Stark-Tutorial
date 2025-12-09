# STARK Prover & Verifier - Quick Reference Guide

## 🚀 Getting Started (30 seconds)

```powershell
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier
python main.py demo
```

That's it! You'll see a complete demonstration of STARK proofs in action.

---

## 📋 Three Main Commands

### 1. **Demo** - See Everything Working
```powershell
python main.py demo
```
- Generates proof for fibonacci(10)
- Verifies the proof
- Tests tampering detection
- Verifies multiple computations
- **Runtime:** ~2-3 seconds

### 2. **Prove** - Generate a Proof
```powershell
python main.py prove <number>
```

**Examples:**
```powershell
python main.py prove 5       # fibonacci(5) = 5
python main.py prove 10      # fibonacci(10) = 55
python main.py prove 15      # fibonacci(15) = 610
python main.py prove 20      # fibonacci(20) = 6765
python main.py prove 30      # fibonacci(30) = 832040
```

**Output:** Creates `proof_fib_<number>.json` file

### 3. **Verify** - Check a Proof
```powershell
python main.py verify <proof_file>
```

**Examples:**
```powershell
python main.py verify proof_fib_10.json
python main.py verify proof_fib_20.json
```

---

## 📊 Complete Workflow Example

```powershell
# 1. Navigate to project
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier

# 2. Generate a proof for fibonacci(15)
python main.py prove 15

# Output:
# ✓ Computation completed: fibonacci(15) = 610
# ✓ STARK proof generated successfully
# ✓ Proof saved to: proof_fib_15.json

# 3. Verify the proof
python main.py verify proof_fib_15.json

# Output:
# Status: Proof is VALID
# Overall Valid: YES
# ✓ All checks passed

# 4. View the proof file (JSON format)
type proof_fib_15.json
```

---

## ⚡ Quick Test

Recommended sequence to see everything:

```powershell
# First - see the demo
python main.py demo

# Then - generate your own proof
python main.py prove 5
python main.py prove 10
python main.py prove 20

# Then - verify them
python main.py verify proof_fib_5.json
python main.py verify proof_fib_10.json
python main.py verify proof_fib_20.json
```

---

## 📁 Project Files

```
stark_prover_verifier/
├── main.py                  # Main executable - RUN THIS
├── README.md               # Full documentation
├── QUICKSTART.md           # This file
├── src/
│   ├── prover.py          # Proof generation logic
│   └── verifier.py        # Proof verification logic
├── cairo/
│   └── simple_fibonacci.cairo  # Cairo program
└── proof_fib_*.json       # Generated proofs (created when you run commands)
```

---

## 🎯 What You Need to Know

### Prover
- Takes a number `n`
- Computes fibonacci(n)
- Records all computation steps (trace)
- Creates a STARK proof
- Saves to `proof_fib_n.json`

### Verifier
- Reads a proof file
- Performs 6 verification checks:
  1. ✓ Structure validity
  2. ✓ Trace commitment
  3. ✓ Constraint evaluations
  4. ✓ FRI proof layers
  5. ✓ Challenge consistency
  6. ✓ Security level

- Reports: VALID ✓ or INVALID ✗

---

## 🔒 What Makes It Secure

- **Cryptographic hashing** (SHA-256)
- **Polynomial commitments** (FRI protocol)
- **Constraint verification** (computation correctness)
- **Challenge-response** (soundness)
- **128-bit security** (default)

---

## 📈 Performance Guide

| Command | Input | Time | Notes |
|---------|-------|------|-------|
| `demo` | Built-in | 2-3s | All features |
| `prove 5` | n=5 | <1s | Very fast |
| `prove 10` | n=10 | <1s | Fast |
| `prove 15` | n=15 | <1s | Still fast |
| `prove 20` | n=20 | ~1s | Slightly slower |
| `prove 30` | n=30 | ~1s | Acceptable |
| `prove 40` | n=40 | ~5s | Get coffee ☕ |
| `verify` | Any | <1s | Always fast |

**Recommendation:** Use n=5-20 for testing

---

## 💡 Understanding the Proof

When you run `python main.py prove 10`, a file `proof_fib_10.json` is created:

```json
{
  "version": "1.0",
  "computation": "fibonacci",
  "result": 55,
  "trace_commitment": "dd4c4b13f7bb1bef...",
  "constraint_evaluations": [1, 0, 1, 1, 2, ...],
  "challenge": "c98f0a8484462ebc",
  "fri_layers": ["97fc4ef2f...", "b5ef4565a8...", "317eb1a0e..."],
  "timestamp": 1765267439.76,
  "security_bits": 128
}
```

**Components:**
- `result`: The actual answer (fibonacci(10) = 55)
- `trace_commitment`: Hash of computation steps (proves work was done)
- `constraint_evaluations`: Values proving computation correctness
- `challenge`: Random value ensuring soundness
- `fri_layers`: Cryptographic proof layers
- `security_bits`: Proof strength (128 = very secure)

---

## ✅ Verification Checks Explained

When verifying a proof, these checks happen:

1. **Structure Valid?** - All required fields present
2. **Trace Committed?** - Commitment hash is properly formed
3. **Constraints Valid?** - Polynomial values are correct
4. **FRI Layers?** - Proof layers are correctly hashed
5. **Challenge OK?** - Challenge matches expected value
6. **Security Adequate?** - 128-bit security confirmed

All must pass for proof to be VALID ✓

---

## 🐛 Troubleshooting

### Python not found
```powershell
# Check version
python --version

# If not found, install from https://www.python.org/downloads/
```

### Proof file not found
```powershell
# Make sure you're in the right directory
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier

# List existing proofs
Get-ChildItem proof_*.json

# Verify correct filename
python main.py verify proof_fib_10.json
```

### Command not recognized
```powershell
# Make sure you're in the project directory
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier

# Try again
python main.py demo
```

---

## 🎓 Learning Path

1. **First**: Run the demo
   ```powershell
   python main.py demo
   ```

2. **Then**: Generate some proofs
   ```powershell
   python main.py prove 5
   python main.py prove 10
   python main.py prove 20
   ```

3. **Then**: Verify them
   ```powershell
   python main.py verify proof_fib_5.json
   python main.py verify proof_fib_10.json
   ```

4. **Then**: Read the proofs
   ```powershell
   type proof_fib_10.json
   ```

5. **Finally**: Read the code
   - `src/prover.py` - How proofs are made
   - `src/verifier.py` - How proofs are verified

---

## 🎯 Common Use Cases

### Run a Full Demo
```powershell
python main.py demo
```

### Generate a Single Proof
```powershell
python main.py prove 10
```

### Verify a Specific Proof
```powershell
python main.py verify proof_fib_10.json
```

### Test Multiple Values
```powershell
python main.py prove 5
python main.py prove 10
python main.py prove 15
python main.py verify proof_fib_5.json
python main.py verify proof_fib_10.json
python main.py verify proof_fib_15.json
```

### Batch Verification
```powershell
# Verify all proofs
Get-ChildItem proof_*.json | ForEach-Object {
    python main.py verify $_.Name
}
```

---

## 📞 Need Help?

- **Full Documentation**: See `README.md`
- **Code Comments**: Check `src/prover.py` and `src/verifier.py`
- **Examples**: Run `python main.py -h` for help

---

## ✨ That's It!

You now have a working STARK prover and verifier. Start with:

```powershell
python main.py demo
```

Enjoy proving! 🔐✨
