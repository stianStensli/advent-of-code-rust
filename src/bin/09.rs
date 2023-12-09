advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.lines().map(|line| {
        let mut sum = 0;
        let mut numbers: Vec<i32> = line.split_ascii_whitespace()
            .map(|n| n.parse().unwrap()).collect();
        loop {
            let mut derev = Vec::new();
            let mut current: i32 = *numbers.first().unwrap();
            sum += *numbers.last().unwrap();

            numbers.iter().skip(1).for_each(|numb| {
                let next: i32 = *numb;
                derev.push(next - current);
                current = next;
            });
            if derev.iter().all(|d| *d == 0) || derev.len() == 1{
                break;
            }
            numbers = derev;
        }
        sum
    }).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
        Some(input.lines().map(|line| {
            let mut numbers: Vec<i32> = line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap()).collect();
            let mut forst: Vec<i32> = Vec::new();
            loop {
                let mut derev = Vec::new();
                let mut current: i32 = *numbers.first().unwrap();
                forst.insert(0,current);
                //println!("first: {}", current);

                numbers.iter().skip(1).for_each(|numb| {
                    let next: i32 = *numb;
                    //println!("curr: {}, next: {}", current, next);
                    derev.push(next - current);
                    current = next;
                });
                if derev.iter().all(|d| *d == 0) || derev.len() == 1{
                    break;
                }
                numbers = derev;
            }
            forst.iter().fold(0,|acc,v| {
                //println!("v {}, acc: {}res {}", v,acc,v-acc);
                v - acc
            })
        }).sum())
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
