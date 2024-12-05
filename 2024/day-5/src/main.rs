use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    println!("Part 1: {}", SafetyManual::parse(input).part_1());
}

#[derive(Debug)]
pub struct SafetyManual {
    pub ordering_rules: HashMap<usize, Vec<usize>>,
    pub updates: Vec<Update>,
}

#[derive(Debug)]
pub struct Update {
    pub pages: Vec<usize>,
}

impl Update {
    pub fn is_ordered(&self, ordering_rules: &HashMap<usize, Vec<usize>>) -> bool {
        self.pages
            .iter()
            .rev()
            .enumerate()
            .all(|(current_page_idx, current_page)| {
                ordering_rules
                    .get(current_page)
                    .map_or(false, |lookup_list| {
                        lookup_list.iter().all(|item| {
                            !self.pages[0..self.pages.len() - current_page_idx - 1].contains(item)
                        })
                    })
            })
    }
}

impl SafetyManual {
    pub fn parse(input: &str) -> Self {
        let mut input = input.split("\n\n");

        let ordering_rules = input.next().unwrap().lines().fold(
            HashMap::new(),
            |mut map: HashMap<usize, Vec<usize>>, line| {
                let mut parts = line.split("|");

                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();

                map.entry(x).or_default().push(y);

                map
            },
        );

        let updates = input
            .next()
            .unwrap()
            .lines()
            .map(|line| Update {
                pages: line.split(",").map(|part| part.parse().unwrap()).collect(),
            })
            .collect();

        SafetyManual {
            ordering_rules,
            updates,
        }
    }

    pub fn part_1(&self) -> usize {
        self.updates
            .iter()
            .filter(|update| update.is_ordered(&self.ordering_rules))
            .map(|update| update.pages[(update.pages.len() as f32 / 2.0).floor() as usize])
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE: &str = "47|53\n\
                           97|13\n\
                           97|61\n\
                           97|47\n\
                           75|29\n\
                           61|13\n\
                           75|53\n\
                           29|13\n\
                           97|29\n\
                           53|29\n\
                           61|53\n\
                           97|53\n\
                           61|29\n\
                           47|13\n\
                           75|47\n\
                           97|75\n\
                           47|61\n\
                           75|61\n\
                           47|29\n\
                           75|13\n\
                           53|13\n\
                           \n\
                           75,47,61,53,29\n\
                           97,61,53,29,13\n\
                           75,29,13\n\
                           75,97,47,61,53\n\
                           61,13,29\n\
                           97,13,75,29,47";

    #[test]
    fn should_verify_order() {
        let safety_manual = SafetyManual::parse(EXAMPLE);

        assert!(safety_manual.updates[0].is_ordered(&safety_manual.ordering_rules));
        assert!(safety_manual.updates[1].is_ordered(&safety_manual.ordering_rules));
        assert!(safety_manual.updates[2].is_ordered(&safety_manual.ordering_rules));
        assert!(!safety_manual.updates[3].is_ordered(&safety_manual.ordering_rules));
        assert!(!safety_manual.updates[4].is_ordered(&safety_manual.ordering_rules));
        assert!(!safety_manual.updates[5].is_ordered(&safety_manual.ordering_rules));
    }

    #[test]
    fn should_sum_middle_numbers() {
        let safety_manual = SafetyManual::parse(EXAMPLE);
        assert_eq!(safety_manual.part_1(), 143);
    }
}
