use std::cmp::Ordering;
advent_of_code::solution!(7);


struct Hand {
    cards: [u32; 5],
    bet: u64,
    score: u32,
}

fn get_nr_from_card_v1(card: char) -> u32 {
    get_nr_from_card(card, false)
}

fn get_nr_from_card_v2(card: char) -> u32 {
    get_nr_from_card(card, true)
}

fn get_nr_from_card(card: char, is_v2: bool) -> u32 {
    if card.is_numeric() {
        card.to_digit(10).unwrap()
    } else if card == 'A' {
        14
    } else if card == 'K' {
        13
    } else if card == 'Q' {
        12
    } else if card == 'J' && is_v2 {
        0
    } else if card == 'J' {
        11
    } else if card == 'T' {
        10
    } else {
        println!("what?: {}", card);
        0
    }
}

fn get_hand_score_v1(cards: [u32; 5]) -> u32 {
    let mut hand_score = [0; 15];
    cards.iter().for_each(|card| {
        hand_score[*card as usize] += 1;
    });

    let mut max = 1;
    let mut max_s = 0;

    hand_score.iter().for_each(|score| {
        if *score >= max {
            max_s = max;
            max = *score;
        } else if *score > max_s {
            max_s = *score;
        }
    });
    if max == 1 {
        1  // worst hand
    } else if max == 2 && max_s == 1 {
        2 // par
    } else if max == 2 && max_s == 2 {
        3 // two pars
    } else if max == 3 && max_s == 1 {
        4 // three pars
    } else if max == 3 && max_s == 2 {
        5 // house
    } else if max == 4 {
        6 // 4 of a kind
    } else if max == 5 {
        7 // 5 of a kind
    } else {
        println!("what? m:{}, m_s:{}", max, max_s);
        max
    }
}

fn get_hand_score_v2(cards: [u32; 5]) -> u32 {
    let mut hand_score = [0; 15];
    cards.iter()
        .for_each(|card| {
            hand_score[*card as usize] += 1;
        });

    let nr_of_j = hand_score[0];
    if nr_of_j == 5 || nr_of_j == 4 {
        return 70;  // 5 of a kind
    }

    let mut max = 1;
    let mut max_s = 0;

    hand_score.iter()
        .skip(1)
        .for_each(|score| {
            if *score >= max {
                max_s = max;
                max = *score;
            } else if *score > max_s {
                max_s = *score;
            }
        });

    if nr_of_j == 3 {
        if max == 1 {
            return 60 ;  // 4 of a kind
        } else if max == 2 {
            return 70 ;// 4 of a kind
        }
    }

    if nr_of_j == 2 {
        if max == 1 {
            return 40 ; // three of a kind
        } else if max == 2 {
            return 60 ; // 4 of a kind
        } else if max == 3 {
            return 70 ; // 5 of a kind
        }
    }

    if nr_of_j == 1 {
        if max == 1 {
            return 20 ; // par
        } else if max == 2 && max_s == 2 {
            return 50 ; // house
        } else if max == 2 {
            return 40 ; // three of a kind
        } else if max == 3 {
            return 60 ; // 4 of a kind
        } else if max == 4 {
            return 70 ; // 5 of a kind
        }
    }

    if max == 1 {
        10  // worst hand
    } else if max == 2 && max_s == 1 {
        20 // par
    } else if max == 2 && max_s == 2 {
        30 // two pars
    } else if max == 3 && max_s == 1 {
        40 // three of a kind
    } else if max == 3 && max_s == 2 {
        50 // house
    } else if max == 4 {
        60 // 4 of a kind
    } else {
        70 // 5 of a kind
    }
}

fn main_part(input: &str, get_hand_score: &dyn Fn([u32; 5]) -> u32, get_nr_from_card: &dyn Fn(char) -> u32) -> Option<u64> {
    let mut hands = Vec::new();
    input.lines().for_each(|l| {
        let mut index = 0;
        let split = l.split_whitespace();
        let mut cards = [0; 5];
        let mut score = 0;
        split.for_each(|s| {
            if index == 0 {
                let mut j = 0;
                s.chars().for_each(|card| {
                    cards[j] = get_nr_from_card(card);
                    j += 1;
                });
                score = get_hand_score(cards);
                index += 1;
            } else {
                hands.push(Hand {
                    cards,
                    score,
                    bet: s.parse().unwrap(),
                })
            }
        });
    });
    hands.sort_by(|o, p| {
        if o.score != p.score {
            o.score.cmp(&p.score)
        } else {
            for i in 0..o.cards.len() {
                let other_c = o.cards.get(i).unwrap();
                let p_c = p.cards.get(i).unwrap();
                if other_c != p_c {
                    return other_c.cmp(p_c);
                }
            }
            println!("what?");
            Ordering::Equal
        }
    });
    let mut index = 0;
    Some(hands.iter()
        .map(|hand| {
            index += 1;
            hand.bet * index
        })
        .sum())
}

pub fn part_one(input: &str) -> Option<u64> {
    main_part(input, &get_hand_score_v1, &get_nr_from_card_v1)
}

pub fn part_two(input: &str) -> Option<u64> {
    main_part(input, &get_hand_score_v2, &get_nr_from_card_v2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));// 251287184
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
