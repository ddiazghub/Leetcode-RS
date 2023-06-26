/// Fibonacci numbers.
pub fn fib(n: i32) -> i32 {
    let mut memory = [0, 1];

    if n < 2 {
        return memory[n as usize];
    }

    for _ in 2..=n {
        let current = memory[0] + memory[1];
        memory = [memory[1], current];
    }

    return memory[1];
}
