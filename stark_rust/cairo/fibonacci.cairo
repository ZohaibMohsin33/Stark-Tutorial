// cairo/fibonacci.cairo - Fibonacci computation in Cairo
// This program computes the nth Fibonacci number using Cairo

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
    // Compute fibonacci(10)
    let (result) = fib(10);
    
    // The result is 55
    assert result = 55;
    
    return ();
}
