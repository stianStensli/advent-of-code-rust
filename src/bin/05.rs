advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = lines.get(0)
        .unwrap()
        .replace("seeds: ", "");
    let mut seeds: Vec<u32> = seeds.split(" ")
        .map(|seed| {
            seed.parse::<u32>().unwrap()
        }).collect();
    /*
        let tittles: Vec<&&str> = lines.iter().filter(|z| z.contains("map"))
            .colct();

        print!("seeds:");
        for i in 0..seeds.len() {
            print!(" {}", seeds[i])
        }*/
        //print!("\n");
        let parts: Vec<&str> = input.split(":").collect();
        for part in 2..parts.len() {
            //println!("{}",tittles[part-2]);
            let mut touced_seed: Vec<bool> = seeds.iter().map(|_| false).collect();
            let txt_map: Vec<&str> = parts.get(part).unwrap()
                .split("\n")
                .filter(|p| {
                    !(p.is_empty() || p.contains("map"))
                }).collect();
            for single_map in txt_map {
                let data: Vec<u32> = single_map.split(" ")
                    .map(|seed| {
                        seed.parse::<u32>().unwrap()
                    }).collect();
                let destination = data.get(0).unwrap().to_owned();
                let source = data.get(1).unwrap().to_owned();
                let limit = data.get(2).unwrap().to_owned();

                //println!("dest: {}, src: {}, limit: {}", destination, source, limit);

                for i in 0..seeds.len() {
                    let seed = seeds[i];
                    //println!("seed : {}",seed);
                    if touced_seed[i] {
                        continue;
                    }
                    /*if 52 == seed && source == 11 {
                        println!("ok?");
                        println!("dest: {}, src: {}, limit: {}", destination, source, limit);
                        println!("res: {}, min: {}, max: {}", seed >= source && seed <= source + limit,source,source + limit );

                    }*/
                    //println!("res: {}, min: {}, max: {}", seed >= source && seed <= source + limit,source,source + limit );
                    if seed >= source && seed < source + limit {
                        //println!("moving {}, to {}. det: {}, src: {}", seed, destination + (seed - source),
                        //         destination, source);
                        seeds[i] = destination + (seed - source);
                        touced_seed[i] = true;
                    }
                }
            }
            /*println!();
            print!("seeds:");
            for i in 0..seeds.len() {
                print!(" {}", seeds[i])
            }
            print!("\n");
            print!("\n");*/
        }


        seeds.iter().min().to_owned().map(|m| m.to_owned())
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
            assert_eq!(result, Some(35));
        }

        #[test]
        fn test_part_two() {
            let result = part_two(&advent_of_code::template::read_file("examples", DAY));
            assert_eq!(result, None);
        }
    }
