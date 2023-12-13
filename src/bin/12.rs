advent_of_code::solution!(12);


fn place_first(input: Option<Vec<char>>, nr_groups: &Vec<u32>) -> u32 {
    if input.is_none() {
        return 0;
    }
    let input = input.unwrap();
    //let input_clone: String = input.clone().iter().collect();
    if !input.contains(&'?') {
        if is_possible(input, nr_groups) {
            //  println!("possible: {}", input_clone);
            return 1;
        }
        //println!("not possible: {}", input_clone);
        return 0;
    }
    place_first(next_move_good(input.clone(), nr_groups, true), nr_groups) +
        place_first(next_move_good(input, nr_groups, false), nr_groups)
}

// ????????.##.?.?##..??? 1,2,3
// s:(8,0),s:(2,2), s:(1,0), s:(3,2), s:(3,0)


fn is_possible(problem: Vec<char>, nr_groups: &Vec<u32>) -> bool {
    let mut g_index = 0;
    let mut numb_gear = 0;

    //let input_clone: String = problem.clone().iter().collect();
    //println!("prob: {}", input_clone);
    for i in 0..problem.len() {
        if problem[i] == '.' {
            if numb_gear > 0 {
                //println!("Sceck: i {}, g_i: {}, gears: {}", i, g_index, numb_gear);
                if g_index >= nr_groups.len() {
                    return false;
                }
                if nr_groups[g_index] != numb_gear {
                    //println!("r: {:?}", nr_groups);
                    // println!("wrong: {},{}, {}",g_index,nr_groups[g_index],numb_gear);
                    return false;
                }
                g_index += 1;
            }
            numb_gear = 0;
        } else {
            numb_gear += 1;
        }
    }
    if numb_gear > 0 {
        if g_index >= nr_groups.len() {
            return false;
        }
        if nr_groups[g_index] != numb_gear {
            return false;
        }
        g_index += 1;
    }
    if g_index != nr_groups.len() {
        return false;
    }
    true
}

fn next_move_good(problem: Vec<char>, nr_groups: &Vec<u32>, place: bool) -> Option<Vec<char>> {
    let mut chars: Vec<char> = problem;
    let mut index = 0;
    let mut nr_of_gears = 0;
    let mut is_pre_init = true;

    for i in 0..chars.len() {
        let c = chars[i];
        if c == '#' {
            nr_of_gears += 1;
            is_pre_init = false;
        } else if c == '.' && nr_of_gears > 0 {
            index += 1;
            nr_of_gears = 0;
        } else if c == '?' {
            return if place {
                if index < nr_groups.len() && nr_of_gears < nr_groups[index] {
                    chars[i] = '#';
                    Some(chars)
                } else {
                    None
                }
            } else {

                if  index < nr_groups.len() {
                    let mut rest_sum = nr_groups[index];
                    for j in index + 1..nr_groups.len() {
                        rest_sum += nr_groups[j];
                    }
                    if rest_sum < nr_of_gears {
                        return None

                    }
                    if rest_sum - nr_of_gears > (chars.len() - i -1) as u32{
                        //let p: String = chars.iter().collect();
                        //println!("prob: {}, sum: {}", p, chars.len() - i -1);
                        //println!("rest_sum: {}, i: {}, c: {}", rest_sum-nr_of_gears, i, chars.len());
                        return None
                    }
                }
                if is_pre_init ||
                    (i > 0 && chars[i - 1] == '.') ||
                    (index < nr_groups.len() && nr_of_gears == nr_groups[index]) {
                    chars[i] = '.';
                    Some(chars)
                } else {
                    //chars[i] = '.';
                    //let r: String = chars.iter().collect();
                    //println!("Nope: {}", r);
                    None
                }
            };
        }
    }
    None
}


fn main_stuff(input: &str) -> u32 {
    //println!("Main: {}", input);
    let mut input = input.split(' ');
    let problem: Vec<char> = input.next().unwrap().chars().collect();
    let nr_groups: Vec<u32> = input.next().unwrap().split(',').map(|e| e.parse::<u32>().unwrap()).collect();
    let res = place_first(Some(problem), &nr_groups);
    //println!("res: {}", res);
    res
}

pub fn part_one(input: &str) -> Option<u32> {
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
    //Some(input.lines().map(fold_5).map(|s| main_stuff(s.as_str())).map(|a| a as u64).sum())
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
        //assert_eq!(None, None);
    }
}
