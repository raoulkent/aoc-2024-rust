use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

fn split_input(input: &str) -> (&str, &str) {
    input
        .split_once("\n\n")
        .expect("Input should contain two sections separated by '\\n\\n'")
}

fn parse_page_ordering(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| match line.split_once('|') {
            Some((first_str, second_str)) => {
                match (first_str.parse::<usize>(), second_str.parse::<usize>()) {
                    (Ok(first), Ok(second)) => (first, second),
                    (Err(_), Ok(_)) => panic!("Failed to parse first number {}", first_str),
                    (Ok(_), Err(_)) => panic!("Failed to parse second number {}", second_str),
                    (Err(_), Err(_)) => {
                        panic!("Failed to parse both numbers {}, {}", first_str, second_str)
                    }
                }
            }
            None => panic!("Invalid format, no '|' found for line; {}", line),
        })
        .collect()
}

fn parse_updates(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {

            line.split(',')
                .map(|num_str| {
                    num_str
                        .trim()
                        .parse::<usize>()
                        .expect("Failed to parse number: Input should contain only comma-separated numbers per line.")
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn calculate_page_ordering(input: Vec<(usize, usize)>) -> HashMap<usize, Vec<usize>> {
    input.iter().fold(
        // HashMap::<usize, Vec<usize>>(),
        HashMap::new(),
        |mut acc_map, &(first, second)| {
            acc_map.entry(first).or_default().push(second);
            // acc_map.entry(first).or_insert_with(Vec::new).push(second);
            acc_map
        },
    )
}

fn is_update_valid(update: &[usize], ordering_map: &HashMap<usize, Vec<usize>>) -> bool {
    let page_indices: HashMap<usize, usize> = update
        .iter()
        .enumerate()
        .map(|(index, &page)| (page, index))
        .collect();
    let any_violation_found = ordering_map
        .iter()
        .flat_map(|(&page_x, pages_after_x)| {
            pages_after_x.iter().map(move |&page_y| (page_x, page_y))
        })
        .any(
            |(page_x, page_y)| match (page_indices.get(&page_x), page_indices.get(&page_y)) {
                (Some(&index_x), Some(&index_y)) => index_x > index_y,
                _ => false,
            },
        );
    !any_violation_found
}

fn correct_update(update: &[usize], ordering_map: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    // 1. Find the unique pages for this update
    let pages_in_update: HashSet<usize> = update.iter().cloned().collect();
    if pages_in_update.is_empty() {
        return Vec::new();
    }

    // 2. Calculate the in-degrees for the pages
    let mut in_degrees: HashMap<usize, usize> = pages_in_update.iter().map(|&p| (p, 0)).collect();
    // Create adjacency list for all dependent pages
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();

    // Iterate across all keys in the ordering_map and check if they are in the pages_in_update
    for (&page_x, pages_after_x) in ordering_map {
        if pages_in_update.contains(&page_x) {
            for &page_y in pages_after_x {
                if pages_in_update.contains(&page_y) {
                    // Increment the in_degree for the page_y
                    *in_degrees.entry(page_y).or_insert(0) += 1;
                    // Add the edge X -> Y to the adjacency list
                    adj.entry(page_x).or_default().push(page_y);
                }
            }
        }
    }

    // 3. Initialise the queue with those pages that have an in_degree of 0
    let mut queue: VecDeque<usize> = pages_in_update
        .iter()
        .filter(|&&p| in_degrees.get(&p).copied().unwrap_or(0) == 0)
        .cloned()
        .collect();

    let mut corrected: Vec<usize> = Vec::with_capacity(update.len());

    // 4. Process the queue using Kahns Algorithm
    while let Some(page_p) = queue.pop_front() {
        corrected.push(page_p);

        // For each neighbour N of page P
        if let Some(neighours) = adj.get(&page_p) {
            for &page_n in neighours {
                if let Some(degree) = in_degrees.get_mut(&page_n) {
                    *degree -= 1;
                    // If the degree becomes 0, add neighbour to queue
                    if *degree == 0 {
                        queue.push_back(page_n);
                    }
                }
            }
        }
    }

    // L ← Empty list that will contain the sorted elements
    // S ← Set of all nodes with no incoming edge
    //
    // while S is not empty do
    //     remove a node n from S
    //     add n to L
    //     for each node m with an edge e from n to m do
    //         remove edge e from the graph
    //         if m has no other incoming edges then
    //             insert m into S
    //
    // if graph has edges then
    //     return error   (graph has at least one cycle)
    // else
    //     return L   (a topologically sorted order)

    corrected
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules_str, updates_str) = split_input(input);

    let parsed_rules = parse_page_ordering(rules_str);
    let ordering_map = calculate_page_ordering(parsed_rules);

    let updates = parse_updates(updates_str);

    let total_middle_page_sum = updates
        .iter()
        .filter(|update_vec| !update_vec.is_empty() && is_update_valid(update_vec, &ordering_map))
        .map(|valid_update_vec| {
            let middle_index = (valid_update_vec.len() - 1) / 2;
            valid_update_vec[middle_index] as u64
        })
        .sum();

    Some(total_middle_page_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules_str, updates_str) = split_input(input);

    let parsed_rules = parse_page_ordering(rules_str);
    let ordering_map = calculate_page_ordering(parsed_rules);

    let updates = parse_updates(updates_str);

    let total_fixed_middle_page_sum = updates
        .iter()
        .filter(|update_vec| !update_vec.is_empty() && !is_update_valid(update_vec, &ordering_map))
        .map(|invalid_update_vec| {
            let corrected_vec = correct_update(invalid_update_vec, &ordering_map);

            if corrected_vec.is_empty() {
                0
            } else {
                let middle_index = (corrected_vec.len() - 1) / 2;
                corrected_vec[middle_index] as u64
            }
        })
        .sum();

    Some(total_fixed_middle_page_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
