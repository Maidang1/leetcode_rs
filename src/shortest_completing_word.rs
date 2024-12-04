use std::collections::HashMap;

pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let mut license_plate_map: HashMap<char, i32> = HashMap::new();
    license_plate
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_alphabetic())
        .for_each(|c| *license_plate_map.entry(c).or_insert(0) += 1);

    words.into_iter()
        .filter(|word| {
            let mut word_map: HashMap<char, i32> = HashMap::new();
            word.to_lowercase()
                .chars()
                .for_each(|c| *word_map.entry(c).or_insert(0) += 1);
            license_plate_map.iter().all(|(&c, &count)| {
                word_map.get(&c).map_or(false, |&word_count| word_count >= count)
            })
        })
        .min_by_key(|word| word.len())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_completing_word() {
        let words = vec!["step".to_string(), "steps".to_string(), "stripe".to_string(), "stepple".to_string()];
        assert_eq!(
            shortest_completing_word("1s3 PSt".to_string(), words),
            "steps".to_string()
        );
    }
}
