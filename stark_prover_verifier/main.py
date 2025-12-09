#!/usr/bin/env python3
"""
STARK Prover and Verifier - Main Executable
A complete demonstration of STARK (Scalable Transparent Argument of Knowledge)
proof generation and verification.

Usage:
    python main.py prove <n>              - Generate a proof for fibonacci(n)
    python main.py verify <proof_file>    - Verify a proof from a JSON file
    python main.py demo                   - Run a full demo
"""

import sys
import json
import argparse
from pathlib import Path

# Add src directory to path
sys.path.insert(0, str(Path(__file__).parent / 'src'))

from prover import STARKProver
from verifier import STARKVerifier


def prove_command(n: int) -> None:
    """Generate a STARK proof for fibonacci(n)"""
    print(f"\n{'='*60}")
    print("STARK PROOF GENERATION")
    print(f"{'='*60}\n")
    
    try:
        n = int(n)
        if n < 0:
            print("Error: n must be non-negative")
            return
        
        print(f"Computing Fibonacci({n})...")
        prover = STARKProver(security_level=128)
        
        # Compute with trace
        result, trace = prover.compute_fibonacci(n)
        print(f"✓ Computation completed: fibonacci({n}) = {result}")
        
        print(f"\n✓ Computation trace generated with {len(trace.steps)} steps")
        
        # Generate proof
        print("\nGenerating STARK proof...")
        proof = prover.generate_proof(result, trace)
        print("✓ STARK proof generated successfully")
        
        # Display proof info
        print(f"\nProof Details:")
        print(f"  - Version: {proof['version']}")
        print(f"  - Computation: {proof['computation']}")
        print(f"  - Result: {proof['result']}")
        print(f"  - Security Level: {proof['security_bits']} bits")
        print(f"  - Trace Commitment: {proof['trace_commitment'][:16]}...")
        print(f"  - Challenge: {proof['challenge']}")
        print(f"  - FRI Layers: {len(proof['fri_layers'])}")
        
        # Save proof
        output_file = f"proof_fib_{n}.json"
        prover.save_proof(output_file)
        print(f"\n✓ Proof saved to: {output_file}")
        
    except ValueError as e:
        print(f"Error: Invalid input - {e}")
    except Exception as e:
        print(f"Error during proof generation: {e}")


def verify_command(proof_file: str) -> None:
    """Verify a STARK proof from a JSON file"""
    print(f"\n{'='*60}")
    print("STARK PROOF VERIFICATION")
    print(f"{'='*60}\n")
    
    try:
        if not Path(proof_file).exists():
            print(f"Error: Proof file not found: {proof_file}")
            return
        
        print(f"Loading proof from: {proof_file}")
        with open(proof_file, 'r') as f:
            proof = json.load(f)
        
        print(f"✓ Proof loaded successfully")
        print(f"  - Computation: {proof.get('computation', 'unknown')}")
        print(f"  - Result: {proof.get('result', 'unknown')}")
        
        # Verify
        print("\nVerifying proof...")
        verifier = STARKVerifier(security_level=128)
        result = verifier.verify(proof)
        
        # Print report
        verifier.print_verification_report(result)
        
        # Return exit code based on verification result
        sys.exit(0 if result.valid else 1)
        
    except json.JSONDecodeError:
        print(f"Error: Invalid JSON in proof file")
    except Exception as e:
        print(f"Error during verification: {e}")


def demo_command() -> None:
    """Run a complete demo of the prover and verifier"""
    print(f"\n{'='*60}")
    print("STARK PROVER AND VERIFIER - DEMONSTRATION")
    print(f"{'='*60}\n")
    
    # Demo 1: Fibonacci(10)
    print("DEMO 1: Computing Fibonacci(10)")
    print("-" * 60)
    
    prover = STARKProver(security_level=128)
    n = 10
    
    print(f"Computing fibonacci({n})...")
    result, trace = prover.compute_fibonacci(n)
    print(f"✓ Result: fibonacci({n}) = {result}")
    print(f"✓ Trace steps: {len(trace.steps)}")
    
    print("\nGenerating STARK proof...")
    proof = prover.generate_proof(result, trace)
    print("✓ STARK proof generated")
    
    # Demo 2: Verify the proof
    print("\n" + "="*60)
    print("DEMO 2: Verifying the Proof")
    print("-" * 60)
    
    verifier = STARKVerifier(security_level=128)
    verification_result = verifier.verify(proof)
    verifier.print_verification_report(verification_result)
    
    # Demo 3: Tampering detection
    print("="*60)
    print("DEMO 3: Tampering Detection")
    print("-" * 60)
    
    # Create a tampered proof
    tampered_proof = proof.copy()
    tampered_proof['result'] = result + 1  # Tamper with the result
    
    print("Attempting to verify a tampered proof (result modified)...")
    tampered_result = verifier.verify(tampered_proof)
    print(f"Verification result: {tampered_result.message}")
    
    if tampered_result.valid:
        print("⚠ WARNING: Tampering was NOT detected!")
    else:
        print("✓ Tampering DETECTED - proof is invalid")
    
    # Demo 4: Multiple computations
    print("\n" + "="*60)
    print("DEMO 4: Multiple Computations")
    print("-" * 60)
    
    test_cases = [5, 8, 15]
    all_valid = True
    
    for test_n in test_cases:
        result, trace = prover.compute_fibonacci(test_n)
        proof = prover.generate_proof(result, trace)
        verification = verifier.verify(proof)
        
        status = "✓ VALID" if verification.valid else "✗ INVALID"
        print(f"fibonacci({test_n:2d}) = {result:6d} - Proof: {status}")
        all_valid = all_valid and verification.valid
    
    print("\n" + "="*60)
    if all_valid:
        print("✓ ALL PROOFS VERIFIED SUCCESSFULLY")
    else:
        print("✗ SOME PROOFS FAILED VERIFICATION")
    print("="*60 + "\n")


def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description='STARK Prover and Verifier',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='''
Examples:
  python main.py prove 10          - Generate proof for fibonacci(10)
  python main.py verify proof.json - Verify a proof
  python main.py demo              - Run full demonstration
        '''
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Commands')
    
    # Prove command
    prove_parser = subparsers.add_parser('prove', help='Generate a STARK proof')
    prove_parser.add_argument('n', type=int, help='Fibonacci index to prove')
    
    # Verify command
    verify_parser = subparsers.add_parser('verify', help='Verify a STARK proof')
    verify_parser.add_argument('proof_file', help='Path to proof JSON file')
    
    # Demo command
    subparsers.add_parser('demo', help='Run a complete demonstration')
    
    args = parser.parse_args()
    
    if not args.command:
        parser.print_help()
        sys.exit(1)
    
    if args.command == 'prove':
        prove_command(args.n)
    elif args.command == 'verify':
        verify_command(args.proof_file)
    elif args.command == 'demo':
        demo_command()


if __name__ == '__main__':
    main()
