fn prepare(file: &str) -> Vec<Vec<char>> {
    file.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(file: &str) -> i32 {
    let mut matrix = prepare(file);
    let mut count = 0;

    let dir = (-1, 0);

    let mut guard_pos: (i32, i32);

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '^' {
                guard_pos = (row as i32, col as i32);
                matrix[row][col] = 'X';
                move_into_direction(&mut matrix, &mut guard_pos, dir);
            }
        }
    }

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'X' {
                count += 1;
            }
        }
    }

    count
}

fn move_into_direction(
    matrix: &mut Vec<Vec<char>>,
    guard_pos: &mut (i32, i32),
    dir: (i32, i32),
) -> bool {
    let mut exit_found = false;

    let next_row = guard_pos.0 + dir.0;
    let next_col = guard_pos.1 + dir.1;

    if next_row < 0
        || next_row >= matrix.len() as i32
        || next_col < 0
        || next_col >= matrix[0].len() as i32
    {
        return exit_found;
    }

    match matrix[next_row as usize][next_col as usize] {
        '.' => {
            *guard_pos = (next_row, next_col);
            matrix[next_row as usize][next_col as usize] = 'X';
            exit_found = move_into_direction(matrix, guard_pos, dir);
        }
        'X' => {
            *guard_pos = (next_row, next_col);
            exit_found = move_into_direction(matrix, guard_pos, dir);
        }
        '#' => {
            let new_dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => dir,
            };
            exit_found = move_into_direction(matrix, guard_pos, new_dir);
        }
        _ => {
            exit_found = true;
        }
    }

    exit_found
}

pub fn part_two(_file: &str) -> i32 {
    0
}
