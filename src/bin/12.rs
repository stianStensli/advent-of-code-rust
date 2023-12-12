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


    while next_problem.contains('?') {
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
    //println!("\n");
    let mut groups: Vec<GNode> = Vec::new();
    problem.split('.').for_each(|s| {
        if s.len() > 0 {
            //print!("s: {}", s);
            groups.push(GNode {
                size: s.len() as u32,
                forced: s.chars().filter(|c| c == &'#').count() as u32,
            })
        }
    });
    //println!("\n");
    if groups.len() != nr_groups.len() {
        //println!("Is not possible: {}, g: {:?}, nr_g: {:?}", problem, groups, nr_groups);
        return false;
    }

    let res = groups.iter().zip(nr_groups.iter()).all(|(springs, springs_real)| springs.forced == *springs_real);
    /*
    if res {
        println!("Is possible: {}, g: {:?}, nr_g: {:?}", problem, groups, nr_groups);
    } else {
        println!("Is not possible: {}, g: {:?}, nr_g: {:?}", problem, groups, nr_groups);
    }*/
    res
}

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
    if !problem.contains('?') {
        if groups.len() != nr_groups.len() {
            return true;
        }
        return !groups.iter().zip(nr_groups.iter()).all(|(springs, springs_real)| springs.forced == *springs_real);
    }
    false
}


fn main_stuff(input: &str) -> u32 {
    println!("problem: {}", input);
    let mut input = input.split(' ');
    let problem: &str = input.next().unwrap();
    let nr_groups: Vec<u32> = input.next().unwrap().split(',').map(|e| e.parse::<u32>().unwrap()).collect();

    let mut next_problem: String = problem.to_string();
    let mut nr_of_possible = 0;

    while next_problem.contains('?') {
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
        println!("Is possible: {}, {:?}", next_problem, nr_groups);
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
