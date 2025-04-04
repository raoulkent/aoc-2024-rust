advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct InputRange {
    x: i32,
    y: i32,
}

// Helper to safely get a character from the grid
fn get_char_at(lines: &[&str], point: Point, range: InputRange) -> Option<char> {
    // Basic bounds check using the provided range
    if point.x < 0 || point.y < 0 || point.x > range.x || point.y > range.y {
        return None;
    }

    // Convert point coordinates to usize for indexing
    let y_idx = point.y as usize;
    let x_idx = point.x as usize;

    // Access the line and then the character
    lines.get(y_idx)?.chars().nth(x_idx)
}

fn verify_path_matches_word(
    path: &[Point],        // Slice of points representing the path
    lines: &[&str],        // Grid lines for character lookup
    range: InputRange,     // Grid boundaries needed for safe lookup
    target_chars: &[char], // The word we are looking for (e.g., ['X','M','A','S'])
) -> bool {
    if path.len() != target_chars.len() {
        return false;
    }

    path.iter()
        .zip(target_chars.iter())
        .all(|(point, target_char)| get_char_at(lines, *point, range) == Some(*target_char))
}

fn get_points_between_inclusive(start: Point, end: Point) -> Vec<Point> {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let num_steps = dx.abs().max(dy.abs());
    let capacity = (num_steps + 1) as usize;
    let mut points = Vec::with_capacity(capacity);
    let step_x = dx.signum();
    let step_y = dy.signum();

    for i in 0..=num_steps {
        let current_point = Point {
            x: start.x + i * step_x,
            y: start.y + i * step_y,
        };
        points.push(current_point);
    }
    points
}

fn get_possible_word_coordinates(
    range: InputRange,
    point: Point,
    word_len: usize,
) -> Option<Vec<Vec<Point>>> {
    // Make word_len a parameter
    if word_len == 0 {
        return None;
    } // Handle zero length word
    let steps: i32 = (word_len - 1) as i32;

    let mut possible_word_coordinates: Vec<Vec<Point>> = Vec::new();

    if point.x < 0 || point.y < 0 || point.x > range.x || point.y > range.y {
        return None;
    }

    const DIRECTIONS: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    for (dx, dy) in DIRECTIONS {
        let end_x = point.x + steps * dx;
        let end_y = point.y + steps * dy;

        let end_in_bounds = end_x >= 0 && end_x <= range.x && end_y >= 0 && end_y <= range.y;

        if end_in_bounds {
            let end_point = Point { x: end_x, y: end_y };
            let coords = get_points_between_inclusive(point, end_point);

            if coords.len() == word_len {
                possible_word_coordinates.push(coords);
            } else {
                eprintln!(
                     "Warning: Coordinate generation mismatch for start={:?}, end={:?}. Expected len {}, got {}. Steps={}, dx={}, dy={}",
                     point, end_point, word_len, coords.len(), steps, dx, dy
                 );
            }
        }
    }
    // Return None if no valid coordinates were found, Some otherwise
    if possible_word_coordinates.is_empty() {
        None
    } else {
        Some(possible_word_coordinates)
    }
}

fn get_char_points(c: char, lines: &[&str]) -> Option<Vec<Point>> {
    let points: Vec<Point> = lines
        .iter()
        .enumerate()
        .flat_map(|(y_usize, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, char)| char == &c)
                .map(move |(x_usize, _)| Point {
                    x: x_usize as i32,
                    y: y_usize as i32,
                })
        })
        .collect();

    if points.is_empty() {
        None
    } else {
        Some(points)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // Define the target word
    const TARGET_WORD: &str = "XMAS"; // Example word
    let target_chars: Vec<char> = TARGET_WORD.chars().collect();
    let word_len = target_chars.len();

    // Process the input into lines
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() || lines[0].is_empty() {
        return None; // Handle empty input
    }

    // Determine grid dimensions
    let range = InputRange {
        x: (lines[0].len() - 1) as i32,
        y: (lines.len() - 1) as i32,
    };

    // Find all starting points (locations of the first character)
    let first_char = match target_chars.first() {
        Some(c) => *c,
        None => return Some(0), // Or handle empty word case appropriately
    };

    let start_points = match get_char_points(first_char, &lines) {
        Some(points) => points,
        None => return Some(0), // First character not found, so word cannot exist
    };

    let mut found_count = 0;

    // Iterate through each potential starting point
    for start_point in start_points {
        // Get all possible coordinate paths starting from this point
        if let Some(possible_paths) = get_possible_word_coordinates(range, start_point, word_len) {
            // Check each path against the target word
            for path in possible_paths {
                if verify_path_matches_word(&path, &lines, range, &target_chars) {
                    found_count += 1;
                }
            }
        }
    }

    Some(found_count) // Return the total count of found words
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
