use std::collections::HashMap;

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

fn is_update_valid(updates: &[usize], ordering_map: &HashMap<usize, Vec<usize>>) -> bool {
    let page_indices: HashMap<usize, usize> = updates
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

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
