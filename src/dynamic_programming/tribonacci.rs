/// Return the nth tribonacci number
pub fn tribonacci(n: i32) -> i32 {
    let mut memory = [0, 1, 1];

    if n < 3 {
        return memory[n as usize];
    }

    for _ in 3..=n {
        let current = memory[0] + memory[1] + memory[2];
        memory = [memory[1], memory[2], current];
    }

    return memory[2];
}
