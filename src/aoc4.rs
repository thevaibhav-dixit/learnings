use crate::helpers::take_input;

fn count_xmas_occurrences(grid: Vec<Vec<char>>) -> usize {
    let word = "XMAS";
    let directions = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for (k, c) in word.chars().enumerate() {
                    let nx = i as isize + k as isize * dx;
                    let ny = j as isize + k as isize * dy;

                    if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                        found = false;
                        break;
                    }

                    if grid[nx as usize][ny as usize] != c {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn aoc4_solve() {
    let input = take_input("aoc4.txt").unwrap();
    let grid = input.lines().map(|line| line.chars().collect()).collect();

    let count = count_xmas_occurrences(grid);
    println!("Total occurrences of XMAS: {}", count);
}
