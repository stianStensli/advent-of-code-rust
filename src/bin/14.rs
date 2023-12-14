advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let max_row = input.lines().next().unwrap().len();
    let mut stones_index: Vec<u32> = vec![0; max_row];
    let max_row: u32 = max_row as u32;

    let mut r_index = 0;
    let mut res = 0;
    input.lines().for_each(|l| {
        let mut c_index = 0;
        l.chars().for_each(|c| {
            if c == 'O' {
                let old_index = stones_index[c_index];
                res += max_row - old_index;
                stones_index[c_index] += 1;
            } else if c == '#' {
                stones_index[c_index] = r_index + 1;
            }
            c_index += 1;
        });
        r_index += 1;
    });
    Some(res)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
