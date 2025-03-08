use std::collections::{HashSet, VecDeque};

/// Find all shortest paths between start and end numbers in a grid
/// Returns paths as strings of directional characters: '<', '>', '^', 'v'
///
/// # Arguments
/// * `grid` - A 2D vector representing the grid of numbers
/// * `start_number` - The number to start from
/// * `end_number` - The number to end at
///
/// # Returns
/// * A vector of paths, where each path is a string of direction characters
///
/// # Note
/// * Assumes each number appears exactly once in the grid
/// * '<' means move left, '>' means move right, '^' means move up, 'v' means move down
pub fn find_all_shortest_paths(
    grid: &Vec<Vec<i32>>,
    start_number: i32,
    end_number: i32,
) -> Vec<String> {
    // Find the positions of start and end numbers
    let mut start_pos = None;
    let mut end_pos = None;

    for (row, row_values) in grid.iter().enumerate() {
        for (col, &value) in row_values.iter().enumerate() {
            if value == start_number {
                start_pos = Some((row, col));
            } else if value == end_number {
                end_pos = Some((row, col));
            }
        }
    }

    // If either start or end number is not found, return empty vector
    if start_pos.is_none() || end_pos.is_none() {
        return vec![];
    }

    // Find all shortest paths between the start and end positions
    bfs_shortest_paths(grid, start_pos.unwrap(), end_pos.unwrap())
}

/// Helper function to find all shortest paths using BFS
/// Returns paths as strings of directional characters
fn bfs_shortest_paths(
    grid: &Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<String> {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Define possible movements: right, down, left, up
    // The index corresponds to the direction: 0 = right (>), 1 = down (v), 2 = left (<), 3 = up (^)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let direction_chars = ['>', 'v', '<', '^'];

    // Queue for BFS - stores (position, path_string, level)
    // We add level to track distance from start
    let mut queue = VecDeque::new();
    queue.push_back((start, String::new(), 0));

    // Track distance to each cell (used instead of a simple visited set)
    let mut distance = std::collections::HashMap::new();
    distance.insert(start, 0);

    // Store all shortest paths
    let mut shortest_paths = vec![];
    let mut shortest_distance = usize::MAX;

    while let Some((current, path, level)) = queue.pop_front() {
        // If we've already found shorter paths, and this path is longer, skip it
        if !shortest_paths.is_empty() && level > shortest_distance {
            continue;
        }

        // If we found a path to the end
        if current == end {
            // If this is shorter than our current shortest, reset the list
            if level < shortest_distance {
                shortest_paths = vec![path];
                shortest_distance = level;
            }
            // If it's the same length as our shortest, add it to the list
            else if level == shortest_distance {
                shortest_paths.push(path);
            }
            continue;
        }

        // Explore all four directions
        for (dir_idx, &(dr, dc)) in directions.iter().enumerate() {
            let new_row = current.0 as isize + dr;
            let new_col = current.1 as isize + dc;

            // Check if the new position is valid
            if new_row >= 0 && new_row < rows as isize &&
                new_col >= 0 && new_col < cols as isize {
                let new_pos = (new_row as usize, new_col as usize);
                let new_level = level + 1;

                // Visit this cell if:
                // 1. We haven't seen it before, OR
                // 2. We've found an equally short path to it
                if !distance.contains_key(&new_pos) || distance[&new_pos] == new_level {
                    // Update distance
                    distance.insert(new_pos, new_level);

                    // Create new path by extending the current path with the direction character
                    let mut new_path = path.clone();
                    new_path.push(direction_chars[dir_idx]);
                    queue.push_back((new_pos, new_path, new_level));
                }
            }
        }
    }

    shortest_paths
}

/// Example usage
fn main() {
    let grid = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];

    let paths = find_all_shortest_paths(&grid, 1, 9);

    println!("Shortest paths from 1 to 9:");
    for (i, path) in paths.iter().enumerate() {
        println!("Path {}: {}", i + 1, path);
    }
}

// Additional test function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_shortest_paths() {
        let grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];

        // Test path from 1 to 9
        let paths = find_all_shortest_paths(&grid, 1, 9);
        println!("Paths: {:?}", paths);
        assert_eq!(paths.len(), 2); // There should be 2 paths of equal length

        // Both paths should have length 4 (4 movement directions)
        assert_eq!(paths[0].len(), 4);
        assert_eq!(paths[1].len(), 4);

        // Test that the paths are correctly represented
        // From 1 to 9, the paths should be ">>vv" (right, right, down, down)
        // or "vv>>" (down, down, right, right)
        assert!(paths.contains(&String::from(">>vv")) || paths.contains(&String::from("vv>>")));

        // Test path from 1 to 5
        let paths = find_all_shortest_paths(&grid, 1, 5);
        println!("Paths: {:?}", paths);
        assert_eq!(paths.len(), 2); // There should be 2 paths of equal length
        // Paths should be ">" or "v>"
        assert!(paths.contains(&String::from(">v")) || paths.contains(&String::from("v>")));

        // Test with non-existent number
        let paths = find_all_shortest_paths(&grid, 1, 10);
        assert_eq!(paths.len(), 0);
    }

    #[test]
    fn test_complex_paths() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ];

        // Test path from 1 to 16
        let paths = find_all_shortest_paths(&grid, 1, 16);

        // There should be multiple paths, all of length 6
        assert!(!paths.is_empty());
        for path in &paths {
            assert_eq!(path.len(), 6);

            // Verify the path only contains valid direction characters
            for c in path.chars() {
                assert!(c == '>' || c == 'v' || c == '<' || c == '^');
            }
        }

        // Test a simple case with one obvious path
        let simple_grid = vec![
            vec![1, 2],
            vec![3, 4]
        ];

        let paths = find_all_shortest_paths(&simple_grid, 1, 4);
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&String::from(">v")) || paths.contains(&String::from("v>")));
    }
}