use std::collections::HashSet;
advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u64> {
    main_stuff(input, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    main_stuff(input, true)
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

pub fn main_stuff(input: &str, is_part2: bool) -> Option<u64> {
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
        let mut dir = problem.next().unwrap();
        let mut nr = problem.next().unwrap().parse::<u64>().unwrap();

        if is_part2 {
            let hash = problem.next().unwrap();

            let hash_nr: &str = &hash[2..7];
            nr = u64::from_str_radix(hash_nr, 16).unwrap();
            let new_dir: u64 = hash[7..8].parse().unwrap();
            if new_dir == 0 {
                dir = "R";
            } else if new_dir == 1 {
                dir = "D";
            } else if new_dir == 2 {
                dir = "L";
            } else if new_dir == 3 {
                dir = "U";
            }
        }

        let mut dir_char = ' ';
        if start_dir == " " {
            start_dir = dir;
        }
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
                start_char: '-',
                end_char: ' ',
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
                start_char: '|',
                end_char: ' ',
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
                // println!("yp?");
                c += nr;
            }
            board.push(FenceNode {
                start_c: c - nr,
                end_c: c - 1,
                start_r: r,
                end_r: r,
                start_char: ' ',
                end_char: '-',
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
                start_char: ' ',
                end_char: '|',
            });
            r -= nr;
        }
        let b_l = board.len();
        if b_l > 1 {
            board[b_l - 2].update_char(dir_char);
        }
        last_dir = dir;
    });
    let b_l = board.len();
    if start_dir == "R" && last_dir == "U" {
        board[b_l - 1].update_char('F');
    } else if start_dir == "L" && last_dir == "U" {
        board[b_l - 1].update_char('7');
    } else if start_dir == "L" && last_dir == "D" {
        board[b_l - 1].update_char('J');
    } else if start_dir == "R" && last_dir == "D" {
        board[b_l - 1].update_char('L');
    }
    // TODO: loop can be writen in main loop
    board.sort_by(|b, b1| b.start_r.cmp(&b1.start_r));
    board.iter().for_each(|b| {
        rows_of_interest.insert(b.start_r.saturating_sub(1));
        rows_of_interest.insert(b.end_r.saturating_sub(1));
        rows_of_interest.insert(b.start_r);
        rows_of_interest.insert(b.end_r);
    });
    let mut rows_of_interest = rows_of_interest.iter().collect::<Vec<&u64>>();
    rows_of_interest.sort();

    let mut res = 0;
    let mut last_row_nr: u64 = 0;
    let mut last_row_res: u64 = 0;

    rows_of_interest.iter().for_each(|row| {
        let row = **row;

        let mut inside_next = false;
        let mut active_nodes: Vec<&FenceNode> = board
            .iter()
            .filter(|b| row >= b.start_r && row <= b.end_r)
            .collect();
        active_nodes.sort_by(|r, s| r.start_c.cmp(&s.start_c));

        let mut last_col = 0;
        active_nodes.iter().for_each(|node| {
            let is_line = node.end_c - node.start_c == 0;
            let is_diff = node.start_r != row || node.end_r != row;

            if is_line && !is_diff {
                last_row_res += 1;
                if inside_next {
                    last_row_res += node.start_c - last_col;
                }
                if node.start_char == 'F'
                    || node.start_char == '7'
                    || node.end_char == 'F'
                    || node.end_char == '7'
                {
                    inside_next = !inside_next;
                }
            } else if is_line && is_diff {
                last_row_res += 1;
                if inside_next {
                    last_row_res += node.end_c - last_col;
                }
                if (row == node.start_r
                    && (node.start_char == 'F' || node.start_char == '|' || node.start_char == '7'))
                    || (row == node.end_r
                        && (node.end_char == 'F' || node.end_char == '|' || node.end_char == '7'))
                    || (row != node.start_r && row != node.end_r)
                {
                    inside_next = !inside_next;
                }
            } else {
                last_row_res += 1 + node.end_c - node.start_c;

                if inside_next {
                    last_row_res += node.start_c - last_col;
                }
                if node.start_r == row
                    && (node.start_char == 'F' || node.start_char == '|' || node.start_char == '7')
                {
                    inside_next = !inside_next;
                }
                if node.end_r == row
                    && (node.end_char == 'F' || node.end_char == '|' || node.end_char == '7')
                {
                    inside_next = !inside_next;
                }
            }
            last_col = node.end_c + 1;
        });

        if row == 0 {
            res += last_row_res;
        } else {
            res += last_row_res * (row - last_row_nr);
        }
        last_row_nr = row;
        last_row_res = 0;
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
