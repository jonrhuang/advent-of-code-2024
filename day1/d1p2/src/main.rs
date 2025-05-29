use std::env;
use std::fs;
use anyhow::{Context, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("need input as arg1")?;
    let contents = fs::read_to_string(filename)?;

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        for (num, char) in line.split_whitespace().enumerate() {
            if num % 2 == 0 {
                list1.push(char.parse().unwrap());
            }
            else {
                list2.push(char.parse().unwrap());
            }
        }
    }

    let mut list2_freq: HashMap<i32, i32> = HashMap::new();

    for number in &list2 {
        list2_freq.entry(*number).and_modify(|counter| *counter += 1).or_insert(1);
    }

    let mut count: i32 = 0;

    for number in &list1 {
        count += *number * *list2_freq.entry(*number).or_default();
    }    

    println!("{:?}", count);

    return Ok(())
}
