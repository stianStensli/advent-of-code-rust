advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy)]
struct FNode {
    start_dir: char,
    steps: u32,
    r: usize,
    c: usize,
    pipe: char,
}

#[derive(Debug, Clone, Copy)]
struct LNode {
    r: usize,
    c: usize,
}


pub fn part_one(input: &str) -> Option<u32> {
    let board: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let nr_of_col = board[0].len();
    let mut explored_board: Vec<Vec<Option<FNode>>> = board.iter().map(|l| l.iter().map(|_| None).collect()).collect();
    let start_row = board.iter().position(|l| l.contains(&'S')).unwrap();
    let start_col = board[start_row].iter().position(|l| l == &'S').unwrap();

    explored_board[start_row][start_col] = Option::from(FNode {
        steps: 0,
        start_dir: '.',
        r: start_row,
        c: start_col,
        pipe: 'S',
    });

    let mut leaf: Vec<FNode> = vec![FNode {
        steps: 0,
        start_dir: '.',
        r: start_row,
        c: start_col,
        pipe: 'S',
    }];

    Some('mainloop: loop {
        let curr = leaf.pop();
        if curr.is_none() {
            break 0;
        }
        let curr = curr.unwrap();

        for i in 0..4 {
            let mut dir = '.';
            let mut col = curr.c;
            let mut row = curr.r;
            let pipe = board[row][col];
            let mut can_connect = false;

            if i == 0 && curr.r != 0 && (pipe == '|' || pipe == 'L' || pipe == 'J' || pipe == 'S') {
                dir = 'u';
                row -= 1;
                let pipe = board[row][col];
                if pipe == '|' || pipe == 'F' || pipe == '7' {
                    can_connect = true;
                }
            } else if i == 1 && curr.r != board.len() - 1 && (pipe == '|' || pipe == 'F' || pipe == '7' || pipe == 'S') {
                dir = 'd';
                row += 1;
                let pipe = board[row][col];
                if pipe == '|' || pipe == 'L' || pipe == 'J' {
                    can_connect = true;
                }
            } else if i == 2 && curr.c != 0 && (pipe == '-' || pipe == 'J' || pipe == '7' || pipe == 'S') {
                dir = 'l';
                col -= 1;
                let pipe = board[row][col];
                if pipe == '-' || pipe == 'L' || pipe == 'F' {
                    can_connect = true;
                }
            } else if i == 3 && curr.c != nr_of_col - 1 && (pipe == '-' || pipe == 'L' || pipe == 'F' || pipe == 'S') {
                dir = 'r';
                col += 1;
                let pipe = board[row][col];
                if pipe == '-' || pipe == 'J' || pipe == '7' {
                    can_connect = true;
                }
            }
            if (col == curr.c && row == curr.r) || !can_connect {
                // can not connect to next leaf
                continue;
            }
            {
                let e_b = explored_board[row][col].as_ref();
                if let Some(e_b) = e_b {
                    if e_b.start_dir == curr.start_dir {
                        continue;
                    }
                    break 'mainloop curr.steps + 1;
                }
            }
            let start_dir = if curr.start_dir == '.' {
                dir
            } else {
                curr.start_dir
            };
            let new_node = FNode {
                steps: curr.steps + 1,
                start_dir,
                r: row,
                c: col,
                pipe: board[row][col],
            };

            leaf.insert(0, new_node);
            explored_board[row][col] = Some(new_node);
        }
    })
}

fn explode_board(board: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut explode_board: Vec<Vec<char>> =Vec::new();
    board.iter().for_each(|v| {
        let mut explode_row = Vec::new();
        {
            v.iter().for_each(|v| {
                let v = *v;
                explode_row.push(v);
                if v == 'F' || v == 'L'|| v == '-'{
                    explode_row.push('-');
                } else {
                    explode_row.push(' ');
                }
            });
        }

        let mut explode_row_2 = Vec::new();
        {
            explode_row.iter().for_each(|v| {
                let v = *v;
                if v == '|' || v == '7' || v == 'F'{
                    explode_row_2.push('|');
                } else {
                    explode_row_2.push(' ');
                }
            });
        }

        explode_board.push(explode_row);
        explode_board.push(explode_row_2);
    });
    explode_board
}

pub fn part_two(input: &str) -> Option<u32> {
    let board: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let nr_of_col = board[0].len();
    let mut explored_board: Vec<Vec<Option<FNode>>> = board.iter().map(|l| l.iter().map(|_| None).collect()).collect();
    let start_row = board.iter().position(|l| l.contains(&'S')).unwrap();
    let start_col = board[start_row].iter().position(|l| l == &'S').unwrap();

    explored_board[start_row][start_col] = Option::from(FNode {
        steps: 0,
        start_dir: '.',
        r: start_row,
        c: start_col,
        pipe: 'S',
    });

    let mut leaf: Vec<FNode> = vec![FNode {
        steps: 0,
        start_dir: '.',
        r: start_row,
        c: start_col,
        pipe: 'S',
    }];
    let mut start_dir_1 = ' ';
    let mut start_dir_2 = ' ';
    Some('mainloop: loop {
        let curr = leaf.pop();
        if curr.is_none() {
            break 0;
        }
        let curr = curr.unwrap();

        for i in 0..4 {
            let mut dir = '.';
            let mut col = curr.c;
            let mut row = curr.r;
            let pipe = board[row][col];
            let mut can_connect = false;

            if i == 0 && curr.r != 0 && (pipe == '|' || pipe == 'L' || pipe == 'J' || pipe == 'S') {
                dir = 'u';
                row -= 1;
                let pipe = board[row][col];
                if pipe == '|' || pipe == 'F' || pipe == '7' {
                    can_connect = true;
                }
            } else if i == 1 && curr.r != board.len() - 1 && (pipe == '|' || pipe == 'F' || pipe == '7' || pipe == 'S') {
                dir = 'd';
                row += 1;
                let pipe = board[row][col];
                if pipe == '|' || pipe == 'L' || pipe == 'J' {
                    can_connect = true;
                }
            } else if i == 2 && curr.c != 0 && (pipe == '-' || pipe == 'J' || pipe == '7' || pipe == 'S') {
                dir = 'l';
                col -= 1;
                let pipe = board[row][col];
                if pipe == '-' || pipe == 'L' || pipe == 'F' {
                    can_connect = true;
                }
            } else if i == 3 && curr.c != nr_of_col - 1 && (pipe == '-' || pipe == 'L' || pipe == 'F' || pipe == 'S') {
                dir = 'r';
                col += 1;
                let pipe = board[row][col];
                if pipe == '-' || pipe == 'J' || pipe == '7' {
                    can_connect = true;
                }
            }
            if (col == curr.c && row == curr.r) || !can_connect {
                // can not connect to next leaf
                continue;
            }
            {
                let e_b = explored_board[row][col].as_ref();
                if let Some(e_b) = e_b {
                    if e_b.start_dir == curr.start_dir {
                        continue;
                    }
                    start_dir_1 = curr.start_dir;
                    start_dir_2 = e_b.start_dir;
                    break 'mainloop curr.steps + 1;
                }
            }
            let start_dir = if curr.start_dir == '.' {
                dir
            } else {
                curr.start_dir
            };
            let new_node = FNode {
                steps: curr.steps + 1,
                start_dir,
                r: row,
                c: col,
                pipe: board[row][col],
            };

            leaf.insert(0, new_node);
            explored_board[row][col] = Some(new_node);
        }
    });

    let mut numb_of_res = 0;
    let fence_board: Vec<Vec<char>> = explored_board.iter().map(|l| l.iter().map(|b| {
        match b {
            Some(b) => {
                if b.start_dir == start_dir_1 || b.start_dir == start_dir_2 {
                    b.pipe
                } else if b.pipe == 'S' {
                    if (start_dir_1 == 'u' && start_dir_2 == 'd') || (start_dir_2 == 'u' && start_dir_1 == 'd') {
                        return '|';
                    } else if (start_dir_1 == 'l' && start_dir_2 == 'r') || (start_dir_1 == 'r' && start_dir_2 == 'l') {
                        return '-';
                    } else if (start_dir_1 == 'd' && start_dir_2 == 'r') || (start_dir_1 == 'r' && start_dir_2 == 'd') {
                        return 'F';
                    } else if (start_dir_1 == 'd' && start_dir_2 == 'l') || (start_dir_1 == 'l' && start_dir_2 == 'd') {
                        return '7';
                    } else if (start_dir_1 == 'u' && start_dir_2 == 'r') || (start_dir_1 == 'r' && start_dir_2 == 'u') {
                        return 'L';
                    } else if (start_dir_1 == 'u' && start_dir_2 == 'l') || (start_dir_1 == 'l' && start_dir_2 == 'u') {
                        return 'J';
                    }
                    '?'
                } else {
                    numb_of_res += 1;
                    '.'
                }
            }
            _ => {
                numb_of_res += 1;
                '.'
            }
        }
    }).collect()).collect();

    let mut fence_board = explode_board(fence_board);

    let mut row_index = 0;
    let mut leaf: Vec<LNode> = Vec::new();
    let mut explored_board: Vec<Vec<bool>> = fence_board.iter().map(|l| l.iter().map(|_| false).collect()).collect();
    let nr_of_col = explored_board[0].len();

    //println!("debug");
    fence_board.iter().for_each(|row| {
        let mut col_index = 0;

        //println!("{:?}", row);
        if row_index == 0 || row_index == fence_board.len() - 1{
            row.iter().for_each(|v| {
                if v == &'.' ||  v == &' ' {
                    leaf.push(LNode {
                        r: row_index,
                        c: col_index,
                    });
                }
                explored_board[row_index][col_index] = true;
                col_index += 1;
            });
        } else {
            let start = row[0];
            let end = row[row.len() - 1];
            if start == '.'||  start == ' '{
                leaf.push(LNode {
                    r: row_index,
                    c: 0,
                });
            }
            if end == '.' || end == ' '{
                leaf.push(LNode {
                    r: row_index,
                    c: row.len() - 1,
                });
            }

            explored_board[row_index][0] = true;
            explored_board[row_index][row.len() - 1] = true;
        }
        row_index += 1;
    });

    loop {
        let curr = leaf.pop();
        match curr {
            Some(curr) => {
                if fence_board[curr.r][curr.c] == '.' {
                    numb_of_res -= 1;
                    fence_board[curr.r][curr.c] = '0';
                }

                for i in 0..4 {
                    let mut col = curr.c;
                    let mut row = curr.r;
                    let mut can_connect = false;

                    if i == 0 && curr.r != 0 {
                        //dir = 'u';
                        row -= 1;
                        let pipe = fence_board[row][col];
                        if pipe == '.'  || pipe == ' '{
                            can_connect = true;
                        }
                    } else if i == 1 && curr.r != fence_board.len() - 1 {
                        //dir = 'd';
                        row += 1;
                        let pipe = fence_board[row][col];
                        if pipe == '.'  || pipe == ' '{
                            can_connect = true;
                        }
                    } else if i == 2 && curr.c != 0 {
                        //dir = 'l';
                        col -= 1;
                        let pipe = fence_board[row][col];
                        if pipe == '.'  || pipe == ' '{
                            can_connect = true;
                        }
                    } else if i == 3 && curr.c != nr_of_col - 1 {
                        //dir = 'r';
                        col += 1;
                        let pipe = fence_board[row][col];
                        if pipe == '.' ||  pipe == ' '{
                            can_connect = true;
                        }
                    }
                    if (col == curr.c && row == curr.r) || !can_connect {
                        // can not connect to next leaf
                        continue;
                    }
                    if explored_board[row][col] {
                        continue;
                    }
                    leaf.push(LNode {
                        r: row,
                        c: col,
                    });
                    explored_board[row][col] = true;
                }
            }
            _ => {
                break;
            }
        }
    };
    Some(numb_of_res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        //let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        let result1 = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        let result2 = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result1, Some(4));
        assert_eq!(result2, Some(8));
    }
}
