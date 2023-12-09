use num::integer::lcm;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let rules: Vec<char> = lines.next().unwrap().chars().collect();
    let mut current = Vec::new();
    let mut node_map:[Option<Node>; 17578] =[None;17578];

    lines.map(|line| {
        line.split(" =")
    }).for_each(|mut pair| {
        let key = pair.next();
        let paths_pair = pair.next();
        if let Some(key) = key {
            if let Some(paths_pair) = paths_pair {
                let key_map = v_from_string(key);
                if key == "AAA" {
                    current.push(key_map);
                }

                let mut split = paths_pair.split(", ");
                let hand_l = split.next().unwrap().replace(" (", "");
                let hand_r = split.next().unwrap().replace(')', "");
                if hand_l.ends_with('Z') && hand_r.ends_with('Z'){
                    node_map[key_map] = Some(Node{
                        left: 0,
                        right: 0,
                    })
                } else if hand_r.ends_with('Z'){
                    node_map[key_map] = Some(Node{
                        left: v_from_string(hand_l.as_str()),
                        right: 0,
                    })
                }else if hand_l.ends_with('Z'){
                    node_map[key_map] = Some(Node{
                        left: 0,
                        right: v_from_string(hand_r.as_str()),
                    })
                } else {
                    node_map[key_map] = Some(Node{
                        left: v_from_string(hand_l.as_str()),
                        right: v_from_string(hand_r.as_str()),
                    })
                }
            }
        }
    });
    let first = get_nr_of_steps_v2(&rules, &node_map, current[0]);
    Some(current.iter().skip(1).fold(first,|r, start| {
        lcm(r, get_nr_of_steps_v2(&rules, &node_map, *start))
    }))
}

#[derive(Copy, Clone)]
struct Node {
    right: usize,
    left: usize,
}


fn get_nr_of_steps_v2(rules: &[char],  nodes: &[Option<Node>; 17578], start: usize) -> u64 {
    let mut current = start;
    let mut steps = 0;
    let mut rules = rules.iter().cycle();
    loop {
        let current_rule = *rules.next().unwrap();
        steps += 1;

        let pairs = nodes[current].unwrap();

        current = if current_rule == 'L' {
            pairs.left
        } else {
            pairs.right
        };

        if current == 0 {
            break steps as u64
        }
    }
}
fn v_from_string(input: &str)-> usize {
    input.chars().fold(1,|a,c| {
        if c.is_numeric(){
            let dig = c.to_digit(10).unwrap() as usize;
            return a*dig
        }
        a*(c as usize-64)
    }) + 1
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let rules: Vec<char> = lines.next().unwrap().chars().collect();
    let mut current = Vec::new();
    let mut node_map:[Option<Node>; 17578] =[None;17578];

    lines.map(|line| {
        line.split(" =")
    }).for_each(|mut pair| {
        let key = pair.next();
        let paths_pair = pair.next();
        if let Some(key) = key {
            if let Some(paths_pair) = paths_pair {
                let key_map = v_from_string(key);
                if key.ends_with('A') {
                    current.push(key_map);
                }

                let mut split = paths_pair.split(", ");
                let hand_l = split.next().unwrap().replace(" (", "");
                let hand_r = split.next().unwrap().replace(')', "");
                if hand_l.ends_with('Z') && hand_r.ends_with('Z'){
                    node_map[key_map] = Some(Node{
                        left: 0,
                        right: 0,
                    })
                } else if hand_r.ends_with('Z'){
                    node_map[key_map] = Some(Node{
                        left: v_from_string(hand_l.as_str()),
                        right: 0,
                    })
                }else if hand_l.ends_with('Z'){
                    node_map[key_map] = Some(Node{
                        left: 0,
                        right: v_from_string(hand_r.as_str()),
                    })
                } else {
                    node_map[key_map] = Some(Node{
                        left: v_from_string(hand_l.as_str()),
                        right: v_from_string(hand_r.as_str()),
                    })
                }
            }
        }
    });
    let first = get_nr_of_steps_v2(&rules, &node_map, current[0]);
    Some(current.iter().skip(1).fold(first,|r, start| {
        lcm(r, get_nr_of_steps_v2(&rules, &node_map, *start))
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
