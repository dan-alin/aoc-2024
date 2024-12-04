use std::vec;

fn prepare(file: &str) -> Vec<Vec<char>> {
    let mut matr: Vec<Vec<char>> = Vec::new();

    for line in file.lines() {
        let row: Vec<char> = line.chars().collect();
        matr.push(row);
    }
    matr
}

fn search_word(
    matrix: &Vec<Vec<char>>,
    word: &str,
    target_idx: usize,
    (r, c): (i32, i32),
    (delta_r, delta_c): (i32, i32),
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    // if the target of the word is reached, the word is found
    if target_idx == word.len() {
        return true;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    // check bounds and visited
    if r < 0 || r >= rows as i32 || c < 0 || c >= cols as i32 || visited[r as usize][c as usize] {
        return false;
    }

    // if the current character is not the target character, the word is not found
    if matrix[r as usize][c as usize] != word.chars().nth(target_idx).unwrap() {
        return false;
    }

    // mark the current cell as visited
    visited[r as usize][c as usize] = true;

    // keep searching in the same direction
    let found = search_word(
        matrix,
        word,
        target_idx + 1,
        (r + delta_r, c + delta_c),
        (delta_r, delta_c),
        visited,
    );

    // unmark the current cell for backtracking
    visited[r as usize][c as usize] = false;

    found
}

pub fn part_one(file: &str) -> i32 {
    let matrix = prepare(file);

    let word = "XMAS";
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut count = 0;
    let directions = vec![
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Bottom-right diagonal
        (1, -1),  // Bottom-left diagonal
        (-1, 1),  // Top-right diagonal
        (-1, -1), // Top-left diagonal
    ];

    let mut visited = vec![vec![false; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == word.chars().nth(0).unwrap() {
                for &(dr, dc) in &directions {
                    if search_word(
                        &matrix,
                        word,
                        1,
                        (row as i32 + dr, col as i32 + dc),
                        (dr, dc),
                        &mut visited,
                    ) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

// pub fn part_two(file: &str) -> i32 {
//     let reports = prepare(file);

//     let mut count = 0;

//     count
// }
