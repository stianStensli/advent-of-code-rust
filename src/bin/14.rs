advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones_2: Vec<Vec<bool>> = vec![Vec::new(); input.lines().next().unwrap().len()];
    let mut r_index = 0;
    input.lines().for_each(|l| {
        let mut c_index = 0;
        l.chars().for_each(|c| {
            if c == 'O' {
                stones_2[c_index].push(true);
            } else if c == '#' {
                while stones_2[c_index].len() <= r_index {
                    stones_2[c_index].push(false);
                }
            }
            c_index += 1;
        });
        r_index += 1;
    });
    let max_row = r_index as u32;
    r_index = 0;
    Some(stones_2.iter().map(|s| {
        let mut res: u32 = 0;

        r_index = 0;
        s.iter().for_each(|v| {
            if *v {
                res += max_row - r_index as u32;
            }
            r_index += 1;
        });
        res
    }).sum())
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
