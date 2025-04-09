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
    target_chars: &[char], // The word we are looking for (e.g., ['X','M','A','S'], or ['M','A','S'])
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
/// This function gets the cross word coordinates around a given point `p`.
/// It generates paths of `word_len` centered at `p`, extending symmetrically
/// along the axes defined by DIRECTIONS (North-South, East-West, NE-SW, NW-SE).
/// Returns None if no such paths are possible within the bounds or if word_len is invalid.
fn get_cross_word_coordinates(
    range: InputRange,
    p: Point, // The center point
    word_len: usize,
) -> Option<Vec<Vec<Point>>> {
    // Constants for opposite direction pairs
    const CARDINAL_DIRECTIONS: [((i32, i32), (i32, i32)); 2] =
        [((0, 1), (0, -1)), ((1, 0), (-1, 0))]; // N-S, E-W (using X right, Y down)
    const ORDINAL_DIRECTIONS: [((i32, i32), (i32, i32)); 2] =
        [((1, 1), (-1, -1)), ((-1, 1), (1, -1))]; // SE-NW, NE-SW
    const DIRECTIONS: [((i32, i32), (i32, i32)); 4] = [
        CARDINAL_DIRECTIONS[0], // N-S
        CARDINAL_DIRECTIONS[1], // E-W
        ORDINAL_DIRECTIONS[0],  // SE-NW
        ORDINAL_DIRECTIONS[1],  // NE-SW
    ];

    // Basic validation
    if word_len == 0 {
        eprintln!("Error: get_cross_word_coordinates called with word_len 0.");
        return None;
    }
    if word_len % 2 != 1 {
        // Word must have odd length to be centered on a single point 'p'
        eprintln!(
            "Warning: get_cross_word_coordinates requires odd word_len (got {}). Cannot center.",
            word_len
        );
        return None;
    }
    // Check if the center point itself is valid first
    if p.x < 0 || p.y < 0 || p.x > range.x || p.y > range.y {
        return None; // Center point is out of bounds
    }

    // Calculate steps needed from center to each end
    // e.g., for len 3 -> half_steps = 1; for len 5 -> half_steps = 2
    let half_steps = (word_len / 2) as i32;

    // Handle the edge case where word_len is 1
    if half_steps == 0 {
        // The path is just the center point itself
        return Some(vec![vec![p]]);
    }

    let mut cross_word_paths: Vec<Vec<Point>> = Vec::new();

    // Iterate through the pairs of opposite directions (axes)
    for (dir1, dir2) in DIRECTIONS {
        let (dx1, dy1) = dir1; // Direction 1 (e.g., North)
        let (dx2, dy2) = dir2; // Opposite Direction 2 (e.g., South)

        // Calculate the end points of the potential word path
        let end1_x = p.x + half_steps * dx1;
        let end1_y = p.y + half_steps * dy1;
        let end2_x = p.x + half_steps * dx2;
        let end2_y = p.y + half_steps * dy2;

        // Check if *both* end points are within the grid boundaries
        let end1_in_bounds = end1_x >= 0 && end1_x <= range.x && end1_y >= 0 && end1_y <= range.y;
        let end2_in_bounds = end2_x >= 0 && end2_x <= range.x && end2_y >= 0 && end2_y <= range.y;

        if end1_in_bounds && end2_in_bounds {
            // Both ends are valid, generate the path between them using the existing helper
            let end1 = Point {
                x: end1_x,
                y: end1_y,
            };
            let end2 = Point {
                x: end2_x,
                y: end2_y,
            };

            // This function generates points from end1 to end2, inclusive.
            // Since end1 and end2 are symmetrical around 'p', this path will contain 'p'
            // and have the correct total length.
            let path = get_points_between_inclusive(end1, end2);

            // Sanity check the path length - should always match word_len if logic is correct
            if path.len() == word_len {
                cross_word_paths.push(path);
            } else {
                // This suggests a potential issue in calculations or get_points_between_inclusive
                eprintln!(
                    "Warning: Generated cross path length mismatch. Center={:?}, End1={:?}, End2={:?}. Expected len {}, got {}. HalfSteps={}",
                    p, end1, end2, word_len, path.len(), half_steps
                );
            }
        }
        // If either end point is out of bounds, this axis doesn't yield a valid path,
        // so we simply ignore it and continue to the next direction pair.
    }

    // Return the collected paths if any were found, otherwise None
    if cross_word_paths.is_empty() {
        None
    } else {
        Some(cross_word_paths)
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
    // Define the target word and its properties
    const TARGET_WORD: &str = "MAS"; // The word forming the arms of the X
    const TARGET_CHAR: char = 'A'; // The center character of the X
    let target_chars: Vec<char> = TARGET_WORD.chars().collect(); // ['M', 'A', 'S']
    let target_chars_rev: Vec<char> = TARGET_WORD.chars().rev().collect(); // ['S', 'A', 'M']
    let word_len = TARGET_WORD.len(); // Should be 3

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

    // Find all potential center points ('A')
    let center_points = match get_char_points(TARGET_CHAR, &lines) {
        Some(points) => points,
        None => return Some(0), // No center points found, so no X-MAS structures
    };

    let mut x_mas_count = 0; // Initialize count for complete X-MAS structures

    // Iterate through each potential center point ('A')
    for center_point in center_points {
        // Get all possible symmetrical paths (arms) centered at this point
        if let Some(potential_arms) = get_cross_word_coordinates(range, center_point, word_len) {
            // An X-MAS structure requires TWO valid DIAGONAL arms.
            // We need to count how many of the diagonal arms match "MAS" or "SAM".

            let mut valid_diagonal_arms_count = 0;

            // The order of arms returned by get_cross_word_coordinates corresponds
            // to the DIRECTIONS constant used inside it:
            // Index 0: N-S (Cardinal)
            // Index 1: E-W (Cardinal)
            // Index 2: SE-NW (Ordinal/Diagonal)
            // Index 3: NE-SW (Ordinal/Diagonal)

            for (i, arm_path) in potential_arms.iter().enumerate() {
                // --- Focus only on the diagonal arms (indices 2 and 3) ---
                if i >= 2 {
                    // Check if this diagonal arm matches "MAS" or "SAM"
                    let matches_fwd =
                        verify_path_matches_word(arm_path, &lines, range, &target_chars);
                    let matches_rev =
                        verify_path_matches_word(arm_path, &lines, range, &target_chars_rev);

                    if matches_fwd || matches_rev {
                        valid_diagonal_arms_count += 1;
                    }
                }
            }

            // If we found exactly two valid diagonal arms intersecting at this center_point,
            // then we have found one complete X-MAS structure.
            if valid_diagonal_arms_count == 2 {
                x_mas_count += 1;
            }
            // Optional sanity check: if count is > 2, something is weird.
            // else if valid_diagonal_arms_count > 2 {
            //     eprintln!("Warning: Found {} diagonal arms for center {:?}. Expected 0 or 2.", valid_diagonal_arms_count, center_point);
            // }
        }
    }

    Some(x_mas_count) // Return the total count of X-MAS structures
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
