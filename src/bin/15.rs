advent_of_code::solution!(15);


fn hash_fn(input: Vec<char>) -> u64{
    input.iter().fold(0, |current_value, c| {
        let mut v = current_value;
        v += *c as u64;
        v *= 17;
        v %= 256;
        v
    })
}


pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().next().unwrap()
        .split(',')
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(hash_fn)
        .sum())
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(510388));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
