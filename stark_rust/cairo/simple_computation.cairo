// cairo/simple_computation.cairo - Simple computation example

// Simple arithmetic computation
func compute_sum(a: felt, b: felt, c: felt) -> (result: felt) {
    let sum = a + b + c;
    return (result=sum);
}

// Simple multiplication
func compute_product(a: felt, b: felt) -> (result: felt) {
    let product = a * b;
    return (result=product);
}

// Nested computation
func nested_computation(x: felt) -> (result: felt) {
    let (sum_result) = compute_sum(x, x, x);
    let (product_result) = compute_product(sum_result, 2);
    
    return (result=product_result);
}

func main() {
    // Test compute_sum: 1 + 2 + 3 = 6
    let (result1) = compute_sum(1, 2, 3);
    assert result1 = 6;
    
    // Test compute_product: 4 * 5 = 20
    let (result2) = compute_product(4, 5);
    assert result2 = 20;
    
    // Test nested_computation: (10 + 10 + 10) * 2 = 60
    let (result3) = nested_computation(10);
    assert result3 = 60;
    
    return ();
}
