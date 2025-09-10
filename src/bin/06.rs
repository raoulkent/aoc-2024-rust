use std::collections::HashSet;
use std::ops::Add;

advent_of_code::solution!(6);

fn find_starting_position(input: &str) -> Option<(Coordinate, Direction)> {
    input.lines().enumerate().find_map(|(y, line)| {
        line.char_indices().find_map(|(x, c)| {
            Direction::try_from(c).ok().map(|dir| {
                (
                    Coordinate {
                        x: x as isize,
                        y: y as isize,
                    },
                    dir,
                )
            })
        })
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> Coordinate {
        match self {
            Direction::Up => Coordinate { x: 0, y: -1 },
            Direction::Down => Coordinate { x: 0, y: 1 },
            Direction::Left => Coordinate { x: -1, y: 0 },
            Direction::Right => Coordinate { x: 1, y: 0 },
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Make CellState hold Direction in the Guard variant
// Add derives for usability
#[derive(Debug, Clone, PartialEq, Eq)]
enum CellState {
    Guard(Direction), // Stores the direction the guard is facing
    Visited,
    Unvisited,
    Obstructed,
}

const GUARD_CHARS: &[char] = &['^', 'v', '<', '>'];
const UNOBSTRUCTED_CHAR: char = '.'; // Example: '.' means Unvisited
const OBSTRUCTED_CHAR: char = '#'; // Example: '#' means Obstructed

#[derive(Debug, Clone)]
struct GuardMap {
    width: usize,
    height: usize,
    cells: Vec<CellState>,
}

impl GuardMap {
    pub fn from_input(input: &str) -> Option<Self> {
        let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();
        if lines.is_empty() {
            return None;
        }

        let height = lines.len();
        let width = lines[0].chars().count();
        if width == 0 {
            return None;
        }

        // Ensure that all lines have the same numeber of chars
        if lines.iter().any(|line| line.chars().count() != width) {
            eprintln!("Warning; Inconsisent input line widths")
        }

        let mut cells = Vec::with_capacity(height * width);

        for line in lines.iter() {
            let chars: Vec<char> = line.chars().collect();
            for x in 0..width {
                let c = chars.get(x).copied().unwrap_or(' ');

                let state = match c {
                    UNOBSTRUCTED_CHAR => CellState::Unvisited,
                    OBSTRUCTED_CHAR => CellState::Obstructed,
                    guard_char if GUARD_CHARS.contains(&guard_char) => {
                        let dir = Direction::try_from(guard_char).unwrap();
                        CellState::Guard(dir)
                    }
                    _ => CellState::Unvisited, // Default for unknown characters
                };
                cells.push(state);
            }
        }

        Some(GuardMap {
            width,
            height,
            cells,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = GuardMap::from_input(input)?;
    let (mut guard_coord, mut guard_dir) = find_starting_position(input)?;

    let mut visited_tuple: HashSet<(Coordinate, Direction)> = HashSet::new();

    loop {
        let current_cell_index = (guard_coord.y * map.width as isize + guard_coord.x) as usize;
        map.cells[current_cell_index] = CellState::Visited;

        if !visited_tuple.insert((guard_coord, guard_dir)) {
            break;
        }

        let next_coord = guard_coord + guard_dir.delta();

        if next_coord.x < 0
            || next_coord.y < 0
            || next_coord.x >= map.width as isize
            || next_coord.y >= map.height as isize
        {
            break;
        }

        let next_cell_index = (next_coord.y * map.width as isize + next_coord.x) as usize;
        match map.cells.get(next_cell_index) {
            Some(&CellState::Obstructed) => {
                guard_dir = guard_dir.turn_right();
            }
            Some(_) => {
                guard_coord = next_coord;
            }
            None => {
                // Should not happen due to our bounds check, but it's good practice to handle it.
                break;
            }
        }
    }

    Some(
        map.cells
            .iter()
            .filter(|&cell| *cell == CellState::Visited)
            .count() as u64,
    )
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
