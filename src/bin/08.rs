use std::collections::HashMap;
use num::integer::lcm;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let rules : Vec<char> = lines.next().unwrap().chars().collect();
    let mut current = "AAA";

    let mut nodes = HashMap::new();
    lines.map(|line| {
        line.split(" =")
    }).for_each(|mut pair|{
        let key = pair.next();
        let paths_pair = pair.next();
        if let Some(key) = key {
            if let Some(paths_pair) = paths_pair {
                let mut split = paths_pair.split(", ");
                nodes.insert(key, [split.next().unwrap().replace(" (", ""), split.next().unwrap().replace(")", "")]);
            }
        }
    });

    let mut steps = 0;
    Some(loop {
        let current_rule = rules.get(steps % rules.len()).unwrap();
        steps += 1;

        let pairs = nodes.get(current).unwrap();

        current = if *current_rule == 'L' {
            pairs.first()
        } else {
            pairs.last()
        }.unwrap();

        if current == "ZZZ" {
            break steps as u32
        }
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let rules: Vec<char> = lines.next().unwrap().chars().collect();
    let mut current = Vec::new();
    let mut nr_to_z = Vec::new();
    let mut reached_z = Vec::new();

    let mut nodes = HashMap::new();
    lines.map(|line| {
        line.split(" =")
    }).for_each(|mut pair| {
        let key = pair.next();
        let paths_pair = pair.next();
        if let Some(key) = key {
            if let Some(paths_pair) = paths_pair {
                if key.ends_with('A') {
                    current.push(key);
                }

                let mut split = paths_pair.split(", ");
                let hand_l = split.next().unwrap().replace(" (", "");
                let hand_r = split.next().unwrap().replace(')', "");
                nodes.insert(key, [hand_l, hand_r]);
            }
        }
    });
    current.iter().for_each(|_| {
        nr_to_z.push(0);
        reached_z.push(false);
    });


    let mut steps = 0;
    let mut done = false;
    while !done {
        for i in 0..reached_z.len() {
            if current[i].ends_with('Z') {
                reached_z[i] = true;
                done = reached_z.iter().all(|r| *r);
            }
        }

        let current_rule = *rules.get(steps % rules.len()).unwrap();
        steps += 1;

        for i in 0..reached_z.len() {
            if !reached_z[i] {
                nr_to_z[i] += 1
            }
        }
        for i in 0..current.len() {
            let pairs = nodes.get(current[i]).unwrap();
            current[i] = if current_rule == 'L' {
                pairs.first()
            } else {
                pairs.last()
            }.unwrap()
        }
    };

    Some(nr_to_z.iter().skip(1).fold(nr_to_z[0], |r, s| {
        lcm(r, *s)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result1 = part_one(&advent_of_code::template::read_file("examples", DAY));
        let result2 = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result1, Some(6));
        assert_eq!(result2, Some(2))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(6));
    }
}
