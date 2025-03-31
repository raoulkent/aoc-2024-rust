use regex::Regex;

advent_of_code::solution!(3);

fn get_muls(s: &str) -> Vec<(i32, i32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result: Vec<(i32, i32)> = Vec::new();

    for cap in re.captures_iter(s) {
        let first = cap[1].parse::<i32>().unwrap();
        let second = cap[2].parse::<i32>().unwrap();
        result.push((first, second));
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let mul_pairs = get_muls(input);

    Some(
        mul_pairs
            .iter()
            .map(|pair| (pair.0 as u64) * (pair.1 as u64))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut sum = 0;
    let mut enabled = true;

    for cap in re.captures_iter(input) {
        if let Some(m) = cap.get(1) {
            match m.as_str() {
                s if s.starts_with("mul") => {
                    if enabled {
                        let first = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
                        let second = cap.get(3).unwrap().as_str().parse::<u64>().unwrap();
                        sum += first * second;
                    }
                }
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {}
            }
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
