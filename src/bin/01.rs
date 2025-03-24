use std::{collections::HashMap, iter::Map};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    if left.len() != right.len() {
        return None;
    }

    let mut total_distance = 0;

    for i in 0..left.len() {
        let distance = i32::abs(left[i] - right[i]) as u64;
        total_distance += distance
    }

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);

    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    for v in right {
        frequency_map.entry(v).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut sum: u64 = 0;

    for v in left {
        if let Some(map_value) = frequency_map.get(&v) {
            sum += v as u64 * *map_value as u64;
        }
    }

    Some(sum)
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

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        if let Some((left, right)) = line.split_once("   ") {
            if let (Ok(left_num), Ok(right_num)) =
                (left.trim().parse::<i32>(), right.trim().parse::<i32>())
            {
                left_list.push(left_num);
                right_list.push(right_num);
            }
        }
    }

    (left_list, right_list)
}

#[cfg(test)]
mod tests_parser {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        let (left, right) = parse_input(input);
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }
}
