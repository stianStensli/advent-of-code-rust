advent_of_code::solution!(12);


fn place_first(input: &str, nr_groups: &Vec<u32>) -> u32 {
    let mut has_placed_main = false;
    let mut nr_of_possible = 0;

    let mut next_problem = input.chars().map(|c| {
        if !has_placed_main && c == '?' {
            has_placed_main = true;
            '#'
        } else {
            c
        }
    }).collect::<String>();


    while next_problem.contains('?') && !is_definitly_not_possible(next_problem.as_str(), &nr_groups) {
        nr_of_possible += place_first(next_problem.as_str(), nr_groups);
        let mut has_placed = false;
        next_problem = next_problem.chars().map(|c| {
            if !has_placed && c == '?' {
                has_placed = true;
                '.'
            } else {
                c
            }
        }).collect::<String>();
    }

    if is_possible(next_problem.as_str(), nr_groups) {
        nr_of_possible += 1;
    }
    nr_of_possible
}

// ????????.##.?.?##..??? 1,2,3
// s:(8,0),s:(2,2), s:(1,0), s:(3,2), s:(3,0)


#[derive(Debug)]
struct GNode {
    size: u32,
    forced: u32,
}

fn is_possible(problem: &str, nr_groups: &Vec<u32>) -> bool {
    let mut groups: Vec<u32> = Vec::new();
    problem.split('.').for_each(|s| {
        if !s.is_empty() {
            groups.push(s.len() as u32);
        }
    });
    if groups.len() != nr_groups.len() {
        return false;
    }

    groups.iter().zip(nr_groups.iter()).all(|(springs, springs_real)| springs == springs_real)
}

/*

    let groups_size: Vec<u32> = problem.split('.').map(|s| s.len() as u32).collect();
    let groups_forced: Vec<u32> = problem.split(['.', '?']).map(|s| s.len() as u32).filter(|x| x > &0).collect();

    let nr_of_springs: u32 = groups_forced.iter().sum();
    let nr_of_poss: u32 = groups_size.iter().sum();
    let nr_of_springs_real: u32 = nr_groups.iter().sum();
    if nr_of_springs > nr_of_springs_real || nr_of_poss < nr_of_springs_real {
        return true;
    }
 */
fn is_definitly_not_possible(problem: &str, nr_groups: &Vec<u32>) -> bool {
    // #.#.### 1,1,3
    let groups: Vec<GNode> = problem.split('.').map(|s| {
        GNode {
            size: s.len() as u32,
            forced: s.chars().filter(|c| c == &'#').count() as u32,
        }
    }).collect();
    let nr_of_springs: u32 = groups.iter().map(|g| g.forced).sum();
    let nr_of_poss: u32 = groups.iter().map(|g| g.size).sum();
    let nr_of_springs_real: u32 = nr_groups.iter().sum();
    if nr_of_springs > nr_of_springs_real || nr_of_poss < nr_of_springs_real {
        return true;
    }

    /*
    let mut index = 0;
    //groups.iter().map(|g|g.forced).max().unwrap() > *nr_groups.iter().max().unwrap()
    //false
    let r = groups_forced.iter()
        .any(|g| {
            for i in index..nr_groups.len() {
                index = i + 1;
                if *g <= nr_groups[i] {
                    return false;
                }
            }
            if index == nr_groups.len() {
                return true;
            }
            false
        });

    println!("r: {}, prob: {}, nr_groups: {:?}, g: {:?}", r, problem, nr_groups, groups_forced);
    r*/
    false
}


fn main_stuff(input: &str) -> u32 {
    //println!("problem: {}", input);
    let mut input = input.split(' ');
    let problem: &str = input.next().unwrap();
    let nr_groups: Vec<u32> = input.next().unwrap().split(',').map(|e| e.parse::<u32>().unwrap()).collect();

    let mut next_problem: String = problem.to_string();
    let mut nr_of_possible = 0;

    while next_problem.contains('?') && !is_definitly_not_possible(next_problem.as_str(), &nr_groups) {
        nr_of_possible += place_first(next_problem.as_str(), &nr_groups);
        let mut has_placed = false;
        next_problem = next_problem.chars().map(|c| {
            if !has_placed && c == '?' {
                has_placed = true;
                '.'
            } else {
                c
            }
        }).collect::<String>();
    }
    if is_possible(next_problem.as_str(), &nr_groups) {
        //println!("Is possible: {}, {:?}", next_problem, nr_groups);
        nr_of_possible += 1;
    }

    nr_of_possible
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(main_stuff).sum())
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
