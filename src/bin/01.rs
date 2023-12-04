advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines()
        .map(|line| line.chars())
        .map(|chars| {
            let mut isinit = true;
            let mut first = 0;
            let mut last = 0;
            chars.for_each(|char| {
                if char.is_numeric() {
                    if isinit {
                        first = char.to_digit(10).unwrap();
                        isinit = false;
                    }
                    last = char.to_digit(10).unwrap();
                }
            });
            first*10+last
        })
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    part_one(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
