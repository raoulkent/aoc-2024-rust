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

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
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
