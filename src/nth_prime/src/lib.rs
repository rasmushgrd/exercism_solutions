pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut prime = 2;
    let mut end = 10;
    while count < n {
        println!("{}", prime);

        match (prime..end).filter(|x| x % prime == 0).next() {
            Some(p) => {
                println!("{}", p);
                prime = p;
                count += 1;
            }
            None => {
                end += 10;
            }
        }
    }
    prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_prime() {
        let output = nth(0);
        let expected = 2;
        assert_eq!(output, expected);
    }

    #[test]
    fn second_prime() {
        let output = nth(3);
        let expected = 5;
        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn sixth_prime() {
        let output = nth(5);
        let expected = 13;
        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn big_prime() {
        let output = nth(10_000);
        let expected = 104_743;
        assert_eq!(output, expected);
    }
}
