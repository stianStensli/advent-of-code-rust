advent_of_code::solution!(15);
/*
#[derive(Copy, Clone)]
struct Lens {
    focal_length: u64,
    label: String
}*/

fn hash_fn_old(input: Vec<char>) -> u64 {
    input.iter().fold(0, |current_value, c| {
        let mut v = current_value;
        v += *c as u64;
        v *= 17;
        v %= 256;
        v
    })
}

fn hash_fn(input: &[char]) -> usize {
    input.iter().fold(0, |current_value, c| {
        let mut v = current_value;
        v += *c as usize;
        v *= 17;
        v %= 256;
        v
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().next().unwrap()
        .split(',')
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(hash_fn_old)
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut box_labels: Vec<Vec<String>> = vec![Vec::new(); 255];
    let mut box_lengths: Vec<Vec<u64>> = vec![Vec::new(); 255];
    let inputs: Vec<Vec<char>> = input.lines().next().unwrap()
        .split(',')
        .map(|s| s.chars()
            .collect::<Vec<char>>()).collect();
    inputs.iter().for_each(|prob| {
        let prob_index = prob.iter().position(|r| r == &'-' || r == &'=').unwrap();
        let label = &prob[0..prob_index];
        let curr_box = hash_fn(label);
        let label: String = label.iter().collect();
        let operation = &prob[prob_index];
        let labels = box_labels.get_mut(curr_box).unwrap();
        let lengths = box_lengths.get_mut(curr_box).unwrap();
        if operation == &'-' {
            for i in 0..labels.len() {
                if labels[i] == label {
                    labels.remove(i);
                    lengths.remove(i);
                    break;
                }
            }
        } else if operation == &'=' {
            let focal_length = prob[prob_index + 1].to_digit(10).unwrap() as u64;
            if labels.is_empty() {
                labels.push(label);
                lengths.push(focal_length);
            } else {
                match labels.iter_mut()
                    .position(|l| *l == label) {
                    None => {
                        labels.push(label);
                        lengths.push(focal_length);
                    }
                    Some(i) => {
                        labels.remove(i);
                        lengths.remove(i);
                        labels.insert(i, label);
                        lengths.insert(i, focal_length);
                    }
                }
            }
        } else {
            panic!("operation unkown: {:?}", prob)
        }
    });

    let mut index = 0;
    let mut res = 0;
    box_lengths.iter().for_each(|l| {
        index += 1;
        if l.is_empty() {
            return;
        }
        let mut slot = 1;
        l.iter().for_each(|lo| {
            res += lo * slot * index;
            slot += 1;
        })
    });

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(510388));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
