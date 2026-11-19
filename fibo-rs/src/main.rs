use std::io;

/// Naive Recursion
/// - Time Complexity: O(2^n)
/// - Space Complexity: O(n)
/// - Note: Just to know; Never use this
fn fib_recursive(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

/// Memoization (Top-Down DP)
/// - Time Complexity: O(n)
/// - Space Complexity: O(n)
/// - Note: Good for sparse recursive problems, though recursion adds call-stack overhead
fn fib_memoized(n: u32) -> u128 {
    fn helper(n: usize, memo: &mut Vec<u128>) -> u128 {
        if n <= 1 {
            return n as u128;
        }

        if memo[n] != 0 {
            return memo[n];
        }

        memo[n] = helper(n - 1, memo) + helper(n - 2, memo);
        memo[n]
    }

    let n_usize = n as usize;

    let mut memo = vec![0u128; n_usize + 1];
    helper(n_usize, &mut memo)
}

/// Iterative (Space-Optimized Bottom-Up)
/// - Time Complexity: O(n)
/// - Space Complexity: O(1)
/// - Note: Best general purpose choice for small to medium n when memory should be minimal
fn fib_iterative(n: u32) -> u128 {
    if n <= 1 {
        return n as u128;
    }

    let (mut prev, mut curr) = (0u128, 1u128);
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

/// Tabulation (Bottom-Up DP)
/// - Time Complexity: O(n)
/// - Space Complexity: O(n)
/// - Note: Useful when you need all Fibonacci number from 0..=n for repeated lookups
fn fib_tabulated(n: u32) -> u128 {
    let n_usize = n as usize;
    if n_usize <= 1 {
        return n as u128;
    }

    let mut table = vec![0u128; n_usize + 1];
    table[1] = 1;
    for i in 2..=n_usize {
        table[i] = table[i - 1] + table[i - 2];
    }

    table[n_usize]
}

/// Matrix Exponentiation Optimization
/// - Time Complexity: O(log n):
///   Halves `n` at each step using identities:
///   F(2k) = F(k) * [2*F(k + 1) - F(k)]
///   F(2k + 1) = F(k)^2 + F(k + 1)^2
/// - Space Complexity: O(log n)
/// - Note: Fastest exact algorithm for large n in production and CP
fn fib_fast_doubling(n: u32) -> u128 {
    fn helper(n: u32) -> (u128, u128) {
        if n == 0 {
            return (0, 1);
        }
        let (a, b) = helper(n >> 1);
        let c = a * (2 * b - a);
        let d = a * a + b * b;
        if n & 1 == 0 { (c, d) } else { (d, c + d) }
    }

    helper(n).0
}

/// Golden Ratio / Binet's Formula
/// - Time Complexity: O(1)
/// - Space Complexity: O(1)
/// - Note: Not used in production for exact integers; Loses precision when n > 70
fn fib_golden_ratio(n: u32) -> u128 {
    let sqrt_5 = 5.0_f64.sqrt();
    let phi = (1.0 + sqrt_5) / 2.0;
    (phi.powi(n as i32) / sqrt_5).round() as u128
}

const MAX_N_U128: u32 = 186;
const MAX_N_RECURSIVE: u32 = 45;

fn main() {
    println!("Fibonacci Calculator!");
    println!("\nAvailable algorithms:");
    println!("1. Naive Recursion");
    println!("2. Memoization");
    println!("3. Iterative");
    println!("4. Tabulation");
    println!("5. Fast Doubling");
    println!("6. Golden Ratio (n <= 70)");
    println!("\nType 'quit' or 'exit' to exit.");

    loop {
        println!("\nSelect an algorithm (1..=6) or 'quit'/'exit' to exit:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("quit") || trimmed.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        let algorithm: u32 = match trimmed.parse() {
            Ok(n @ 1..=6) => n,
            _ => {
                println!("Invalid selection. Please choose 1-6.");
                continue;
            }
        };

        let max_allowed = if algorithm == 1 {
            MAX_N_RECURSIVE
        } else {
            MAX_N_U128
        };

        println!("\nPlease input n (0..={max_allowed}):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let n: u32 = match input.trim().parse() {
            Ok(num) if num <= max_allowed => num,
            Ok(num) if num > MAX_N_U128 => {
                println!(
                    "Input too large ({num}). Maximum allowed is {MAX_N_U128} to prevent u128 overflow."
                );
                continue;
            }
            Ok(num) => {
                println!(
                    "Input too large ({num}) for Naive Recursion. Please enter n <= {MAX_N_RECURSIVE}."
                );
                continue;
            }
            Err(_) => {
                println!("Invalid input. Please enter a non-negative integer.");
                continue;
            }
        };

        let result = match algorithm {
            1 => fib_recursive(n),
            2 => fib_memoized(n),
            3 => fib_iterative(n),
            4 => fib_tabulated(n),
            5 => fib_fast_doubling(n),
            6 => fib_golden_ratio(n),
            _ => unreachable!(),
        };

        println!("Fibonacci({n}) = {result}");
    }
}
