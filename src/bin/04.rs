advent_of_code::solution!(4);

pub fn part_one(options: &str) -> Option<u32> {
    let mut index = 0;
    let mut res = 0;
    let mut winning = [0; 10];
    let mut numbers_matched = 0;
    let mut winning_phase = false;
    let mut my_phase = false;
    let mut space_space = false;
    let mut char1 = -1;
    let mut char2 = -1;
    Some(options.chars().for_each(|c| {
        if c == '\n' {
            let my_nr = char1 * 10 + char2;
            if winning.contains(&my_nr) {
                numbers_matched += 1;
            };
            if numbers_matched == 0 {
                res += numbers_matched;
            } else {
                res += 2u32.pow(numbers_matched - 1);
            }

            index = 0;
            numbers_matched = 0;
            winning_phase = false;
            my_phase = false;
            space_space = false;
            char1 = -1;
            char2 = -1;
        } else {
            if char2 != -1 {
                if winning_phase {
                    let my_nr = char1 * 10 + char2;
                    winning[index] = my_nr;
                    index += 1;
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
        }
    }));
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_index = 0;
    let mut index = 0;
    let mut res = 0;
    let mut copies = [0; 201];
    let mut winning = [0; 10];
    let mut numbers_matched = 0;
    let mut winning_phase = false;
    let mut my_phase = false;
    let mut space_space = false;
    let mut char1 = -1;
    let mut char2 = -1;
    input.chars().for_each(|c| {
        if c == '\n' {
            let my_nr = char1 * 10 + char2;
            if winning.contains(&my_nr) {
                numbers_matched += 1;
            };
            let copies_at_index_plus_one = copies[card_index] + 1;
            card_index += 1;
            for i in card_index..(card_index + numbers_matched) {
                copies[i] = copies[i] + copies_at_index_plus_one;
            }
            if char1 != -1 {
                res += copies_at_index_plus_one;
            }

            index = 0;
            numbers_matched = 0;
            winning_phase = false;
            my_phase = false;
            space_space = false;
            char1 = -1;
            char2 = -1;
        } else {
            if char2 != -1 {
                if winning_phase {
                    let my_nr = char1 * 10 + char2;
                    winning[index] = my_nr;
                    index += 1;
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
        }
    });

    return Some(res);
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
        assert_eq!(result, Some(30));
    }
}
