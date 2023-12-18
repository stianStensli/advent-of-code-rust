use std::collections::HashSet;
advent_of_code::solution!(18);

fn insert_and_push(vec: &mut Vec<char>, index: usize, value: char) {
    while vec.len() <= index {
        vec.push('.');
    }
    vec[index] = value;
}

fn insert_and_push_arr(vec: &mut Vec<Vec<char>>, row: usize, col: usize, value: char) {
    while vec.len() <= row {
        vec.push(Vec::new());
    }
    while vec[row].len() <= col {
        vec[row].push('.');
    }
    vec[row][col] = value;
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut r = 0;
    let mut c = 0;
    let mut board: Vec<Vec<char>> = vec![vec!['.'; 1]; 1];
    let mut start_dir = " ";
    let mut last_dir = " ";
    input.lines().for_each(|l| {
        if l.is_empty() {
            return;
        }
        /*
        board.iter().for_each(|b| {
            print!("\n");
            b.iter()
                .for_each(|v| if *v { print!("#") } else { print!(".") })
        });*/
        let mut problem = l.split(' ');
        let dir = problem.next().unwrap();
        let nr = problem.next().unwrap().parse::<u64>().unwrap();
        if start_dir == " " {
            start_dir = dir;
        }
        // println!("\nBoard: r: {}, c: {}, dir: {}, nr: {}", r, c, dir, nr);
        if dir == "R" {
            if last_dir == "D" {
                board[r][c] = 'L';
            } else if last_dir == "U" {
                board[r][c] = 'F';
            }
            // println!("r: {}, c: {}", r, c);
            for _ in 0..nr {
                c += 1;
                insert_and_push(&mut board[r], c, '-')
            }
        } else if dir == "D" {
            if last_dir == "R" {
                board[r][c] = '7';
            } else if last_dir == "L" {
                board[r][c] = 'F';
            }
            for _ in 0..nr {
                r += 1;
                insert_and_push_arr(&mut board, r, c, '|')
            }
        } else if dir == "L" {
            if last_dir == "D" {
                board[r][c] = 'J';
            } else if last_dir == "U" {
                board[r][c] = '7';
            }
            for _ in 0..nr {
                if c == 0 {
                    for i in 0..board.len() {
                        for _ in 0..10 {
                            board[i].insert(0, '.');
                        }
                    }
                    c += 10;
                }
                c -= 1;
                insert_and_push(&mut board[r], c, '-');
            }
        } else if dir == "U" {
            if last_dir == "R" {
                board[r][c] = 'J';
            } else if last_dir == "L" {
                board[r][c] = 'L';
            }
            for _ in 0..nr {
                if r == 0 {
                    board.insert(0, Vec::new());
                    r += 1;
                }
                r -= 1;
                insert_and_push(&mut board[r], c, '|');
            }
        }
        last_dir = dir;
    });
    if start_dir == "R" && last_dir == "U" {
        board[r][c] = 'F';
    } else if start_dir == "L" && last_dir == "U" {
        board[r][c] = '7';
    } else if start_dir == "L" && last_dir == "D" {
        board[r][c] = 'J';
    } else if start_dir == "R" && last_dir == "D" {
        board[r][c] = 'L';
    }
    let mut res = 0;
    // println!("\nBoard: ");
    board.iter().for_each(|b| {
        let mut inside = false;
        // print!("\n");
        b.iter().for_each(|v| {
            /*
            if v == &'F' || v == &'7' {
                if start_at_corner {
                    res += 1;
                    start_at_corner = false;
                    print!("{}", v);
                    return;
                }
                start_at_corner = true;
            }*/
            if v == &'|' || v == &'J' || v == &'L' {
                inside = !inside;
                res += 1;
            } else if v != &'.' || inside {
                res += 1;
            } /*
              if inside && v == &'.' {
                  print!("o")
              } else {
                  print!("{}", v)
              }
              */
            /*
            if *v {
                last_hash = true;
                res += 1;
                print!("#")
            } else {
                if last_hash && crossed {
                    inside != inside;
                }
                if inside {
                    res += 1;
                }
                print!(".")
            }*/
        })
    });
    Some(res)
}

#[derive(Debug, Clone, Copy)]
struct FenceNode {
    start_r: u64,
    end_r: u64,
    start_c: u64,
    end_c: u64,
    start_char: char,
    end_char: char,
}

impl FenceNode {
    fn update_r(&mut self, nr: u64) {
        self.start_r += nr;
        self.end_r += nr;
    }
    fn update_c(&mut self, nr: u64) {
        self.start_c += nr;
        self.end_c += nr;
    }

    fn update_char(&mut self, new_char: char) {
        if self.end_char == ' ' {
            self.end_char = new_char;
        } else {
            self.start_char = new_char;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut r: u64 = 0;
    let mut c: u64 = 0;
    let mut board: Vec<FenceNode> = Vec::new();
    let mut rows_of_interest: HashSet<u64> = HashSet::new();
    rows_of_interest.insert(0);
    let mut start_dir = " ";
    let mut last_dir = " ";
    input.lines().for_each(|l| {
        if l.is_empty() {
            return;
        }
        let mut problem = l.split(' ');
        let dir = problem.next().unwrap();
        let nr = problem.next().unwrap().parse::<u64>().unwrap();
        let mut dir_char = ' ';
        if start_dir == " " {
            start_dir = dir;
        }
        println!("\nBoard: r: {}, c: {}, dir: {}, nr: {}", r, c, dir, nr);
        if dir == "R" {
            if last_dir == "D" {
                dir_char = 'L';
            } else if last_dir == "U" {
                dir_char = 'F';
            }
            board.push(FenceNode {
                start_c: c + 1,
                end_c: c + nr,
                start_r: r,
                end_r: r,
                start_char: dir_char,
                end_char: ' ', // TODO
            });
            c += nr;
        } else if dir == "D" {
            if last_dir == "R" {
                dir_char = '7';
            } else if last_dir == "L" {
                dir_char = 'F';
            }

            board.push(FenceNode {
                start_c: c,
                end_c: c,
                start_r: r + 1,
                end_r: r + nr,
                start_char: dir_char,
                end_char: ' ', // TODO
            });
            r += nr;
        } else if dir == "L" {
            if last_dir == "D" {
                dir_char = 'J';
            } else if last_dir == "U" {
                dir_char = '7';
            }
            if nr > c {
                board.iter_mut().for_each(|b| b.update_c(nr));
                println!("yp?");
                c += nr;
            }
            board.push(FenceNode {
                start_c: c - nr,
                end_c: c - 1,
                start_r: r,
                end_r: r,
                start_char: ' ', // TODO
                end_char: dir_char,
            });
            c -= nr;
        } else if dir == "U" {
            if last_dir == "R" {
                dir_char = 'J';
            } else if last_dir == "L" {
                dir_char = 'L';
            }
            if nr > r {
                board.iter_mut().for_each(|b| b.update_r(nr));
                r += nr
            }
            board.push(FenceNode {
                start_c: c,
                end_c: c,
                start_r: r - nr,
                end_r: r - 1,
                start_char: ' ', // TODO
                end_char: dir_char,
            });
            r -= nr;
        }
        let b_l = board.len();
        if b_l > 1 {
            board[b_l - 2].update_char(dir_char);
        }

        //println!("last: {:?}", board.last().unwrap());
        last_dir = dir;
    });
    let b_l = board.len();
    if start_dir == "R" && last_dir == "U" {
        board[b_l - 1].update_char('F');
        board[0].update_char('F');
    } else if start_dir == "L" && last_dir == "U" {
        board[b_l - 1].update_char('7');
        board[0].update_char('7');
    } else if start_dir == "L" && last_dir == "D" {
        board[b_l - 1].update_char('J');
        board[0].update_char('J');
    } else if start_dir == "R" && last_dir == "D" {
        board[b_l - 1].update_char('L');
        board[0].update_char('L');
    }
    // TODO: loop can be writen in main loop
    board.sort_by(|b, b1| b.start_r.cmp(&b1.start_r));
    board.iter().for_each(|b| {
        rows_of_interest.insert(b.start_r);
        rows_of_interest.insert(b.end_r);
    });
    let mut rows_of_interest = rows_of_interest.iter().collect::<Vec<&u64>>();

    let mut res = 0;
    let mut nex_row_nr = 0;
    rows_of_interest.iter().for_each(|row| {
        let mut inside = false;
        let mut inside_next = false;
        let r_res = 0;
        let row = **row;
        println!("Point: {}", row);
        let mut active_nodes: Vec<&FenceNode> = board
            .iter()
            .filter(|b| row >= b.start_r && row <= b.end_r)
            .collect();
        active_nodes.sort_by(|r, s| r.start_c.cmp(&s.start_c));
        active_nodes.iter().for_each(|node| {})
    });
    // println!("\nBoard: ");
    println!("res: {}", board.len());
    board.iter().for_each(|b| {
        println!("{:?}", b);
    });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
