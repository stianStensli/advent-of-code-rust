advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy)]
struct FNode {
    start_dir: char,
    steps: u32,
    r: usize,
    c: usize,
}


pub fn part_one(input: &str) -> Option<u32> {
    let board: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let nr_of_col = board[0].len();
    let mut explored_board: Vec<Vec<Option<FNode>>> = board.iter().map(|l| l.iter().map(|_| None).collect()).collect();
    let start_row = board.iter().position(|l| l.contains(&'S')).unwrap();
    let start_col = board[start_row].iter().position(|l| l == &'S').unwrap();
    //println!("s_r: {}, s_c: {}", start_row, start_col);

    explored_board[start_row][start_col] = Option::from(FNode {
        steps: 0,
        start_dir: '.',
        r: start_row,
        c: start_col,
    });

    let mut leaf: Vec<FNode> = vec![FNode {
        steps: 0,
        start_dir: '.',
        r: start_row,
        c: start_col,
    }];

    let res = 'mainloop: loop {
        //println!("leafs: {:?}", leaf);
        let curr = leaf.pop();
        if curr.is_none() {
            break 0;
        }
        let curr = curr.unwrap();

/*
        println!("Curr Explored:");
        //explored_board.iter().for_each(|b| println!("{:?}", b));
        explored_board.iter().for_each(|b| {
            print!("\n");
            b.iter().for_each(|l| {
                if l.is_none() {
                    print!("x");
                } else {
                    let n = l.unwrap();
                    if n.c == curr.c && n.r == curr.r {
                        print!("C");
                    } else {
                        print!("{}", l.unwrap().steps);
                    }
                }
            })
        });
        print!("\n");*/

        for i in 0..4 {
            let mut dir = '.';
            let mut col = curr.c;
            let mut row = curr.r;
            let pipe = board[row][col];
            let mut can_connect = false;

            //println!("dir: {}, node {}", i, pipe);
            if i == 0 && curr.r != 0 && (pipe == '|' || pipe == 'L' || pipe == 'J' || pipe == 'S') {
                // opp
                //println!("UP?");
                dir = 'u';
                row -= 1;
                let pipe = board[row][col];
                if pipe == '|' || pipe == 'F' || pipe == '7' {
                   // println!("UP yes {}", pipe);
                    can_connect = true;
                }
            } else if i == 1 && curr.r != board.len() - 1 && (pipe == '|' || pipe == 'F' || pipe == '7' || pipe == 'S') {
                // ned
                //println!("DOWN");
                dir = 'd';
                row += 1;
                let pipe = board[row][col];
                if pipe == '|' || pipe == 'L' || pipe == 'J' {
                    //println!("DOWN yes {}", pipe);
                    can_connect = true;
                }
            } else if i == 2 && curr.c != 0 && (pipe == '-' || pipe == 'J' || pipe == '7' || pipe == 'S') {
                // venstre
                //println!("LEFT");
                dir = 'l';
                col -= 1;
                let pipe = board[row][col];
                if pipe == '-' || pipe == 'L' || pipe == 'F' {
                    //println!("LEFT yes {}", pipe);
                    can_connect = true;
                }
            } else if i == 3 && curr.c != nr_of_col - 1 && (pipe == '-' || pipe == 'L' || pipe == 'F' || pipe == 'S') {
                // høyre
                //println!("RIGTH");
                dir = 'r';
                col += 1;
                let pipe = board[row][col];
                if pipe == '-' || pipe == 'J' || pipe == '7' {
                    //println!("RIGTH yes {}", pipe);
                    can_connect = true;
                }
            }
            if (col == curr.c && row == curr.r) || !can_connect {
                if !can_connect{
                    //println!("STOP {}, con: {} row: {},  col: {}", board[row][col], can_connect, row, col);
                }
                // can not connect to next leaf
                continue;
            }

            //println!("Con {}", board[row][col]);
            {
                let e_b = explored_board[row][col].as_ref();
                if let Some(e_b) = e_b {
                    if e_b.start_dir == curr.start_dir {
                        //println!("Conn {}, cur: {}, eb: {}", board[row][col], curr.start_dir, e_b.start_dir);
                        continue;
                    }/*

                    explored_board.iter().for_each(|b| {
                        print!("\n");
                        b.iter().for_each(|l| {
                            if l.is_none() {
                                print!("x");
                            } else {
                                let n = l.unwrap();
                                if n.c == col && n.r == row {
                                    print!("Y");
                                } else {
                                    print!("{}", l.unwrap().steps);
                                }
                            }
                        })
                    });
                    print!("\n");

                    println!("yes: this: {:?}, next: {:?}",curr, e_b);*/
                    //println!("curr: {:?}, eb: {:?}", curr, e_b);
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
            };
            //println!("new node: {:?}", new_node);


            leaf.insert(0, new_node);
            explored_board[row][col] = Some(new_node);
        }
    };/*

    print!("\n");

    print!("\n");
    println!("Explored:");
    //explored_board.iter().for_each(|b| println!("{:?}", b));
    explored_board.iter().for_each(|b| {
        print!("\n");
        b.iter().for_each(|l| {
            if l.is_none() {
                print!("x");
            } else {
                print!("{}", l.unwrap().steps);
            }
        })
    });*/

    Some(res)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
