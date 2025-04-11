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
        .lines() // 1. Iterate over each line in the input string
        .map(|line| {
            // 2. Process each line
            line.split(',') // 3. Split the line into parts based on the comma
                .map(|num_str| { // 4. Process each part (string representation of a number)
                    num_str
                        .trim() // 5. Remove leading/trailing whitespace (important!)
                        .parse::<usize>() // 6. Attempt to parse the string part into a usize
                        .expect("Failed to parse number: Input should contain only comma-separated numbers per line.") // 7. Handle potential parsing errors (panics on error)
                })
                .collect::<Vec<usize>>() // 8. Collect the parsed numbers for this line into a Vec<usize>
        })
        .collect::<Vec<Vec<usize>>>() // 9. Collect all the Vec<usize> (one for each line) into the final Vec<Vec<usize>>
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
