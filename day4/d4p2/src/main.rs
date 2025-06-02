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
            if grid[(row,col)] == 'A' {
                total += search_mas(&grid, row, col);
            }
        }
    }

    println!("{}", total);

    Ok(())
}

fn search_mas(grid: &grid::Grid<char>, starting_row_ind: usize, starting_col_ind: usize) -> i32 {
    let num_rows = grid.rows();
    let num_cols = grid.cols();

    let mut found_counter: i32 = 0;

    if starting_col_ind < 1 || starting_col_ind == (num_cols-1) {
        return found_counter;
    }
    if starting_row_ind < 1 || starting_row_ind == (num_rows-1) {
        return found_counter;
    }

    // Looking for
    // M.S
    // .A.
    // M.S
    if grid.get(starting_row_ind-1, starting_col_ind-1) == Some(&'M') &&
        grid.get(starting_row_ind+1, starting_col_ind-1) == Some(&'M') &&
        grid.get(starting_row_ind-1, starting_col_ind+1) == Some(&'S') &&
        grid.get(starting_row_ind+1, starting_col_ind+1) == Some(&'S') {
        found_counter += 1;
    }
    // Looking for
    // S.M
    // .A.
    // S.M
    if grid.get(starting_row_ind-1, starting_col_ind+1) == Some(&'M') &&
        grid.get(starting_row_ind+1, starting_col_ind+1) == Some(&'M') &&
        grid.get(starting_row_ind-1, starting_col_ind-1) == Some(&'S') &&
        grid.get(starting_row_ind+1, starting_col_ind-1) == Some(&'S') {
        found_counter += 1;
    }
    // Looking for
    // M.M
    // .A.
    // S.S
    if grid.get(starting_row_ind-1, starting_col_ind-1) == Some(&'M') &&
        grid.get(starting_row_ind-1, starting_col_ind+1) == Some(&'M') &&
        grid.get(starting_row_ind+1, starting_col_ind-1) == Some(&'S') &&
        grid.get(starting_row_ind+1, starting_col_ind+1) == Some(&'S') {
        found_counter += 1;
    }
    // Looking for
    // S.S
    // .A.
    // M.M
    if grid.get(starting_row_ind+1, starting_col_ind+1) == Some(&'M') &&
        grid.get(starting_row_ind+1, starting_col_ind-1) == Some(&'M') &&
        grid.get(starting_row_ind-1, starting_col_ind+1) == Some(&'S') &&
        grid.get(starting_row_ind-1, starting_col_ind-1) == Some(&'S') {
        found_counter += 1;
    }

    if found_counter > 1 {
        println!("\n\n{} {} {}", grid[(starting_row_ind-1, starting_col_ind-1)], 
                             grid[(starting_row_ind-1, starting_col_ind  )], 
                             grid[(starting_row_ind-1, starting_col_ind+1)]);
        println!("{} {} {}", grid[(starting_row_ind,   starting_col_ind-1)], 
                             grid[(starting_row_ind,   starting_col_ind  )], 
                             grid[(starting_row_ind,   starting_col_ind+1)]);
        println!("{} {} {}", grid[(starting_row_ind+1, starting_col_ind-1)], 
                             grid[(starting_row_ind+1, starting_col_ind  )], 
                             grid[(starting_row_ind+1, starting_col_ind+1)]);
    }

    found_counter 
}