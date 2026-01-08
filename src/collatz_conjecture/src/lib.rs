pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => {
            let mut steps = 0;
            let mut m = n;
            while m > 1 {
                m = match m % 2 == 0 {
                    true => m / 2,
                    _ => m * 3 + 1,
                };
                steps += 1;
            }
            Some(steps)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_steps_for_one() {
        let output = collatz(1);
        let expected = Some(0);
        assert_eq!(output, expected);
    }

    #[test]
    fn divide_if_even() {
        let output = collatz(16);
        let expected = Some(4);
        assert_eq!(output, expected);
    }

    #[test]
    fn even_and_odd_steps() {
        let output = collatz(12);
        let expected = Some(9);
        assert_eq!(output, expected);
    }

    #[test]
    fn large_number_of_even_and_odd_steps() {
        let output = collatz(1_000_000);
        let expected = Some(152);
        assert_eq!(output, expected);
    }

    #[test]
    fn zero_is_an_error() {
        let output = collatz(0);
        let expected = None;
        assert_eq!(output, expected);
    }
}
