use regex::Regex;
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
            first * 10 + last
        })
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let numb_regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").expect("Invalid regex");
    let numb_regex_rev = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").expect("Invalid regex");
    let number_as_string_list: Vec<&str> = Vec::from(["one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", ]);

    let number_as_string_list_rev = Vec::from(["eno","owt","eerht","ruof","evif","xis","neves","thgie","enin"]);
    Some(input
        .lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;
            for (i, [_]) in numb_regex.captures_iter(line)
                .map(|l| { l.extract() })
            {
                first = if i.len() == 1 {
                    i.chars().last().unwrap().to_digit(10).unwrap()
                } else {
                    number_as_string_list.iter().position(|s| i == *s).unwrap() as u32 + 1
                };
                break;
            };
            let rev_line = line.chars().rev().collect::<String>();
            for (i, [_]) in numb_regex_rev.captures_iter(rev_line.as_str())
                .map(|l| { l.extract() })
            {
                last = if i.len() == 1 {
                    i.chars().last().unwrap().to_digit(10).unwrap()
                } else {
                    number_as_string_list_rev.iter().position(|s| i == *s).unwrap() as u32 + 1
                };
                break;
            };
            first*10+last
        })
        .sum())
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
