advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
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

    Some(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .filter(|line| is_safe(line))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    fn is_safe(levels: &[i32]) -> bool {
        if levels.len() < 2 {
            return true;
        }

        let diffs: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();
        let increasing = diffs[0] > 0;

        diffs.iter().all(|&diff| {
            (increasing && diff > 0) || (!increasing && diff < 0) && (1..=3).contains(&diff.abs())
        })
    }

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

    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|report| is_report_safe(report))
        .count()
        .try_into()
        .ok()
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
