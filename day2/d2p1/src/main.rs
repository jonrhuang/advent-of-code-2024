use anyhow::{Context, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("must have input")?;
    let contents = fs::read_to_string(filename)?;

    let mut passing = 0;

    for line in contents.lines() {
        // generate report by line
        let mut report: Vec<i32> = Vec::new();
        for number in line.split_whitespace() {
            report.push(number.parse().unwrap());
        }

        // process report
        passing += 1;
        let ascend_flag = report[0] < report[1];

        for number in report.windows(2) {
            if ascend_flag == true  {
                if number[0] >= number[1] {
                    passing -= 1;
                    break;
                }
                if number[1] - number[0] > 3 {
                    passing -= 1;
                    break;
                }
            }
            if ascend_flag == false {
                if number[0] <= number[1] {
                    passing -= 1;
                    break;
                }
                if number[0] - number[1] > 3 {
                    passing -= 1;
                    break;
                }
            }
        }
    }

    println!("{}", passing);

    Ok(())
}
