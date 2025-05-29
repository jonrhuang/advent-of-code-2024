use std::env;
use std::fs;
use anyhow::{Context, Result};

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

    list1.sort();
    list2.sort();

    let mut count: i32 = 0;

    for i in 0..list1.len() {
        let diff = list1[i] - list2[i];
        if diff < 0 {
            count -= diff;
        }
        else {
            count += diff;
        }
    }    

    println!("{:?}", count);


    return Ok(())
}
