advent_of_code::solution!(12);
use cached::proc_macro::cached;

#[cached]
fn place_first_group(input: Vec<Vec<bool>>, nr_groups: Vec<u32>) -> u64 {
    if input.is_empty() {
        if nr_groups.is_empty() {
            return 1;
        }
        return 0;
    }
    if nr_groups.is_empty() {
        if input.iter().all(|v| v.iter().all(|w| !*w)) {
            return 1;
        }
        return 0;
    }
    let place_group = input.first().unwrap().clone();
    let mut nr_of_springs = 0;

    let mut res = 0;
    let next_nr_group = *nr_groups.first().unwrap();
    // try place
    for i in 0..place_group.len() {
        if !place_group[i] {
            // hvis nr_of_springs er mer enn neste gruppe: ##x 1
            if nr_of_springs > next_nr_group {
                // f.eks (##?), (1)
                return 0;
            }

            // f.eks (#??), (1) eller (???), (1)
            if nr_of_springs == 0 || nr_of_springs == next_nr_group{
                let mut next_groups: Vec<Vec<bool>> = Vec::new();
                let mut next_nr_groups = nr_groups.clone();

                // hvis nr_of_springs er mer enn null: #x 1 "fjern neste gruppe"
                if nr_of_springs == next_nr_group {
                    next_nr_groups.remove(0);
                }

                for j in 1..input.len() {
                    next_groups.push(input[j].clone())
                }
                if i == place_group.len() - 1 {
                    // f.eks (#?), (1)
                    res += place_first_group(next_groups, next_nr_groups);
                } else {
                    // f.eks (??), (1)
                    let mut next_prob = Vec::new();
                    for j in i + 1..place_group.len() {
                        next_prob.push(place_group[j])
                    }
                    next_groups.insert(0, next_prob);
                    res += place_first_group(next_groups, next_nr_groups)
                }
            }
            // place group...
            nr_of_springs += 1;
            if nr_of_springs > next_nr_group {
                return res
            }

            let mut next_groups: Vec<Vec<bool>> = Vec::new();
            let mut next_nr_groups = nr_groups.clone();
            for j in 1..input.len() {
                next_groups.push(input[j].clone())
            }
            if i == place_group.len() - 1 {
                if nr_of_springs != next_nr_group {
                    // f.eks (#?), (3)
                    return res
                }
                //happy placed!
                // f.eks (##?), (3)
                next_nr_groups.remove(0);
                res += place_first_group(next_groups, next_nr_groups);
                return res
            } else {
                // f.eks (#???), (3)
                let mut next_prob = Vec::new();
                for j in 0..place_group.len() {
                    if j == i {
                        next_prob.push(true);
                    } else {
                        next_prob.push(place_group[j]);
                    }
                }
                next_groups.insert(0, next_prob);
                res += place_first_group(next_groups, next_nr_groups);

                return res
            }
        } else {
            nr_of_springs += 1;
        }
    }
    // f.eks (###), (3)
    if nr_of_springs == next_nr_group {
        let mut next_nr_groups = nr_groups.clone();
        next_nr_groups.remove(0);
        let mut next_groups: Vec<Vec<bool>> = Vec::new();
        for j in 1..input.len() {
            next_groups.push(input[j].clone())
        }
        res += place_first_group(next_groups, next_nr_groups)
    }
    res
}

fn main_stuff(input: &str) -> u64 {
    let mut input = input.split(' ');
    let problem_str: &str = input.next().unwrap();
    let groups: Vec<Vec<bool>> = problem_str
        .split('.')
        .filter(|s| !s.is_empty())
        .map(|str| str.chars()
            .map(|c| c == '#')
            .collect()
        ).collect();

    let nr_groups: Vec<u32> = input.next().unwrap().split(',').map(|e| e.parse::<u32>().unwrap()).collect();

    place_first_group(groups,nr_groups)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(main_stuff).sum())
}

fn fold_5(input: &str) -> String {
    let mut r = input.split(' ');
    let prob = r.next().unwrap();
    let digs = r.next().unwrap();
    let mut new_prob = prob.to_string();
    let mut new_digs = digs.to_string();
    for _ in 0..4 {
        new_prob.push('?');
        new_digs.push(',');
        new_prob.push_str(prob);
        new_digs.push_str(digs);
    }
    new_prob.push(' ');
    new_prob.push_str(new_digs.as_str());

    new_prob
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(fold_5).map(|s| main_stuff(s.as_str())).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
