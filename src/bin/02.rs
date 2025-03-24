advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .filter(|line| is_safe(line))
            .count() as u64,
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn is_safe(s: &str) -> bool {
    let collection: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    if collection.len() <= 1 {
        return true;
    }

    let is_increasing = collection[1] > collection[0];

    collection.windows(2).all(|w| {
        let diff = w[1] - w[0];
        let abs_diff = diff.abs();

        let monotonic_check = if is_increasing { diff > 0 } else { diff < 0 };

        monotonic_check && (1..=3).contains(&abs_diff)
    })
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
