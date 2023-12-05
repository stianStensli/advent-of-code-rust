advent_of_code::solution!(5);

fn with_seeds(input: &str, mut seeds: Vec<u32>) -> Option<u32> {
    let parts: Vec<&str> = input.split(':').collect();
    for part in 2..parts.len() {
        let mut touched_seed: Vec<bool> = seeds.iter().map(|_| false).collect();
        let txt_map: Vec<&str> = parts.get(part).unwrap()
            .split('\n')
            .filter(|p| {
                !(p.is_empty() || p.contains("map"))
            }).collect();


        for single_map in txt_map {
            let data: Vec<u32> = single_map.split(' ')
                .map(|seed| {
                    seed.parse::<u32>().unwrap()
                }).collect();
            let destination = data.first().unwrap().to_owned();
            let source = data.get(1).unwrap().to_owned();
            let limit = data.get(2).unwrap().to_owned();

            for i in 0..seeds.len() {
                let seed = seeds[i];
                if touched_seed[i] {} else if seed >= source && seed < source + limit {
                    seeds[i] = destination + (seed - source);
                    touched_seed[i] = true;
                }
            }
        }
    }

    seeds.iter().min().to_owned().map(|m| m.to_owned())
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = lines.first()
        .unwrap()
        .replace("seeds: ", "");
    let seeds: Vec<u32> = seeds.split(' ')
        .map(|seed| {
            seed.parse::<u32>().unwrap()
        }).collect();

    with_seeds(input, seeds)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = lines.first()
        .unwrap()
        .replace("seeds: ", "");
    let seeds_range: Vec<u32> = seeds.split(' ')
        .map(|seed| {
            seed.parse::<u32>().unwrap()
        }).collect();

    let mut seeds = Vec::new();
    let mut j = 0;
    while j < seeds_range.len() - 1 {
        for i in seeds_range[j]..seeds_range[j] + seeds_range[j + 1] + 1 {
            seeds.push(i);
        }
        j += 2;
    }
    seeds.sort();
    println!("{}", seeds.len());
    with_seeds(input, seeds)
}


struct SeedControll {
    seeds: Vec<SeedRange>
}

struct SeedRange {
    start: u64,
    end: u64,
}

impl SeedRange {
    fn print(&self) {
        println!("start: {}, end: {} ", self.start, self.end);
    }
}

pub fn part_two_v2(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = lines.first()
        .unwrap()
        .replace("seeds: ", "");


    let seeds_range: Vec<u64> = seeds.split(' ')
        .map(|seed| {
            seed.parse::<u64>().unwrap()
        }).collect();

    let mut seeds: Vec<SeedRange> = Vec::new();
    let mut j = 0;
    while j < seeds_range.len() - 1 {
        seeds.push(SeedRange {
            start: seeds_range[j],
            end: seeds_range[j + 1] + seeds_range[j],
        });
        seeds.last().unwrap().print();
        j += 2;
    }
    println!("{}", seeds.len());
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        part_two_v2(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
