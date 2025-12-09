# Cairo Integration Guide

## 📝 Using Cairo with Rust STARK Prover

This guide shows how to use Cairo programs with the Rust STARK prover.

## 🎯 Overview

### Workflow
```
Cairo Program
    ↓
cairo-compile → Execution Trace
    ↓
Extract Trace → ProofTrace
    ↓
Rust STARK Prover → Generate Proof
    ↓
Rust STARK Verifier → Verify ✓/✗
```

## 🚀 Getting Started

### 1. Install Cairo Compiler

```bash
# Using pip
pip install cairo-lang

# Verify installation
cairo-compile --version
```

### 2. Compile a Cairo Program

```bash
# Compile fibonacci.cairo
cairo-compile cairo/fibonacci.cairo --output fibonacci_compiled.json

# Compile simple_computation.cairo
cairo-compile cairo/simple_computation.cairo --output simple_computation_compiled.json
```

### 3. Run Cairo Program

```bash
# Run the compiled program
cairo-run --program=fibonacci_compiled.json --print_output
```

## 📄 Cairo Program Examples

### Example 1: Simple Fibonacci

**File:** `cairo/fibonacci.cairo`

```cairo
func fib(n: felt) -> (result: felt) {
    if (n == 0) {
        return (result=0);
    }
    if (n == 1) {
        return (result=1);
    }
    
    let (fib_n_minus_1) = fib(n - 1);
    let (fib_n_minus_2) = fib(n - 2);
    
    return (result=fib_n_minus_1 + fib_n_minus_2);
}

func main() {
    let (result) = fib(10);
    assert result = 55;
    return ();
}
```

**Compile and run:**
```bash
cairo-compile cairo/fibonacci.cairo --output fib.json
cairo-run --program=fib.json --print_output
```

### Example 2: Arithmetic Computation

**File:** `cairo/simple_computation.cairo`

```cairo
func compute_sum(a: felt, b: felt, c: felt) -> (result: felt) {
    let sum = a + b + c;
    return (result=sum);
}

func main() {
    let (result) = compute_sum(1, 2, 3);
    assert result = 6;
    return ();
}
```

## 🔄 Integration with Rust Prover

### Step 1: Compile Cairo Program

```bash
cairo-compile cairo/fibonacci.cairo --output fibonacci_compiled.json
```

### Step 2: Extract Execution Trace

The compiled program contains the execution trace. Extract it:

```bash
# Read the JSON to get trace information
cat fibonacci_compiled.json | grep -i "trace"
```

### Step 3: Create Rust Bridge

Create a bridge function in `src/cairo_integration.rs`:

```rust
use crate::types::{ProofTrace, TraceStep};
use std::fs;
use serde_json::Value;

pub fn load_cairo_trace(json_file: &str) -> Result<ProofTrace, Box<dyn std::error::Error>> {
    let json_str = fs::read_to_string(json_file)?;
    let json: Value = serde_json::from_str(&json_str)?;
    
    let mut trace = ProofTrace::new();
    
    // Parse Cairo execution trace
    if let Some(memory) = json.get("execution").and_then(|e| e.get("memory")) {
        // Extract trace information
        // This depends on Cairo's internal representation
    }
    
    Ok(trace)
}
```

### Step 4: Use in Rust Prover

```rust
use stark_prover_verifier::{STARKProver, cairo_integration};

fn main() {
    // Load Cairo execution trace
    let trace = cairo_integration::load_cairo_trace("fibonacci_compiled.json")
        .expect("Failed to load Cairo trace");
    
    // Use with Rust prover
    let prover = STARKProver::new(128);
    let proof = prover.prove("cairo_fibonacci", 55, &trace);
    
    // Save and verify
    prover.save_proof(&proof, "cairo_proof.json").unwrap();
    println!("Cairo proof generated: cairo_proof.json");
}
```

## 📚 Cairo Language Basics

### Variables
```cairo
let x = 5;
let y = 10;
```

### Functions
```cairo
func add(a: felt, b: felt) -> (result: felt) {
    return (result=a + b);
}
```

### Control Flow
```cairo
if (condition) {
    // ...
} else {
    // ...
}
```

### Loops (via recursion)
```cairo
func loop_example(n: felt) {
    if (n == 0) {
        return ();
    }
    loop_example(n - 1);
    return ();
}
```

## 🎯 Common Cairo Patterns

### Pattern 1: Recursive Computation

```cairo
func factorial(n: felt) -> (result: felt) {
    if (n == 0) {
        return (result=1);
    }
    let (smaller) = factorial(n - 1);
    return (result=n * smaller);
}
```

### Pattern 2: Array Operations

```cairo
// Cairo array operations
func array_sum(arr_len: felt, arr: felt*) -> (result: felt) {
    if (arr_len == 0) {
        return (result=0);
    }
    let (rest_sum) = array_sum(arr_len - 1, arr + 1);
    return (result=[arr] + rest_sum);
}
```

### Pattern 3: Assertions (Constraints)

```cairo
func verify_computation(a: felt, b: felt, expected: felt) {
    assert a + b = expected;
    return ();
}
```

## 🔗 Integration Checklist

- [ ] Install Cairo compiler (`pip install cairo-lang`)
- [ ] Write Cairo program in `cairo/` directory
- [ ] Compile with `cairo-compile`
- [ ] Extract execution trace from compiled output
- [ ] Create bridge function in `src/cairo_integration.rs`
- [ ] Load trace in main Rust program
- [ ] Generate proof with `STARKProver`
- [ ] Verify with `STARKVerifier`

## 🧪 Testing Cairo Integration

```bash
# Compile Cairo program
cairo-compile cairo/fibonacci.cairo --output test_fib.json

# Run Cairo program to verify it works
cairo-run --program=test_fib.json --print_output

# Should output: Output: 55
```

## 🚀 Advanced Cairo Features

### Builtins (Mathematical operations)
```cairo
// Range check builtin
%builtins range_check

func check_range(value: felt) {
    [range_check_ptr] = value;
    let range_check_ptr = range_check_ptr + 1;
    return ();
}
```

### Memory Operations
```cairo
// Direct memory access
func memory_operation(ptr: felt*) {
    let x = [ptr];
    let y = [ptr + 1];
    [ptr] = x + y;
    return ();
}
```

### Pedersen Hash
```cairo
// Using Pedersen hash
%builtins pedersen

from starkware.cairo.common.hash import hash2

func hash_values(a: felt, b: felt) -> (hash_result: felt) {
    let (result) = hash2(a, b);
    return (hash_result=result);
}
```

## 📖 Cairo Resources

- **Cairo Official Docs**: https://docs.starkware.co/
- **Cairo Language**: https://github.com/starkware-libs/cairo
- **Cairo Examples**: In this project's `cairo/` directory
- **Starknet Book**: https://book.starknet.io/

## 🎓 Example: Complete Cairo → Rust → Proof

### Step 1: Write Cairo Program

```cairo
// example.cairo
func power(base: felt, exp: felt) -> (result: felt) {
    if (exp == 0) {
        return (result=1);
    }
    let (smaller) = power(base, exp - 1);
    return (result=base * smaller);
}

func main() {
    let (result) = power(2, 10);
    assert result = 1024;
    return ();
}
```

### Step 2: Compile

```bash
cairo-compile example.cairo --output example_compiled.json
```

### Step 3: Extract Trace and Create Proof

```rust
// In Rust
let trace = load_cairo_trace("example_compiled.json")?;
let prover = STARKProver::new(128);
let proof = prover.prove("cairo_power", 1024, &trace);
prover.save_proof(&proof, "power_proof.json")?;
```

### Step 4: Verify Proof

```rust
let verifier = STARKVerifier::new(128);
let result = verifier.verify_from_file("power_proof.json")?;
result.print_report();
```

## 💡 Integration Best Practices

1. **Keep Cairo simple** - Complex Cairo = hard to trace
2. **Test Cairo first** - Verify Cairo program works independently
3. **Use memoization** - For recursive computations
4. **Document traces** - Add comments explaining trace structure
5. **Validate inputs** - Add Cairo assertions for inputs

## 🔒 Security Considerations

- Cairo programs run in a provable VM
- STARK proofs ensure computational integrity
- Use constraints to verify correctness
- Verify security level matches requirements

## ✨ Next Steps

1. Write a Cairo program
2. Compile with cairo-compile
3. Extract execution trace
4. Generate STARK proof
5. Verify proof cryptographically

Ready to prove Cairo computations! 🚀✨
