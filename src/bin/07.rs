use std::cmp::Ordering;
advent_of_code::solution!(7);


struct Hand {
    cards: [u32; 5],
    bet: u64,
    score: u32,
}
/*
impl Hand {
    fn max(o: Hand, b: Hand) {
        return cmp(o, b);
    }
    fn cmp(o: Hand, b: Hand) {}
    //fun partial_cmp(a, b) == Some(cmp(a, b)).
    //max(a, b) == max_by(a, b, cmp) (ensured by the default implementation).
    //min(a, b) == min_by(a, b, cmp) (ensured by the default implementation).
    //For a.clamp(min, max), see the method docs (ensured by the default implementation).
}
*/

fn get_nr_from_card(card: char) -> u32 {
    if card.is_numeric() {
        //println!("{}", card.to_digit(10).unwrap() - 1);
        card.to_digit(10).unwrap() - 1
    } else if card == 'A' {
        13
    } else if card == 'K' {
        12
    } else if card == 'Q' {
        11
    } else if card == 'J' {
        10
    } else if card == 'T' {
        9
    } else {
        println!("what?: {}", card);
        0
    }
}

fn get_hand_score(cards: [u32; 5]) -> u32 {
    let mut hand_score = [0; 14];
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

pub fn part_one(input: &str) -> Option<u64> {
    let mut hands = Vec::new();
    input.lines().for_each(|l| {
        let mut index = 0;
        let mut p_string = "";
        let split = l.split_whitespace();
        let mut cards = [0;5];
        let mut hand_score = 0;

        split.for_each(|s| {
            if index == 0 {
                p_string = s;
                let mut j = 0;
                s.chars().for_each(|card| {
                    cards[j] = get_nr_from_card(card);
                    j+=1;
                });
                hand_score = get_hand_score(cards);
                index += 1;
            } else {
                hands.push(Hand {
                    cards: cards,
                    bet: s.parse().unwrap(),
                    score: hand_score,
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
    /*
    hands.iter().for_each(|hand| {
        println!("hand: {}, h_s: {}, b: {}", hand.p_cards, hand.score, hand.bet)
    });*/
    let mut index = 0;
    Some(hands.iter()
        .map(|hand| {
            index += 1;
            hand.bet * index
        })
        .sum())
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
        assert_eq!(result, Some(251287184));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
