// src/computation.rs - Computation implementations
use crate::types::{ProofTrace, TraceStep};

/// Fibonacci computation with trace
pub fn fibonacci_with_trace(n: u64) -> (u64, ProofTrace) {
    let mut trace = ProofTrace::new();
    trace.set_input("n", n);

    if n > 100 {
        panic!("n must be <= 100 for performance reasons");
    }

    let mut memo = std::collections::HashMap::new();

    fn fib_memo(num: u64, memo: &mut std::collections::HashMap<u64, u64>, trace: &mut ProofTrace, depth: usize) -> u64 {
        if let Some(&result) = memo.get(&num) {
            trace.add_step(TraceStep {
                step: trace.steps.len(),
                operation: "memo_lookup".to_string(),
                input: num,
                output: result,
                depth,
            });
            return result;
        }

        let result = if num == 0 {
            0
        } else if num == 1 {
            1
        } else {
            fib_memo(num - 1, memo, trace, depth + 1) + fib_memo(num - 2, memo, trace, depth + 1)
        };

        memo.insert(num, result);
        trace.add_step(TraceStep {
            step: trace.steps.len(),
            operation: "fib_compute".to_string(),
            input: num,
            output: result,
            depth,
        });

        result
    }

    let result = fib_memo(n, &mut memo, &mut trace, 0);
    trace.set_output("result", result);

    (result, trace)
}

/// Simple hash-based computation for testing
pub fn hash_computation_with_trace(input: u64) -> (String, ProofTrace) {
    let mut trace = ProofTrace::new();
    trace.set_input("input", input);

    use crate::crypto::hash_string;

    let input_str = format!("{:x}", input);
    let mut current = input_str.clone();

    for i in 0..5 {
        let output = hash_string(&current);
        trace.add_step(TraceStep {
            step: i,
            operation: format!("hash_round_{}", i),
            input: input,
            output: (output.len() as u64), // Store output length as u64
            depth: i,
        });
        current = output;
    }

    (current, trace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let (result, _trace) = fibonacci_with_trace(10);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fibonacci_trace() {
        let (_result, trace) = fibonacci_with_trace(5);
        assert!(!trace.steps.is_empty());
        assert_eq!(*trace.outputs.get("result").unwrap(), 5);
    }

    #[test]
    fn test_hash_computation() {
        let (hash, _trace) = hash_computation_with_trace(42);
        assert!(!hash.is_empty());
    }
}
