use anyhow::{Context, Result};
use std::{env, fs};
use grid::*;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("No input file")?;
    let contents = fs::read_to_string(filename)?;

    let mut grid: grid::Grid<char> = Grid::new(0,0);

    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for character in line.chars() {
            row.push(character);
        }
        grid.push_row(row);
    }

    let mut total: i32 = 0;

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row,col)] == 'X' {
                total += search_horizontal(&grid, row, col);
                total += search_vertical(&grid, row, col);
                total += search_diagonal(&grid, row, col);
            }
        }
    }

    println!("{}", total);

    Ok(())
}

fn search_diagonal(grid: &grid::Grid<char>, starting_row_ind: usize, starting_col_ind: usize) -> i32 {
    let num_rows = grid.rows();
    let num_cols = grid.cols();

    let mut found_counter: i32 = 0;
    // only search right
    if starting_col_ind < 3 {
        // only search down
        if starting_row_ind < 3 {
            if grid.get(starting_row_ind+1, starting_col_ind+1) == Some(&'M') &&
                grid.get(starting_row_ind+2, starting_col_ind+2) == Some(&'A') &&
                grid.get(starting_row_ind+3, starting_col_ind+3) == Some(&'S') {
                    found_counter += 1;
            }
        }
        // only search up
        else if num_rows - starting_row_ind < 4 {
            if grid.get(starting_row_ind-1, starting_col_ind+1) == Some(&'M') &&
                grid.get(starting_row_ind-2, starting_col_ind+2) == Some(&'A') &&
                grid.get(starting_row_ind-3, starting_col_ind+3) == Some(&'S') {
                    found_counter += 1;
            }
        }
        // search up and down
        else {
            if grid.get(starting_row_ind+1, starting_col_ind+1) == Some(&'M') &&
                grid.get(starting_row_ind+2, starting_col_ind+2) == Some(&'A') &&
                grid.get(starting_row_ind+3, starting_col_ind+3) == Some(&'S') {
                    found_counter += 1;
            }
            if grid.get(starting_row_ind-1, starting_col_ind+1) == Some(&'M') &&
                grid.get(starting_row_ind-2, starting_col_ind+2) == Some(&'A') &&
                grid.get(starting_row_ind-3, starting_col_ind+3) == Some(&'S') {
                    found_counter += 1;
            }
        }

    }
    // only search left
    else if num_cols - starting_col_ind < 4 {
        // only search down
        if starting_row_ind < 3 {
            if grid.get(starting_row_ind+1, starting_col_ind-1) == Some(&'M') &&
                grid.get(starting_row_ind+2, starting_col_ind-2) == Some(&'A') &&
                grid.get(starting_row_ind+3, starting_col_ind-3) == Some(&'S') {
                    found_counter += 1;
            }
        }
        // only search up 
        else if num_rows - starting_row_ind < 4 {
            if grid.get(starting_row_ind-1, starting_col_ind-1) == Some(&'M') &&
                grid.get(starting_row_ind-2, starting_col_ind-2) == Some(&'A') &&
                grid.get(starting_row_ind-3, starting_col_ind-3) == Some(&'S') {
                    found_counter += 1;
            }
        }
        else {
            if grid.get(starting_row_ind+1, starting_col_ind-1) == Some(&'M') &&
                grid.get(starting_row_ind+2, starting_col_ind-2) == Some(&'A') &&
                grid.get(starting_row_ind+3, starting_col_ind-3) == Some(&'S') {
                    found_counter += 1;
            }
            if grid.get(starting_row_ind-1, starting_col_ind-1) == Some(&'M') &&
                grid.get(starting_row_ind-2, starting_col_ind-2) == Some(&'A') &&
                grid.get(starting_row_ind-3, starting_col_ind-3) == Some(&'S') {
                    found_counter += 1;
            }
        }
    }
    // search both left and right
    else { 
        // search down
        if starting_row_ind < 3 {
            if grid.get(starting_row_ind+1, starting_col_ind+1) == Some(&'M') &&
                grid.get(starting_row_ind+2, starting_col_ind+2) == Some(&'A') &&
                grid.get(starting_row_ind+3, starting_col_ind+3) == Some(&'S') {
                    found_counter += 1;
            }
            if grid.get(starting_row_ind+1, starting_col_ind-1) == Some(&'M') &&
                grid.get(starting_row_ind+2, starting_col_ind-2) == Some(&'A') &&
                grid.get(starting_row_ind+3, starting_col_ind-3) == Some(&'S') {
                    found_counter += 1;
            }
        }
        // search up
        else if num_rows - starting_row_ind < 4 {
            if grid.get(starting_row_ind-1, starting_col_ind+1) == Some(&'M') &&
                grid.get(starting_row_ind-2, starting_col_ind+2) == Some(&'A') &&
                grid.get(starting_row_ind-3, starting_col_ind+3) == Some(&'S') {
                    found_counter += 1;
            }
            if grid.get(starting_row_ind-1, starting_col_ind-1) == Some(&'M') &&
                grid.get(starting_row_ind-2, starting_col_ind-2) == Some(&'A') &&
                grid.get(starting_row_ind-3, starting_col_ind-3) == Some(&'S') {
                    found_counter += 1;
            }
        }
        else {
            if grid.get(starting_row_ind-1, starting_col_ind+1) == Some(&'M') &&
                grid.get(starting_row_ind-2, starting_col_ind+2) == Some(&'A') &&
                grid.get(starting_row_ind-3, starting_col_ind+3) == Some(&'S') {
                    found_counter += 1;
            }
            if grid.get(starting_row_ind-1, starting_col_ind-1) == Some(&'M') &&
                grid.get(starting_row_ind-2, starting_col_ind-2) == Some(&'A') &&
                grid.get(starting_row_ind-3, starting_col_ind-3) == Some(&'S') {
                    found_counter += 1;
            }
            if grid.get(starting_row_ind+1, starting_col_ind+1) == Some(&'M') &&
                grid.get(starting_row_ind+2, starting_col_ind+2) == Some(&'A') &&
                grid.get(starting_row_ind+3, starting_col_ind+3) == Some(&'S') {
                    found_counter += 1;
            }
            if grid.get(starting_row_ind+1, starting_col_ind-1) == Some(&'M') &&
                grid.get(starting_row_ind+2, starting_col_ind-2) == Some(&'A') &&
                grid.get(starting_row_ind+3, starting_col_ind-3) == Some(&'S') {
                    found_counter += 1;
            }
        }
    }

    found_counter 
}

fn search_vertical(grid: &grid::Grid<char>, starting_row_ind: usize, starting_col_ind: usize) -> i32 {
    let num_rows = grid.rows();

    let mut found_counter: i32 = 0;
    // only search down
    if starting_row_ind < 3 {
        if grid.get(starting_row_ind+1, starting_col_ind) == Some(&'M') &&
            grid.get(starting_row_ind+2, starting_col_ind) == Some(&'A') &&
            grid.get(starting_row_ind+3, starting_col_ind) == Some(&'S') {
                found_counter += 1;
        }
    }
    // only search up
    else if num_rows - starting_row_ind < 4 {
        if grid.get(starting_row_ind-1, starting_col_ind) == Some(&'M') &&
            grid.get(starting_row_ind-2, starting_col_ind) == Some(&'A') &&
            grid.get(starting_row_ind-3, starting_col_ind) == Some(&'S') {
                found_counter += 1;
        }
    }
    else { 
        if grid.get(starting_row_ind+1, starting_col_ind) == Some(&'M') &&
            grid.get(starting_row_ind+2, starting_col_ind) == Some(&'A') &&
            grid.get(starting_row_ind+3, starting_col_ind) == Some(&'S') {
                found_counter += 1;
        }
        if grid.get(starting_row_ind-1, starting_col_ind) == Some(&'M') &&
            grid.get(starting_row_ind-2, starting_col_ind) == Some(&'A') &&
            grid.get(starting_row_ind-3, starting_col_ind) == Some(&'S') {
                found_counter += 1;
        }
    }

    found_counter 
}

fn search_horizontal(grid: &grid::Grid<char>, starting_row_ind: usize, starting_col_ind: usize) -> i32 {
    let num_cols = grid.cols();

    let mut found_counter: i32 = 0;
    // only search to the right
    if starting_col_ind < 3 {
        if grid.get(starting_row_ind, starting_col_ind+1) == Some(&'M') &&
            grid.get(starting_row_ind, starting_col_ind+2) == Some(&'A') &&
            grid.get(starting_row_ind, starting_col_ind+3) == Some(&'S') {
                found_counter += 1;
        }
    }
    // only search to the left
    else if num_cols - starting_col_ind < 4 {
        if grid.get(starting_row_ind, starting_col_ind-1) == Some(&'M') &&
            grid.get(starting_row_ind, starting_col_ind-2) == Some(&'A') &&
            grid.get(starting_row_ind, starting_col_ind-3) == Some(&'S') {
                found_counter += 1;
        }
    }
    else { 
        if grid.get(starting_row_ind, starting_col_ind+1) == Some(&'M') &&
            grid.get(starting_row_ind, starting_col_ind+2) == Some(&'A') &&
            grid.get(starting_row_ind, starting_col_ind+3) == Some(&'S') {
                found_counter += 1;
        }
        if grid.get(starting_row_ind, starting_col_ind-1) == Some(&'M') &&
            grid.get(starting_row_ind, starting_col_ind-2) == Some(&'A') &&
            grid.get(starting_row_ind, starting_col_ind-3) == Some(&'S') {
                found_counter += 1;
        }
    }

    found_counter 
}