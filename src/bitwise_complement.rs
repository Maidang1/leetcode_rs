pub fn bitwise_complement(n: i32) -> i32 {
    let mut mask = 1;

    while mask < n {
        mask = (mask << 1) + 1;
    }

    n ^ mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_complement() {
        assert_eq!(bitwise_complement(5), 2);
    }
}