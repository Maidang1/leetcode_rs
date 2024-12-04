fn solution(numbers: &[i32]) -> i32 {
    let mut groups = Vec::new();
    for num in numbers.iter() {
        let chats_arr: Vec<i32> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        groups.push(chats_arr);
    }

    fn calc_nums(group: &Vec<Vec<i32>>, index: usize, current_sum: i32, count: &mut i32) {
        if index == group.len() {
            if current_sum % 2 == 0 {
                *count = *count + 1;
            }
            return;
        }
        for &num in group[index].iter() {
            calc_nums(group, index + 1, num + current_sum, count);
        }
    }

    let mut count = 0;
    calc_nums(&mut groups, 0, 0, &mut count);

     println!("count: {}", count);
     0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        solution(&[1, 2, 3, 4, 5]);
    }
}
