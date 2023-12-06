advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
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
    while j < seeds_range.len() {
        seeds.push(SeedRange {
            start: seeds_range[j],
            end: seeds_range[j],
            last_gen_touched: 1,
        });
        j += 1;
    }
    with_seeds_v2(input, seeds)
}

pub fn part_two(input: &str) -> Option<u64> {
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
            end: seeds_range[j + 1] + seeds_range[j] - 1,
            last_gen_touched: 1,
        });
        j += 2;
    }
    with_seeds_v2(input, seeds)
}

fn move_seeds(mut seeds: Vec<SeedRange>, source: u64, limit: u64, gen: u32, destination: u64) -> Vec<SeedRange> {
    let mut to_be_added: Vec<SeedRange> = Vec::new();

    seeds.iter_mut().for_each(|s| {
        if s.last_gen_touched != gen {
            if s.start >= source && s.end < source + limit {
                // hele flyttes 🎉
                s.update_end(destination + s.end - source);
                s.update_start(destination + s.start - source);
                s.update_gen(gen);
            } else if s.start >= source && s.start < source + limit {
                // start flyttes og en ny legges til
                to_be_added.push(SeedRange {
                    last_gen_touched: gen - 1,
                    end: s.end,
                    start: source + limit,
                });
                s.update_end(destination + limit - 1);
                s.update_start(destination + s.start - source);
                s.update_gen(gen);
            } else if s.end >= source && s.end < source + limit {
                // slutt flyttes og en ny legges til
                to_be_added.push(SeedRange {
                    last_gen_touched: gen,
                    end: destination + s.end - source,
                    start: destination,
                });
                s.update_end(source - 1);
            } else if s.start < source && s.end >= source + limit {
                //println!("s.s: {}, s.e: {}, src: {} dest, {}, lim: {}", s.start, s.end, source, destination, limit);
                // en ny i mellom rangene to legges til
                to_be_added.push(SeedRange {
                    last_gen_touched: gen,
                    end: destination + limit - 1,
                    start: destination,
                });
                // slutt flyttes og en ny legges til
                to_be_added.push(SeedRange {
                    last_gen_touched: gen - 1,
                    end: s.end,
                    start: source + limit,
                });

                s.update_end(source - 1);
            }
        }
    });

    to_be_added.iter().for_each(|a| {
        seeds.push(*a);
    });
    seeds
}

#[derive(Copy, Clone)]
struct SeedRange {
    start: u64,
    end: u64,
    last_gen_touched: u32,
}

impl SeedRange {
    fn update_end(&mut self, newp: u64) {
        self.end = newp;
    }
    fn update_start(&mut self, newp: u64) {
        self.start = newp;
    }

    fn update_gen(&mut self, last_gen_touched: u32) {
        self.last_gen_touched = last_gen_touched;
    }
}


fn with_seeds_v2(input: &str, mut seeds: Vec<SeedRange>) -> Option<u64> {
    let parts: Vec<&str> = input.split(':').collect();
    for part in 2..parts.len() {
        let txt_map: Vec<&str> = parts.get(part).unwrap()
            .split('\n')
            .filter(|p| {
                !(p.is_empty() || p.contains("map"))
            }).collect();

        for single_map in txt_map {
            let data: Vec<u64> = single_map.split(' ')
                .map(|seed| {
                    seed.parse::<u64>().unwrap()
                }).collect();
            let destination = data.first().unwrap().to_owned();
            let source = data.get(1).unwrap().to_owned();
            let limit = data.get(2).unwrap().to_owned();
            seeds = move_seeds(seeds, source, limit, part as u32, destination);
        }
    }
    seeds.iter()
        .map(|s| s.start)
        .min()
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
