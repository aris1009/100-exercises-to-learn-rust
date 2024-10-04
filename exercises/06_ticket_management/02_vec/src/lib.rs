// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.

// rustup toolchain install nightly
#![feature(thread_local)]

#[thread_local]
static mut memo: Vec<Option<u32>> = Vec::new();

pub fn fibonacci(n: u32) -> u32 {
    let n = n as usize;

    unsafe {
        if memo.get(0) == None {
            memo.push(Some(0));
            memo.push(Some(1));
        }
        if let Some(Some(result)) = memo.get(n) {
            return *result;
        }
        if memo.len() <= n {
            memo.resize(n + 1, None);
        }

        for i in 2..=n {
            let n1 = (i - 1).try_into().unwrap();
            let n2 = (i - 2).try_into().unwrap();
            memo[i] = Some(fibonacci(n1) + fibonacci(n2));
        }

        memo[n].unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
