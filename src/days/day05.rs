use tracing::{debug, info};

use crate::{Solution, SolutionPair};
use std::{collections::HashMap, fs::read_to_string};

struct IndexValueMap {
    index_to_value: HashMap<usize, usize>,     // Map index to value
    pub value_to_index: HashMap<usize, usize>, // Map value to index
    order: Vec<usize>,                         // Logical order of indices
}

impl IndexValueMap {
    // Create a new IndexValueMap from a vector of elements
    fn new(elements: Vec<usize>) -> Self {
        let mut index_to_value = HashMap::new();
        let mut value_to_index = HashMap::new();
        for (i, &value) in elements.iter().enumerate() {
            index_to_value.insert(i, value);
            value_to_index.insert(value, i);
        }
        let order: Vec<usize> = (0..elements.len()).collect();
        Self {
            index_to_value,
            value_to_index,
            order,
        }
    }

    // Move an element from one index to another
    fn move_element(&mut self, from_idx: usize, to_idx: usize) {
        debug!("Moving from {} to {}", from_idx, to_idx);
        debug!(
            "Before {:?} {:?} {:?} ",
            self.index_to_value, self.value_to_index, self.order
        );
        let from_value = self.index_to_value.get(&from_idx).unwrap().clone();
        let to_value = self.index_to_value.get(&to_idx).unwrap().clone();

        if let Some(pos) = self.order.iter().position(|&x| x == from_idx) {
            self.order.remove(pos); // Remove the index from the current position
            self.order.insert(to_idx, from_idx); // Insert it at the new position
        }
        self.value_to_index
            .entry(from_value)
            .and_modify(|x| *x = to_idx);
        self.value_to_index
            .entry(to_value)
            .and_modify(|x| *x = from_idx);
        self.index_to_value
            .entry(from_idx)
            .and_modify(|x| *x = to_value);
        self.index_to_value
            .entry(to_idx)
            .and_modify(|x| *x = from_value);

        debug!(
            "After  {:?} {:?} {:?} ",
            self.index_to_value, self.value_to_index, self.order
        );
    }

    // Update the mappings when a move operation changes the logical order
    fn update_mappings(&mut self) {
        self.value_to_index.clear();
        for &idx in &self.order {
            if let Some(&value) = self.index_to_value.get(&idx) {
                self.value_to_index.insert(value, idx);
            }
        }
    }

    // Get elements in their current logical order
    fn get_elements(&self) -> Vec<usize> {
        let mut output = vec![];
        for i in 0..self.index_to_value.len() {
            output.push(*self.index_to_value.get(&i).unwrap());
        };
        output
    }
    fn get_middle_value(&self) -> Option<usize> {
        if self.order.is_empty() {
            return None; // Handle empty case
        }
        let middle_idx = self.order.len() / 2; // Calculate middle index
        self.order
            .get(middle_idx) // Get the logical middle index
            .and_then(|&idx| self.index_to_value.get(&idx).cloned()) // Fetch value
    }
}

#[derive(Debug)]
struct RuleMap {
    rules: HashMap<usize, Vec<usize>>,
}

impl RuleMap {
    fn new(rule_str: &str) -> Self {
        let mut rm = RuleMap {
            rules: HashMap::new(),
        };
        for r in rule_str.split_ascii_whitespace() {
            // debug!("Rule {}", r);
            let (a, b) = r.split_once("|").unwrap();
            let ia = a.parse::<usize>().unwrap();
            let ib = b.parse::<usize>().unwrap();

            let entry_ = rm.rules.entry(ia).or_insert(vec![]);
            entry_.push(ib);
        }
        rm
    }
}

fn check_manual(manual: &str, rulemap: &RuleMap) -> bool {
    let mut page_inserts = HashMap::new();
    for (counter, page) in manual.split(",").enumerate() {
        let p = page.parse::<usize>().unwrap();
        page_inserts.insert(p, counter);
        // debug!("Page {} {}", p, counter);
        if let Some(rules_for_page) = rulemap.rules.get(&p) {
            // debug!("Rules exist for page {} {:?}", p, rules_for_page);
            for r in rules_for_page {
                // debug!("Checking if page {} present", r);
                if let Some(pi) = page_inserts.get(&r) {
                    // debug!("page {} present with value {} {}", r, pi, &counter > pi);
                    if &counter > pi {
                        return false;
                    }
                }
            }
        };
    }
    true
}

fn middle_value(manual: &str) -> usize {
    let pages: Vec<&str> = manual.split(",").collect();
    debug!("{} {}", ((pages.len() - 1) / 2), pages.len());
    let p = pages[((pages.len() - 1) / 2)];
    p.parse::<usize>().unwrap()
}

pub fn solve() -> SolutionPair {
    let content = read_to_string("./input/day5.txt").unwrap_or_default();
    // let content = read_to_string("./input/day5.txt").unwrap_or_default();
    let (rules, pages) = content
        .split_once(
            "

",
        )
        .unwrap();
    debug!("rules {:?} pages {}", rules, pages);
    let rulemap = RuleMap::new(rules);
    debug!("rulemap {:?}", rulemap);
    let mut total = 0;
    let mut total2 = 0;
    let mut wrong_manuals = vec![];
    for (i, manual) in pages.split_ascii_whitespace().enumerate() {
        if check_manual(manual, &rulemap) {
            info!("Valid {} {}", i, manual);
            let mv = middle_value(manual);

            total += mv;
        } else {
            info!("Invalid {} {}", i, manual);
            wrong_manuals.push(manual);
        }
    }

    for wm in wrong_manuals {
        debug!("Fixing wrong manual {}", wm);
        let mv = fix_manual(wm, &rulemap);
        total2 += middle_value(&mv);
    }

    let sol1: u64 = total as u64;
    let sol2: u64 = total2 as u64;

    (Solution::from(sol1), Solution::from(sol2))
}

fn fix_manual(manual: &str, rulemap: &RuleMap) -> String {
    let pages: Vec<usize> = manual
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut indexer = IndexValueMap::new(pages.clone());
    for p in pages {
        let mut counter = indexer.value_to_index.get(&p).unwrap().clone();
        debug!("Page {} {}", p, counter);
        if let Some(rules_for_page) = rulemap.rules.get(&p) {
            debug!("Rules exist for page {} {:?}", p, rules_for_page);
            for r in rules_for_page {
                // debug!("Checking if page {} present", r);
                if let Some(pi) = indexer.value_to_index.get(&r) {
                    debug!("page {} present with value {} {}", r, pi, &counter > pi);
                    if &counter > pi {
                        debug!("Invalid! {} ({}) must be before {} ({})", p, counter, r, pi);
                        debug!("Old {:?}", indexer.get_elements());
                        indexer.move_element(counter, *pi);
                        debug!("Counter {} updating for page {}", counter, p);
                        counter = indexer.value_to_index.get(&p).unwrap().clone();
                        debug!("New {:?}", indexer.get_elements());
                        debug!("Counter updated! {}", counter);
                    }
                }
            }
        };
    }

    let mut fixed_str = format!("{:?}", indexer.get_elements())
        .replace("[", "")
        .replace("]", "")
        .replace(" ", "");
    debug!("Fixed {} -> {:?}", manual, fixed_str);

    while !check_manual(&fixed_str, rulemap) {
        debug!("Still not fixed wah");
        fixed_str = fix_manual(&fixed_str, rulemap)
    }
    fixed_str
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        tracing_subscriber::fmt::init();
        let manual = "97,13,75,29,47";
        let rulesstr = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
        let rulemap = RuleMap::new(&rulesstr);
        debug!("Testing {}", manual);
        let fixed = fix_manual(manual, &rulemap);
        debug!("Fixed {} -> {:?}", manual, fixed);
    }
}
