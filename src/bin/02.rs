use std::cmp::max;
advent_of_code::solution!(2);

const PEAK_RED: u32 = 12;
const PEAK_GREEN: u32 = 13;
const PEAK_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let mut index: u32 = 0;
    Some(input.lines().map(|game| {
        let rounds = game.split(':').skip(1).last().unwrap().split(';');
        let mut peak_red = 0;
        let mut peak_green = 0;
        let mut peak_blue = 0;
        rounds.for_each(|round| {
            let mut last_number = 0;
            round.split(' ').for_each(|v| {
                let n = v.parse();
                if let Ok(n) = n {
                    last_number = n;
                } else if v.contains("red") {
                    peak_red = max(last_number, peak_red);
                } else if v.contains("green") {
                    peak_green = max(last_number, peak_green);
                } else if v.contains("blue") {
                    peak_blue = max(last_number, peak_blue);
                }
            });
        });
        index += 1;
        if peak_red <= PEAK_RED &&
            peak_green <= PEAK_GREEN &&
            peak_blue <= PEAK_BLUE {
            return index
        }
        0
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
