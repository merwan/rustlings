use core::iter::Iterator;

fn factorial_recursive(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    match num {
        0 => 1,
        1 => 1,
        _other => num * factorial(num - 1),
    }
}

fn factorial_fold(num: u64) -> u64 {
    #[allow(clippy::unnecessary_fold)]
    (1..=num).fold(1, |acc, i| acc * i)
}

fn factorial_product(num: u64) -> u64 {
    (1..=num).product()
}

fn factorial(num: u64) -> u64 {
    factorial_product(num)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
