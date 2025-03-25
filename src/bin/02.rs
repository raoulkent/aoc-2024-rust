advent_of_code::solution!(2);

fn is_safe(input: impl AsRef<[i32]>) -> bool {
    let collection = input.as_ref();
    if collection.len() < 2 {
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

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .filter(|line| {
                let collection: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse().expect("Failed to parse number"))
                    .collect();
                is_safe(&collection)
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    fn is_report_safe(report: &str) -> bool {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_safe(&levels) {
            return true;
        }

        for i in 0..levels.len() {
            let mut temp_levels = levels.clone();
            temp_levels.remove(i);
            if is_safe(&temp_levels) {
                return true;
            }
        }

        false
    }

    let result = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|report| is_report_safe(report))
        .count() as u64;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
