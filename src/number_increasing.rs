pub fn are_numbers_ascending(s: String) -> bool {
    let str_arr = s.split(" ");
    str_arr
        .filter(|s| s.parse::<i32>().is_ok())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .all(|w| w[0] < w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_numbers_ascending() {
        assert_eq!(
            are_numbers_ascending(
                "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
            ),
            true
        );
        assert_eq!(
            are_numbers_ascending("hello world 5 x 5".to_string()),
            false
        );
        assert_eq!(
            are_numbers_ascending("1 2 3 4 99 5 6 7 8".to_string()),
            false
        );
        assert_eq!(
            are_numbers_ascending("1 2 3 4 5 6 6 7 8".to_string()),
            false
        );
    }
}