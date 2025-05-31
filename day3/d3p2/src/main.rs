use anyhow::{Context, Result};
use regex::Regex;
use std::{env, fs};

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("No input file")?;
    let contents = fs::read_to_string(filename)?;

    let mut split_do: Vec<&str> = contents.as_str().split("do()").collect();

    for splits in &mut split_do {
        let find_dont = splits.find("don't()");
        if let Some(i) = find_dont {
            *splits = splits.get(0..i).unwrap();
        }
    }

    let target_mul_string = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let target_num_string = Regex::new(r"[0-9]{1,3}").unwrap();
    let mut total: i32 = 0;

    for dos in split_do {
        let matches: Vec<_> = target_mul_string.find_iter(dos).map(|m| m.as_str()).collect();

        for mul in matches {
            let numbers: Vec<_> = target_num_string.find_iter(mul).map(|m| m).collect();
            let first: i32 = numbers[0].as_str().parse().unwrap();
            let second: i32 = numbers[1].as_str().parse().unwrap();
            total += first * second;
        }
    }

    println!("{}", total);

    Ok(())
}
