use std::collections::HashMap;
advent_of_code::solution!(19);

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut rule_mode = true;
    let mut function_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            rule_mode = false;
            return;
        }
        if rule_mode {
            let mut func_rule = l.split(['{', '}']);
            let func = func_rule.next().unwrap();
            //println!("func: {}", func);
            let prob_vec: Vec<&str> = func_rule.next().unwrap().split(',').collect();
            function_map.insert(func, prob_vec);
        } else {
            //",a=", ",s=", "}"
            let rule = l.split_once("{x=").unwrap();
            let rule = rule.1.split_once(",m=").unwrap();
            let x: u64 = rule.0.parse().unwrap();
            let rule = rule.1.split_once(",a=").unwrap();
            let m: u64 = rule.0.parse().unwrap();
            let rule = rule.1.split_once(",s=").unwrap();
            let a: u64 = rule.0.parse().unwrap();
            let rule = rule.1.split_once("}").unwrap();
            let s: u64 = rule.0.parse().unwrap();
            parts.push(Part { x, m, a, s });
            //println!("node: {:?}", parts.last().unwrap())
        }
    });
    Some(parts.iter().map(|part| {
        let mut current_function: String = "in".to_string();

        while current_function != "A" && current_function != "R" {
            //println!("current_function: {}", current_function);
            let op = function_map.get(current_function.as_str()).unwrap();
            let res = get_next_rule(op, part);
            current_function = res;
        }

        return if current_function == "A" {
            part.a + part.m + part.x + part.s
        } else {
            0
        };
    }).sum())
}

fn eval_rule(part_to_check: &str, nr: u64, part: &Part, bigger: bool) -> bool {
    let nr_p = match part_to_check {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => 0
    };
    if bigger {
        return nr_p < nr;
    }
    return nr_p > nr;
}

fn get_next_rule(rules: &Vec<&str>, part: &Part) -> String {
    for i in 0..rules.len() {
        let rule = rules[i];
        if rule.contains('<') {
            let prob = rule.split_once(':').unwrap();
            let m = prob.0.split_once('<').unwrap();
            if eval_rule(m.0, m.1.parse().unwrap(), part, true) {
                return prob.1.to_string();
            }
        } else if rule.contains('>') {
            let prob = rule.split_once(':').unwrap();
            let m = prob.0.split_once('>').unwrap();
            if eval_rule(m.0, m.1.parse().unwrap(), part, false) {
                return prob.1.to_string();
            }
        } else {
            return rule.to_string();
        }
    }
    panic!("nooo");
}

#[derive(Debug, Clone)]
struct PartRange {
    x: u64,
    x_e: u64,
    m: u64,
    m_e: u64,
    a: u64,
    a_e: u64,
    s: u64,
    s_e: u64,
}

impl PartRange {
    pub fn get_possibilities(self) -> u64 {
        (1 + self.x_e - self.x) * (1 + self.m_e - self.m) * (1 + self.a_e - self.a) * (1 + self.s_e - self.s)
    }

    // 2,8 < 5
    //2-4, 5-8
    fn split_bigger(self, part_to_check: &str, nr: u64) -> (Option<PartRange>, Option<PartRange>) {
        let mut x_s = self.x;
        let mut x_e = self.x_e;

        let mut m_s = self.m;
        let mut m_e = self.m_e;

        let mut a_s = self.a;
        let mut a_e = self.a_e;

        let mut s_s = self.s;
        let mut s_e = self.s_e;

        match part_to_check {
            "x" => {
                x_e = nr - 1;
                x_s = nr;
            }
            "m" => {
                m_e = nr - 1;
                m_s = nr;
            }
            "a" => {
                a_e = nr - 1;
                a_s = nr;
            }
            "s" => {
                s_e = nr - 1;
                s_s = nr;
            }
            _ => ()
        };
        let r1 = PartRange {
            x: self.x,
            x_e: x_e,
            m: self.m,
            m_e: m_e,
            a: self.a,
            a_e: a_e,
            s: self.s,
            s_e: s_e,
        };
        let r2 = PartRange {
            x: x_s,
            x_e: self.x_e,
            m: m_s,
            m_e: self.m_e,
            a: a_s,
            a_e: self.a_e,
            s: s_s,
            s_e: self.s_e,
        };
        return (Some(r1), Some(r2));
    }

    // 2,8 > 5
    // 6-8, 2-5
    fn split_lesser(self, part_to_check: &str, nr: u64) -> (Option<PartRange>, Option<PartRange>) {
        let mut x_s = self.x;
        let mut x_e = self.x_e;

        let mut m_s = self.m;
        let mut m_e = self.m_e;

        let mut a_s = self.a;
        let mut a_e = self.a_e;

        let mut s_s = self.s;
        let mut s_e = self.s_e;

        match part_to_check {
            "x" => {
                x_s = nr + 1;
                x_e = nr;
            }
            "m" => {
                m_s = nr + 1;
                m_e = nr;
            }
            "a" => {
                a_s = nr + 1;
                a_e = nr;
            }
            "s" => {
                s_s = nr + 1;
                s_e = nr;
            }
            _ => ()
        };
        let r1 = PartRange {
            x: x_s,
            x_e: self.x_e,
            m: m_s,
            m_e: self.m_e,
            a: a_s,
            a_e: self.a_e,
            s: s_s,
            s_e: self.s_e,
        };
        let r2 = PartRange {
            x: self.x,
            x_e: x_e,
            m: self.m,
            m_e: m_e,
            a: self.a,
            a_e: a_e,
            s: self.s,
            s_e: s_e,
        };
        return (Some(r1), Some(r2));
    }
}


pub fn part_two(input: &str) -> Option<u64> {
    let mut rule_mode = true;
    let mut function_map: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            rule_mode = false;
            return;
        }
        if rule_mode {
            let mut func_rule = l.split(['{', '}']);
            let func = func_rule.next().unwrap();
            //println!("func: {}", func);
            let prob_vec: Vec<&str> = func_rule.next().unwrap().split(',').collect();
            function_map.insert(func, prob_vec);
        } else {
            return;
        }
    });
    let start = PartRange {
        x: 1,
        x_e: 4000,
        m: 1,
        m_e: 4000,
        a: 1,
        a_e: 4000,
        s: 1,
        s_e: 4000,
    };
    Some(get_iter_rules(start, &function_map, "in".to_string()))
}

fn get_iter_rules(node: PartRange, function_map: &HashMap<&str, Vec<&str>>, current_function: String) -> u64 {
    if current_function == "A" {
        return node.get_possibilities();
    }
    if current_function == "R" {
        return 0;
    }
    let rules = function_map.get(current_function.as_str()).unwrap();
    let mut node = Some(node);
    let mut res = 0;
    for i in 0..rules.len() {
        if node.is_none() {
            return res;
        }
        let rule = rules[i];
        if rule.contains(':') {
            let splitted_res = split_range(rule.to_string(), node.clone().unwrap());
            if splitted_res.0.is_some() {
                let new_prob = rule.split_once(':').unwrap().1;
                res += get_iter_rules(splitted_res.0.unwrap(), function_map, new_prob.to_string())
            }
            node = splitted_res.1;
        } else {
            return res + get_iter_rules(node.clone().unwrap(), function_map, rule.to_string());
        }
    };
    panic!("out of loop")
}

fn eval_rule_range(part_to_check: &str, nr: u64, part: PartRange, bigger: bool) -> (Option<PartRange>, Option<PartRange>) {
    let nr_start = match part_to_check {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => 0
    };
    let nr_end = match part_to_check {
        "x" => part.x_e,
        "m" => part.m_e,
        "a" => part.a_e,
        "s" => part.s_e,
        _ => 0
    };
    if bigger {
        if nr_end < nr {
            return (Some(part.clone()), None);
        }
        if nr_start < nr {
            return part.split_bigger(part_to_check, nr);
        }
        return (None, Some(part.clone()));
    }
    if nr_start > nr {
        return (Some(part.clone()), None);
    }
    if nr_end > nr {
        return part.split_lesser(part_to_check, nr);
    }
    return (None, Some(part.clone()));
}

fn split_range(rule: String, node: PartRange) -> (Option<PartRange>, Option<PartRange>) {
    if rule.contains('<') {
        let prob = rule.split_once(':').unwrap();
        let m = prob.0.split_once('<').unwrap();
        return eval_rule_range(m.0, m.1.parse().unwrap(), node, true);
    }
    let prob = rule.split_once(':').unwrap();
    let m = prob.0.split_once('>').unwrap();
    return eval_rule_range(m.0, m.1.parse().unwrap(), node, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
