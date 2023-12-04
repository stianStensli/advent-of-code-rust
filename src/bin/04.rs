use std::collections::HashSet;
advent_of_code::solution!(4);

pub fn part_one(options: &str) -> Option<u32> {
    let mut winning = HashSet::new();
    Some(options.lines().map(|line| {
        winning.clear();
        let mut numbers_matched = 0;
        let mut winning_phase = false;
        let mut my_phase = false;
        let mut space_space = false;
        let mut char1 = -1;
        let mut char2 = -1;
        line.chars().for_each(|c| {
            if char1 != -1 && char2 != -1 {
                if winning_phase {
                    let my_nr = char1 * 10 + char2;
                    winning.insert(my_nr);
                } else if my_phase {
                    let my_nr = char1 * 10 + char2;
                    if winning.contains(&my_nr) {
                        numbers_matched += 1;
                    };
                };
                char1 = -1;
                char2 = -1;
                space_space = false;
            };
            if c == '|' {
                winning_phase = false;
                my_phase = true;
                space_space = false;
            } else if winning_phase | my_phase {
                if c == ' ' {
                    if space_space {
                        char1 = 0;
                    }
                    space_space = true;
                } else if (winning_phase | my_phase) & c.is_numeric() {
                    if char1 == -1 {
                        char1 = c.to_digit(10).unwrap() as i32;
                    } else {
                        char2 = c.to_digit(10).unwrap() as i32;
                    }
                };
            } else if c == ':' {
                winning_phase = true;
                space_space = false;
            }
        });
        let my_nr = char1 * 10 + char2;
        if winning.contains(&my_nr) {
            numbers_matched += 1;
        };
        if numbers_matched == 0 || numbers_matched == 1 {
            return numbers_matched;
        };
        2u32.pow(numbers_matched-1)
    }).sum())
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }
}
