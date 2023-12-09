advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let symbols_per_line: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            let mut index = 0;
            let mut indexs = Vec::new();
            l.chars().for_each(|c| {
                if !c.is_numeric() && c != '.' {
                    indexs.push(index);
                }
                index += 1;
            });
            indexs
        }).collect();
    /*
    symbols_per_line.iter().for_each(|s|{
        println!("{:?}",s);
    });*/

    let mut curr_index: usize = 0;
    Some(input.lines().map(|l| {
        let mut res_this_line = 0;
        let pre_symbols = if curr_index > 0 {
            symbols_per_line[curr_index - 1].clone()
        } else {
            Vec::new()
        };
        let post_symbols = if curr_index < symbols_per_line.len() - 1 {
            symbols_per_line[curr_index + 1].clone()
        } else {
            Vec::new()
        };
        let curr_symbols = symbols_per_line[curr_index].clone();
        let mut curr_number = Vec::new();
        let mut line_index: usize = 0;
        let mut cur_num_start: usize = 0;
        let mut cur_num_init = false;
        l.chars().for_each(|c| {
            let dig = c.to_digit(10);
            match dig {
                Some(d) => {
                    if !cur_num_init {
                        cur_num_init = true;
                        cur_num_start = line_index;
                    }
                    curr_number.push(d)
                }
                _ => {
                    if cur_num_init {
                        let dig = curr_number.iter().fold(0, |acc, d| {
                            acc * 10 + d
                        });
                        curr_number.clear();
                        let start = if cur_num_start == 0 {
                            0
                        } else {
                            cur_num_start - 1
                        };
                        let mut has_symbol = curr_symbols.contains(&line_index) || curr_symbols.contains(&start);
                        for i in start..line_index + 1 {
                            has_symbol = has_symbol ||
                                pre_symbols.contains(&i) ||
                                post_symbols.contains(&i);
                        }
                        if has_symbol {
                            res_this_line += dig;
                        }
                        //println!("dig: {}, has symbol: {}", dig, has_symbol);
                        cur_num_init = false;
                    }
                }
            }
            line_index += 1;
        });
        if cur_num_init {
            let dig = curr_number.iter().fold(0, |acc, d| {
                acc * 10 + d
            });
            curr_number.clear();
            let start = if cur_num_start == 0 {
                0
            } else {
                cur_num_start - 1
            };
            let mut has_symbol = curr_symbols.contains(&start);
            for i in start..line_index {
                has_symbol = has_symbol ||
                    pre_symbols.contains(&i) ||
                    post_symbols.contains(&i);
            }
            if has_symbol {
                res_this_line += dig;
            }
        }
        curr_index += 1;
        res_this_line
    }).sum())
}


#[derive(Debug)]
struct Gear {
    index: usize,
    rate: Vec<u32>,
}

impl Gear {
    fn add_numb(&mut self, newp: u32) {
        self.rate.push(newp);
    }
    fn get_index(&mut self) -> usize {
        self.index
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut symbols_per_line: Vec<Vec<Gear>> = input
        .lines()
        .map(|l| {
            let mut index = 0;
            let mut indexs = Vec::new();
            l.chars().for_each(|c| {
                if c == '*' {
                    indexs.push(Gear {
                        index,
                        rate: Vec::new(),
                    });
                }
                index += 1;
            });
            indexs
        }).collect();
    let nr_of_sym_lines = symbols_per_line.len();

    let mut curr_index: usize = 0;
    input.lines().for_each(|l| {
        let mut curr_number = Vec::new();
        let mut line_index: usize = 0;
        let mut cur_num_start: usize = 0;
        let mut cur_num_init = false;
        l.chars().for_each(|c| {
            let dig = c.to_digit(10);
            match dig {
                Some(d) => {
                    if !cur_num_init {
                        cur_num_init = true;
                        cur_num_start = line_index;
                    }
                    curr_number.push(d)
                }
                _ => {
                    if cur_num_init {
                        let dig = curr_number.iter().fold(0, |acc, d| {
                            acc * 10 + d
                        });
                        curr_number.clear();
                        let start = if cur_num_start == 0 {
                            0
                        } else {
                            cur_num_start - 1
                        };
                        {
                            let curr_symbols = symbols_per_line.get_mut(curr_index).unwrap();
                            curr_symbols.iter_mut().for_each(|g| {
                                if g.index == line_index || g.index == start {
                                    g.add_numb(dig);
                                }
                            });
                        }

                        if curr_index > 0 {
                            let pre_symbols = symbols_per_line.get_mut(curr_index - 1).unwrap();

                            pre_symbols.iter_mut().for_each(|g| {
                                if g.get_index() >= start && g.index <= line_index {
                                    g.add_numb(dig);
                                }
                            });
                        }

                        if curr_index < nr_of_sym_lines - 1 {
                            let pre_symbols = symbols_per_line.get_mut(curr_index + 1).unwrap();

                            pre_symbols.iter_mut().for_each(|g| {
                                if g.get_index() >= start && g.index <= line_index {
                                    g.add_numb(dig);
                                }
                            });
                        }
                        cur_num_init = false;
                    }
                }
            }

            line_index += 1;
        });

        if cur_num_init {
            let dig = curr_number.iter().fold(0, |acc, d| {
                acc * 10 + d
            });
            curr_number.clear();
            let start = if cur_num_start == 0 {
                0
            } else {
                cur_num_start - 1
            };
            {
                let curr_symbols = symbols_per_line.get_mut(curr_index).unwrap();
                curr_symbols.iter_mut().for_each(|g| {
                    if g.index == line_index || g.index == start {
                        g.add_numb(dig);
                    }
                });
            }

            if curr_index > 0 {
                let pre_symbols = symbols_per_line.get_mut(curr_index - 1).unwrap();

                pre_symbols.iter_mut().for_each(|g| {
                    if g.get_index() >= start && g.index <= line_index {
                        g.add_numb(dig);
                    }
                });
            }

            if curr_index < nr_of_sym_lines - 1 {
                let pre_symbols = symbols_per_line.get_mut(curr_index + 1).unwrap();

                pre_symbols.iter_mut().for_each(|g| {
                    if g.get_index() >= start && g.index <= line_index {
                        g.add_numb(dig);
                    }
                });
            }
        }
        curr_index += 1;
    });

    Some(symbols_per_line.iter().flatten().map(|s| {
        if s.rate.len() == 2 {
            *s.rate.first().unwrap() * *s.rate.last().unwrap()
        } else {
            0
        }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
