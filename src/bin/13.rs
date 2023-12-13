advent_of_code::solution!(13);

pub fn mirror_index(prob: &Vec<Vec<bool>>) -> u32 {
    let mut is_col_not_mirror: Vec<bool> = Vec::new();

    prob[0].iter().skip(1).for_each(|_| is_col_not_mirror.push(false));
    for i in 0..prob.len() {
        for j in 0..prob[i].len() - 1 {
            let mut mirr_index = j + 1;
            let mut mirr_rev_index = j;
            let mut is_not_mirror = false;
            if !is_col_not_mirror[j] {
                while mirr_index < prob[i].len() {
                    if prob[i][mirr_index] != prob[i][mirr_rev_index] {
                        is_not_mirror = true;
                        break;
                    }
                    if mirr_rev_index == 0 {
                        break;
                    }
                    mirr_index += 1;
                    mirr_rev_index -= 1;
                }
            }
            if is_not_mirror {
                is_col_not_mirror[j] = true;
            }
        }
    }
    for i in 0..is_col_not_mirror.len() {
        if !is_col_not_mirror[i] {
            return (i + 1) as u32;
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut prob: Vec<Vec<bool>> = Vec::new();
    let mut prob_flip: Vec<Vec<bool>> = Vec::new();
    let mut sum = 0;
    input.lines().for_each(|l| {
        if l.is_empty() && !prob.is_empty() {
            let m = mirror_index(&prob);
            if m > 0 {
                sum += m;
            } else {
                sum += 100 * mirror_index(&prob_flip);
            }
            prob.clear();
            prob_flip.clear()
        } else {
            let mut p_r = Vec::new();
            l.chars().for_each(|c| {
                if prob_flip.len() == p_r.len() {
                    prob_flip.push(Vec::new());
                }
                prob_flip[p_r.len()].push(c == '.');
                p_r.push(c == '.');
            });
            prob.push(p_r);
        }
    });
    let m = mirror_index(&prob);
    if m > 0 {
        sum += m;
    }else {
        sum += 100 * mirror_index(&prob_flip);
    }
    Some(sum)
}


pub fn mirror_index_smudge(prob: &Vec<Vec<bool>>) -> u32 {
    let mut smudge_mirror: Vec<u32> = Vec::new();

    prob[0].iter().skip(1).for_each(|_| smudge_mirror.push(0));
    for i in 0..prob.len() {
        for j in 0..prob[i].len() - 1 {
            let mut mirr_index = j + 1;
            let mut mirr_rev_index = j;
            if !smudge_mirror[j] > 1 {
                while mirr_index < prob[i].len() && smudge_mirror[j] <= 1{
                    if prob[i][mirr_index] != prob[i][mirr_rev_index] {
                        smudge_mirror[j] += 1;
                        break;
                    }
                    if mirr_rev_index == 0 {
                        break;
                    }
                    mirr_index += 1;
                    mirr_rev_index -= 1;
                }
            }
        }
    }
    //println!("smudge: {:?}",smudge_mirror);
    for i in 0..smudge_mirror.len() {
        if smudge_mirror[i] == 1 {
            return (i + 1) as u32;
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut prob: Vec<Vec<bool>> = Vec::new();
    let mut prob_flip: Vec<Vec<bool>> = Vec::new();
    let mut sum = 0;
    input.lines().for_each(|l| {
        if l.is_empty() && !prob.is_empty() {
            let m = mirror_index_smudge(&prob);
            if m > 0 {
                sum += m;
            } else {
                sum += 100 * mirror_index_smudge(&prob_flip);
            }
            prob.clear();
            prob_flip.clear()
        } else {
            let mut p_r = Vec::new();
            l.chars().for_each(|c| {
                if prob_flip.len() == p_r.len() {
                    prob_flip.push(Vec::new());
                }
                prob_flip[p_r.len()].push(c == '.');
                p_r.push(c == '.');
            });
            prob.push(p_r);
        }
    });
    let m = mirror_index_smudge(&prob);
    if m > 0 {
        sum += m;
    }else {
        sum += 100 * mirror_index_smudge(&prob_flip);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
