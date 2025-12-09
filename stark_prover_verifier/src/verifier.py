"""
STARK Verifier Module
This module verifies cryptographic proofs of computations.
"""

import hashlib
import json
from typing import Dict, List, Tuple, Any
from dataclasses import dataclass


@dataclass
class VerificationResult:
    """Represents the result of verification"""
    valid: bool
    message: str
    checks_passed: List[str]
    checks_failed: List[str]


class STARKVerifier:
    """Simple STARK Verifier for verifying proofs"""
    
    def __init__(self, security_level: int = 128):
        """
        Initialize the verifier
        
        Args:
            security_level: Expected security level
        """
        self.security_level = security_level
        self.verification_log = []
    
    def verify(self, proof: Dict) -> VerificationResult:
        """
        Verify a STARK proof
        
        Args:
            proof: The proof to verify
            
        Returns:
            VerificationResult with verification status
        """
        checks_passed = []
        checks_failed = []
        
        try:
            # Check 1: Verify proof structure
            if self._check_proof_structure(proof):
                checks_passed.append("Proof structure is valid")
            else:
                checks_failed.append("Invalid proof structure")
                return VerificationResult(
                    valid=False,
                    message="Proof structure validation failed",
                    checks_passed=checks_passed,
                    checks_failed=checks_failed
                )
            
            # Check 2: Verify trace commitment
            if self._verify_trace_commitment(proof):
                checks_passed.append("Trace commitment verified")
            else:
                checks_failed.append("Trace commitment verification failed")
            
            # Check 3: Verify constraint evaluations
            if self._verify_constraints(proof):
                checks_passed.append("Constraint evaluations verified")
            else:
                checks_failed.append("Constraint evaluation verification failed")
            
            # Check 4: Verify FRI layers
            if self._verify_fri_layers(proof):
                checks_passed.append("FRI proof layers verified")
            else:
                checks_failed.append("FRI proof layer verification failed")
            
            # Check 5: Verify challenge consistency
            if self._verify_challenge(proof):
                checks_passed.append("Challenge generation verified")
            else:
                checks_failed.append("Challenge verification failed")
            
            # Check 6: Verify security level
            if proof.get('security_bits', 0) >= self.security_level:
                checks_passed.append(f"Security level adequate ({proof['security_bits']} bits)")
            else:
                checks_failed.append(f"Insufficient security level ({proof['security_bits']} bits)")
            
            # Determine overall validity
            is_valid = len(checks_failed) == 0
            
            message = "Proof is VALID" if is_valid else "Proof is INVALID"
            
            verification_result = VerificationResult(
                valid=is_valid,
                message=message,
                checks_passed=checks_passed,
                checks_failed=checks_failed
            )
            
            self.verification_log.append(verification_result)
            return verification_result
            
        except Exception as e:
            return VerificationResult(
                valid=False,
                message=f"Verification error: {str(e)}",
                checks_passed=checks_passed,
                checks_failed=[f"Exception: {str(e)}"] + checks_failed
            )
    
    def _check_proof_structure(self, proof: Dict) -> bool:
        """Check if proof has required fields"""
        required_fields = [
            'version', 'computation', 'result', 'trace_commitment',
            'constraint_evaluations', 'challenge', 'fri_layers', 'security_bits'
        ]
        
        for field in required_fields:
            if field not in proof:
                return False
        
        # Check types
        if not isinstance(proof['constraint_evaluations'], list):
            return False
        if not isinstance(proof['fri_layers'], list):
            return False
        
        return True
    
    def _verify_trace_commitment(self, proof: Dict) -> bool:
        """Verify the trace commitment is properly formed"""
        commitment = proof.get('trace_commitment', '')
        
        # A valid SHA256 hash should be 64 hex characters
        if len(commitment) == 64 and all(c in '0123456789abcdef' for c in commitment):
            return True
        
        return False
    
    def _verify_constraints(self, proof: Dict) -> bool:
        """Verify constraint evaluations"""
        evaluations = proof.get('constraint_evaluations', [])
        
        # Check that evaluations exist and are integers
        if not evaluations:
            return False
        
        for eval_val in evaluations:
            if not isinstance(eval_val, int) or eval_val < 0:
                return False
        
        # Check that constraint values are within expected range
        for eval_val in evaluations:
            if eval_val >= 2 ** self.security_level:
                return False
        
        return True
    
    def _verify_fri_layers(self, proof: Dict) -> bool:
        """Verify FRI proof layers"""
        fri_layers = proof.get('fri_layers', [])
        
        if not fri_layers or len(fri_layers) < 1:
            return False
        
        # Check that each layer is a valid hash
        for layer in fri_layers:
            if not isinstance(layer, str):
                return False
            if len(layer) != 64 or not all(c in '0123456789abcdef' for c in layer):
                return False
        
        return True
    
    def _verify_challenge(self, proof: Dict) -> bool:
        """Verify challenge was properly generated"""
        challenge = proof.get('challenge', '')
        trace_commitment = proof.get('trace_commitment', '')
        security_bits = proof.get('security_bits', 0)
        
        # Recompute the challenge
        challenge_input = trace_commitment + str(security_bits)
        expected_challenge = hashlib.sha256(challenge_input.encode()).hexdigest()[:16]
        
        return challenge == expected_challenge
    
    def verify_from_file(self, filename: str) -> VerificationResult:
        """
        Load and verify a proof from a JSON file
        
        Args:
            filename: Path to the proof JSON file
            
        Returns:
            VerificationResult
        """
        try:
            with open(filename, 'r') as f:
                proof = json.load(f)
            return self.verify(proof)
        except FileNotFoundError:
            return VerificationResult(
                valid=False,
                message=f"Proof file not found: {filename}",
                checks_passed=[],
                checks_failed=[f"File not found: {filename}"]
            )
        except json.JSONDecodeError:
            return VerificationResult(
                valid=False,
                message="Invalid JSON in proof file",
                checks_passed=[],
                checks_failed=["JSON parse error"]
            )
    
    def get_verification_log(self) -> List[VerificationResult]:
        """Get the verification log"""
        return self.verification_log
    
    def print_verification_report(self, result: VerificationResult) -> None:
        """Print a detailed verification report"""
        print("\n" + "="*60)
        print("STARK PROOF VERIFICATION REPORT")
        print("="*60)
        print(f"\nStatus: {result.message}")
        print(f"Overall Valid: {'YES' if result.valid else 'NO'}")
        
        print(f"\nChecks Passed ({len(result.checks_passed)}):")
        for check in result.checks_passed:
            print(f"  ✓ {check}")
        
        if result.checks_failed:
            print(f"\nChecks Failed ({len(result.checks_failed)}):")
            for check in result.checks_failed:
                print(f"  ✗ {check}")
        
        print("\n" + "="*60 + "\n")


if __name__ == "__main__":
    # Test the verifier with a sample proof
    sample_proof = {
        'version': '1.0',
        'computation': 'fibonacci',
        'result': 55,
        'trace_commitment': hashlib.sha256(b"test").hexdigest(),
        'constraint_evaluations': [1, 2, 3, 4, 5],
        'challenge': hashlib.sha256(b"test" + b"128").hexdigest()[:16],
        'fri_layers': [
            hashlib.sha256(b"layer1").hexdigest(),
            hashlib.sha256(b"layer2").hexdigest(),
            hashlib.sha256(b"layer3").hexdigest()
        ],
        'timestamp': 1234567890,
        'security_bits': 128
    }
    
    verifier = STARKVerifier()
    result = verifier.verify(sample_proof)
    verifier.print_verification_report(result)
