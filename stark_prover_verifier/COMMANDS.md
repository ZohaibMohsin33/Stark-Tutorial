# STARK Prover & Verifier - Command Reference

## 📍 Location
```
c:\Users\PC-09\Desktop\starknet\stark_prover_verifier
```

## 🎯 Three Commands to Remember

### 1️⃣ See Everything Work (Demo)
```powershell
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier
python main.py demo
```
**What it does:** Shows complete proof generation and verification workflow  
**Runtime:** 2-3 seconds  
**Perfect for:** First-time verification that everything works

---

### 2️⃣ Generate a Proof
```powershell
python main.py prove <number>
```

**Examples:**
```powershell
python main.py prove 5       # Fast
python main.py prove 10      # Fast  
python main.py prove 15      # Fast
python main.py prove 20      # Still fast
python main.py prove 30      # ~1 second
```

**Output:** Creates `proof_fib_<number>.json` file  
**Security Level:** 128-bit

---

### 3️⃣ Verify a Proof
```powershell
python main.py verify proof_fib_<number>.json
```

**Examples:**
```powershell
python main.py verify proof_fib_5.json
python main.py verify proof_fib_10.json
python main.py verify proof_fib_20.json
```

**Result:** Shows VALID ✓ or INVALID ✗

---

## 🚀 Quick Start (Copy & Paste)

```powershell
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier
python main.py demo
```

That's all you need to see it working!

---

## 📚 Full Workflow

```powershell
# Step 1: Navigate to project
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier

# Step 2: See the demo
python main.py demo

# Step 3: Generate your own proofs
python main.py prove 10
python main.py prove 15
python main.py prove 20

# Step 4: Verify the proofs
python main.py verify proof_fib_10.json
python main.py verify proof_fib_15.json
python main.py verify proof_fib_20.json

# Step 5: View the proof structure
type proof_fib_10.json
```

---

## 📂 What You Have

```
stark_prover_verifier/
├── main.py                          # Main program (EXECUTE THIS)
├── README.md                        # Full documentation
├── QUICKSTART.md                    # Quick guide
├── COMMANDS.md                      # This file
├── src/
│   ├── prover.py                   # Proof generation
│   └── verifier.py                 # Proof verification
├── cairo/
│   └── simple_fibonacci.cairo       # Cairo example
└── proof_fib_*.json                # Generated proofs
```

---

## ✅ Test All Features

```powershell
# 1. Full demo
python main.py demo

# 2. Test proof generation (small to large)
python main.py prove 5
python main.py prove 10
python main.py prove 15
python main.py prove 20

# 3. Verify all generated proofs
python main.py verify proof_fib_5.json
python main.py verify proof_fib_10.json
python main.py verify proof_fib_15.json
python main.py verify proof_fib_20.json

# 4. View a proof file
type proof_fib_10.json
```

---

## 🎓 What Each Command Does

### `python main.py demo`
- ✓ Computes fibonacci(10) = 55
- ✓ Generates STARK proof
- ✓ Verifies the proof (passes all checks)
- ✓ Tests tampering detection
- ✓ Verifies multiple computations
- **Total time:** ~2-3 seconds

### `python main.py prove 10`
- ✓ Computes fibonacci(10)
- ✓ Records computation trace (19 steps)
- ✓ Creates cryptographic commitment
- ✓ Evaluates constraints
- ✓ Generates FRI layers
- ✓ Saves proof to `proof_fib_10.json`
- **Time:** <1 second

### `python main.py verify proof_fib_10.json`
- ✓ Loads proof from file
- ✓ Checks structure validity
- ✓ Verifies trace commitment
- ✓ Validates constraint evaluations
- ✓ Confirms FRI layers
- ✓ Recomputes and checks challenge
- ✓ Verifies security level
- **Result:** VALID ✓ or INVALID ✗
- **Time:** <1 second

---

## 💡 Key Concepts

**Prover:** Creates a proof showing fibonacci(10) = 55 was computed correctly

**Verifier:** Checks the proof without re-computing fibonacci(10)

**STARK:** Scalable Transparent Argument of Knowledge - a type of cryptographic proof

**Security:** 128-bit security (equivalent to password with 2^128 possibilities)

---

## 📋 Command Syntax

```
python main.py <command> [arguments]

Commands:
  demo                       - Run full demonstration
  prove <n>                  - Generate proof for fibonacci(n)
  verify <proof_file>        - Verify a proof from JSON file
  --help or -h               - Show help message
```

---

## 🔍 Example Proof File

After running `python main.py prove 10`, you'll have `proof_fib_10.json`:

```json
{
  "version": "1.0",
  "computation": "fibonacci",
  "result": 55,
  "trace_commitment": "dd4c4b13f7bb1bef...",
  "constraint_evaluations": [1, 0, 1, 1, 2, 1, 3, 2, 5, 3],
  "challenge": "c98f0a8484462ebc",
  "fri_layers": [
    "97fc4ef2f7c8e2d25b2a146d2a6bf56f...",
    "b5ef4565a8be7ac525af7bff9e35e7ba...",
    "317eb1a0eada0cbab9ee97531820411..."
  ],
  "timestamp": 1765267439.761,
  "security_bits": 128
}
```

---

## ⚡ Performance

| Operation | Time | Notes |
|-----------|------|-------|
| `demo` | 2-3s | All features |
| `prove 5` | <1s | Very fast |
| `prove 10` | <1s | Very fast |
| `prove 15` | <1s | Fast |
| `prove 20` | ~1s | Still fast |
| `prove 30` | ~1s | Reasonable |
| `verify` | <1s | Always fast |

---

## 🎯 Recommended Usage

### First Time
```powershell
python main.py demo
```

### Learn the System
```powershell
python main.py prove 10
python main.py verify proof_fib_10.json
type proof_fib_10.json
```

### Explore Values
```powershell
python main.py prove 5
python main.py prove 15
python main.py prove 20
python main.py verify proof_fib_5.json
python main.py verify proof_fib_15.json
python main.py verify proof_fib_20.json
```

### Advanced
Read and modify:
- `src/prover.py` - Proof generation logic
- `src/verifier.py` - Verification logic
- Add your own computations

---

## 📖 Documentation Files

1. **COMMANDS.md** (this file) - Quick command reference
2. **QUICKSTART.md** - Getting started guide
3. **README.md** - Complete documentation

---

## ✨ Ready to Start?

Run this NOW:
```powershell
cd c:\Users\PC-09\Desktop\starknet\stark_prover_verifier
python main.py demo
```

Enjoy! 🔐✨
