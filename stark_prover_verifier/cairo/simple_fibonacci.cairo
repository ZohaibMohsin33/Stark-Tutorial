// Simple Fibonacci computation to be proved
// This program computes the nth Fibonacci number

func fib(n) -> (result: felt) {
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
    return ();
}
