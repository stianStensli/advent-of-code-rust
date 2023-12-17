use crate::Direction::{Down, Left, Right, Up};
use std::cmp::max;
advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct LDirection {
    up: bool,
    down: bool,
    left: bool,
    rigth: bool,
}

struct Light {
    dir: Direction,
    c: usize,
    r: usize,
}
fn any_dir(b: &LDirection) -> bool {
    b.left || b.up || b.down || b.rigth
}

fn update_res_board(l: &Light, res: &LDirection) -> Option<LDirection> {
    if l.dir == Left {
        if res.left {
            return None;
        }
        return Some(LDirection {
            left: true,
            rigth: res.rigth,
            down: res.down,
            up: res.up,
        });
    }
    if l.dir == Right {
        if res.rigth {
            return None;
        }
        return Some(LDirection {
            left: res.left,
            rigth: true,
            down: res.down,
            up: res.up,
        });
    }
    if l.dir == Up {
        if res.up {
            return None;
        }
        return Some(LDirection {
            left: res.left,
            rigth: res.rigth,
            down: res.down,
            up: true,
        });
    }
    if l.dir == Down {
        if res.down {
            return None;
        }
        return Some(LDirection {
            left: res.left,
            rigth: res.rigth,
            down: true,
            up: res.up,
        });
    }
    None
}

fn get_dir_node(l: &Light, next_dir: Direction, max_r: usize, max_c: usize) -> Option<Light> {
    match next_dir {
        Up => {
            if l.r == 0 {
                return None;
            }
            Some(Light {
                dir: Up,
                c: l.c,
                r: l.r - 1,
            })
        }
        Down => {
            if l.r + 1 == max_r {
                return None;
            }
            Some(Light {
                dir: Down,
                c: l.c,
                r: l.r + 1,
            })
        }
        Left => {
            if l.c == 0 {
                return None;
            }
            Some(Light {
                dir: Left,
                c: l.c - 1,
                r: l.r,
            })
        }
        Right => {
            if l.c + 1 == max_c {
                return None;
            }
            Some(Light {
                dir: Right,
                c: l.c + 1,
                r: l.r,
            })
        }
    }
}

fn main_part(board: &Vec<Vec<char>>, start: Light) -> u64 {
    let max_r = board.len();
    let max_c = board[0].len();
    let mut res: u64 = 0;
    let mut res_board: Vec<Vec<LDirection>> = vec![
        vec![
            LDirection {
                rigth: false,
                left: false,
                down: false,
                up: false,
            };
            max_c
        ];
        max_r
    ];
    let mut stack: Vec<Light> = vec![start];

    while let Some(current) = stack.pop() {
        let mut new_ligth: Vec<Light> = Vec::new();
        let b_v = board[current.r][current.c];

        match b_v {
            '|' => match current.dir {
                Right | Left => {
                    if let Some(opt) = get_dir_node(&current, Down, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                    if let Some(opt) = get_dir_node(&current, Up, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
                _ => {
                    if let Some(opt) = get_dir_node(&current, current.dir, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
            },
            '-' => match current.dir {
                Up | Down => {
                    if let Some(opt) = get_dir_node(&current, Left, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                    if let Some(opt) = get_dir_node(&current, Right, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
                _ => {
                    if let Some(opt) = get_dir_node(&current, current.dir, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
            },
            '\\' => match current.dir {
                Right => {
                    if let Some(opt) = get_dir_node(&current, Down, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
                Left => {
                    if let Some(opt) = get_dir_node(&current, Up, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
                Up => {
                    if let Some(opt) = get_dir_node(&current, Left, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
                Down => {
                    if let Some(opt) = get_dir_node(&current, Right, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
            },
            '/' => match current.dir {
                Right => {
                    if let Some(opt) = get_dir_node(&current, Up, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
                Left => {
                    if let Some(opt) = get_dir_node(&current, Down, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
                Up => {
                    if let Some(opt) = get_dir_node(&current, Right, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
                Down => {
                    if let Some(opt) = get_dir_node(&current, Left, max_r, max_c) {
                        new_ligth.push(opt)
                    }
                }
            },
            _ => {
                if let Some(opt) = get_dir_node(&current, current.dir, max_r, max_c) {
                    new_ligth.push(opt)
                }
            }
        }
        let r_old = &res_board[current.r][current.c];
        let any_dir = any_dir(r_old);

        if !any_dir {
            res += 1;
        }
        let new_res = update_res_board(&current, r_old);
        if let Some(new_res) = new_res {
            while let Some(l) = new_ligth.pop() {
                stack.push(l)
            }
            res_board[current.r][current.c] = new_res;
        }
    }

    res
}

fn get_board(input: &str) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            return;
        }
        let mut row = Vec::new();
        l.chars().for_each(|c| row.push(c));
        board.push(row);
    });
    board
}

pub fn part_one(input: &str) -> Option<u64> {
    let board = get_board(input);
    Some(main_part(
        &board,
        Light {
            dir: Right,
            c: 0,
            r: 0,
        },
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let board = get_board(input);
    let mut max_res = 0;
    for i in 0..board.len() {
        let t_res = main_part(
            &board,
            Light {
                dir: Down,
                c: i,
                r: 0,
            },
        );
        let b_res = main_part(
            &board,
            Light {
                dir: Up,
                c: i,
                r: board.len() - 1,
            },
        );
        max_res = max(max(b_res, t_res), max_res)
    }

    for i in 0..board[0].len() {
        let t_res = main_part(
            &board,
            Light {
                dir: Right,
                c: 0,
                r: i,
            },
        );
        let b_res = main_part(
            &board,
            Light {
                dir: Left,
                c: board[0].len() - 1,
                r: i,
            },
        );
        max_res = max(max(b_res, t_res), max_res)
    }
    Some(max_res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
