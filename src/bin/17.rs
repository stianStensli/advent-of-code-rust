advent_of_code::solution!(17);
use priority_queue::PriorityQueue;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    No,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LavaPosition {
    heat_loss: u64,
    r: usize,
    c: usize,
    steps_in_a_row: u32,
    dir: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LavaHash {
    r: usize,
    c: usize,
    steps_in_a_row: u32,
    dir: Direction,
}

fn get_hash(pos: &LavaPosition) -> LavaHash {
    LavaHash {
        c: pos.c,
        r: pos.r,
        steps_in_a_row: pos.steps_in_a_row,
        dir: pos.dir,
    }
}

fn legal_moves_ultra(
    position: &LavaPosition,
    next_dir: Direction,
    board: &Vec<Vec<u64>>,
    prune_board: &mut HashMap<LavaHash, u64>,
) -> Option<LavaPosition> {
    if position.dir != Direction::No && position.dir != next_dir && position.steps_in_a_row < 4 {
        // 5?
        return None;
    }
    if position.dir == Direction::Down && next_dir == Direction::Up {
        return None;
    }
    if position.dir == Direction::Up && next_dir == Direction::Down {
        return None;
    }
    if position.dir == Direction::Left && next_dir == Direction::Right {
        return None;
    }
    if position.dir == Direction::Right && next_dir == Direction::Left {
        return None;
    }

    let mut steps_in_a_row = 1;
    if position.dir == next_dir {
        steps_in_a_row = position.steps_in_a_row + 1;
    }
    if steps_in_a_row == 11 {
        return None;
    }
    let mut next = None;
    if next_dir == Direction::Up {
        if position.r == 0 {
            return None;
        }
        next = Some(LavaPosition {
            r: position.r - 1,
            c: position.c,
            heat_loss: position.heat_loss + board[position.r - 1][position.c],
            dir: next_dir,
            steps_in_a_row,
        });
    }
    if next_dir == Direction::Down {
        if position.r == board.len() - 1 {
            return None;
        }
        next = Some(LavaPosition {
            r: position.r + 1,
            c: position.c,
            heat_loss: position.heat_loss + board[position.r + 1][position.c],
            dir: next_dir,
            steps_in_a_row,
        });
    }
    if next_dir == Direction::Left {
        if position.c == 0 {
            return None;
        }
        next = Some(LavaPosition {
            r: position.r,
            c: position.c - 1,
            heat_loss: position.heat_loss + board[position.r][position.c - 1],
            dir: next_dir,
            steps_in_a_row,
        });
    }

    if next_dir == Direction::Right {
        if position.c == board[position.r].len() - 1 {
            return None;
        }
        next = Some(LavaPosition {
            r: position.r,
            c: position.c + 1,
            heat_loss: position.heat_loss + board[position.r][position.c + 1],
            dir: next_dir,
            steps_in_a_row,
        });
    }
    if let Some(n) = next {
        let hash = get_hash(&n);
        match prune_board.get(&hash) {
            None => {
                prune_board.insert(hash, n.heat_loss);
            }
            Some(h_l) => {
                if h_l <= &n.heat_loss {
                    return None;
                } else {
                    prune_board.insert(hash, n.heat_loss);
                }
            }
        }
    }
    next
}

fn legal_moves(
    position: &LavaPosition,
    next_dir: Direction,
    board: &Vec<Vec<u64>>,
    prune_board: &mut HashMap<LavaHash, u64>,
) -> Option<LavaPosition> {
    if position.dir == Direction::Down && next_dir == Direction::Up {
        return None;
    }
    if position.dir == Direction::Up && next_dir == Direction::Down {
        return None;
    }
    if position.dir == Direction::Left && next_dir == Direction::Right {
        return None;
    }
    if position.dir == Direction::Right && next_dir == Direction::Left {
        return None;
    }

    let mut steps_in_a_row = 1;
    if position.dir == next_dir {
        steps_in_a_row = position.steps_in_a_row + 1;
    }
    if steps_in_a_row == 4 {
        return None;
    }
    let mut next = None;
    if next_dir == Direction::Up {
        if position.r == 0 {
            return None;
        }
        next = Some(LavaPosition {
            r: position.r - 1,
            c: position.c,
            heat_loss: position.heat_loss + board[position.r - 1][position.c],
            dir: next_dir,
            steps_in_a_row,
        });
    }
    if next_dir == Direction::Down {
        if position.r == board.len() - 1 {
            return None;
        }
        next = Some(LavaPosition {
            r: position.r + 1,
            c: position.c,
            heat_loss: position.heat_loss + board[position.r + 1][position.c],
            dir: next_dir,
            steps_in_a_row,
        });
    }
    if next_dir == Direction::Left {
        if position.c == 0 {
            return None;
        }
        next = Some(LavaPosition {
            r: position.r,
            c: position.c - 1,
            heat_loss: position.heat_loss + board[position.r][position.c - 1],
            dir: next_dir,
            steps_in_a_row,
        });
    }

    if next_dir == Direction::Right {
        if position.c == board[position.r].len() - 1 {
            return None;
        }
        next = Some(LavaPosition {
            r: position.r,
            c: position.c + 1,
            heat_loss: position.heat_loss + board[position.r][position.c + 1],
            dir: next_dir,
            steps_in_a_row,
        });
    }
    if let Some(n) = next {
        let hash = get_hash(&n);
        match prune_board.get(&hash) {
            None => {
                prune_board.insert(hash, n.heat_loss);
            }
            Some(h_l) => {
                if h_l <= &n.heat_loss {
                    return None;
                } else {
                    prune_board.insert(hash, n.heat_loss);
                }
            }
        }
    }
    next
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut board = Vec::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            return;
        }
        let mut row = Vec::new();
        l.chars()
            .for_each(|c| row.push(c.to_digit(10).unwrap() as u64));
        board.push(row)
    });
    let mut prune_board: HashMap<LavaHash, u64> = HashMap::new();
    let w_rows = board.len() - 1;
    let w_cols = board[0].len() - 1;
    let mut pq = PriorityQueue::new();
    pq.push(
        LavaPosition {
            c: 0,
            r: 0,
            dir: Direction::No,
            steps_in_a_row: 0,
            heat_loss: 0,
        },
        u64::MAX,
    );

    while let Some(current) = pq.pop() {
        if let Some(next) = legal_moves(&current.0, Direction::Up, &board, &mut prune_board) {
            if next.r == w_rows && next.c == w_cols {
                return Some(next.heat_loss);
            }
            pq.push(next, u64::MAX - next.heat_loss);
        };
        if let Some(next) = legal_moves(&current.0, Direction::Down, &board, &mut prune_board) {
            if next.r == w_rows && next.c == w_cols {
                return Some(next.heat_loss);
            }
            pq.push(next, u64::MAX - next.heat_loss);
        };
        if let Some(next) = legal_moves(&current.0, Direction::Right, &board, &mut prune_board) {
            if next.r == w_rows && next.c == w_cols {
                return Some(next.heat_loss);
            }
            pq.push(next, u64::MAX - next.heat_loss);
        };
        if let Some(next) = legal_moves(&current.0, Direction::Left, &board, &mut prune_board) {
            if next.r == w_rows && next.c == w_cols {
                return Some(next.heat_loss);
            }
            pq.push(next, u64::MAX - next.heat_loss);
        };
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut board = Vec::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            return;
        }
        let mut row = Vec::new();
        l.chars()
            .for_each(|c| row.push(c.to_digit(10).unwrap() as u64));
        board.push(row)
    });
    let w_rows = board.len() - 1;
    let w_cols = board[0].len() - 1;
    let mut pq = PriorityQueue::new();
    pq.push(
        LavaPosition {
            c: 0,
            r: 0,
            dir: Direction::No,
            steps_in_a_row: 0,
            heat_loss: 0,
        },
        u64::MAX,
    );
    let mut prune_board: HashMap<LavaHash, u64> = HashMap::new();

    while let Some(current) = pq.pop() {
        if let Some(next) = legal_moves_ultra(&current.0, Direction::Up, &board, &mut prune_board) {
            if next.r == w_rows && next.c == w_cols {
                if next.steps_in_a_row < 4 {
                    continue;
                }
                return Some(next.heat_loss);
            }
            pq.push(next, u64::MAX - next.heat_loss);
        };
        if let Some(next) = legal_moves_ultra(&current.0, Direction::Down, &board, &mut prune_board)
        {
            if next.r == w_rows && next.c == w_cols {
                if next.steps_in_a_row < 4 {
                    continue;
                }
                return Some(next.heat_loss);
            }
            pq.push(next, u64::MAX - next.heat_loss);
        };
        if let Some(next) =
            legal_moves_ultra(&current.0, Direction::Right, &board, &mut prune_board)
        {
            if next.r == w_rows && next.c == w_cols {
                if next.steps_in_a_row < 4 {
                    continue;
                }
                return Some(next.heat_loss);
            }
            pq.push(next, u64::MAX - next.heat_loss);
        };
        if let Some(next) = legal_moves_ultra(&current.0, Direction::Left, &board, &mut prune_board)
        {
            if next.r == w_rows && next.c == w_cols {
                if next.steps_in_a_row < 4 {
                    continue;
                }
                return Some(next.heat_loss);
            }
            pq.push(next, u64::MAX - next.heat_loss);
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
