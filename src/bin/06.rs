advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut race_times: Vec<u32> = Vec::new();
    let mut index = 0;
    let mut good_times = 0;
    let mut is_time = false;
    input.lines().for_each(|l| {
        let data = l.split_whitespace();
        data.for_each(|d| {
            if d.chars().any(|c| !c.is_numeric()) {
                is_time = !is_time;
            } else if is_time {
                race_times.push(d.parse::<u32>().unwrap())
            } else {
                let time = race_times[index];
                index += 1;
                let l = d.parse::<u32>().unwrap();
                //race_lengths.push(l);
                let mut g_t = 0;
                for i in 1..(time - 1) {
                    let test = (time - i) * i;
                    if test > l {
                        g_t += 1;
                    }
                }
                if good_times == 0 {
                    good_times = g_t
                } else {
                    good_times *= g_t;
                }
            }
        })
    });

    Some(good_times)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut race_times_str: Vec<String> = Vec::new();
    let mut race_lengths_str: Vec<String> = Vec::new();
    let mut race_times = 0;
    input.lines().for_each(|l| {
        let data = l.split_whitespace();
        if !race_times_str.is_empty() {
            let r: String = race_times_str.iter().flat_map(|s| s.chars()).collect();
            race_times = r.parse::<u64>().unwrap();
        }
        data.for_each(|d| {
            if d.chars().any(|c| !c.is_numeric()) {} else if race_times == 0 {
                race_times_str.push(d.to_string());
            } else {
                race_lengths_str.push(d.to_string());
            }
        })
    });
    let r: String = race_lengths_str.iter().flat_map(|s| s.chars()).collect();
    let race_length = r.parse::<u64>().unwrap();

    let v = (race_times.pow(2) as f64 - 4f64 * race_length as f64).sqrt();
    Some((0.5 * (race_times as f64 + v)).ceil() as u64 - (0.5 * (race_times as f64 - v)).ceil() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
