use anyhow::{Context, Result};
use std::env;
use std::fs;

fn main() -> Result<()> {
    todo!("handle case where second number falsely sets ascending state \neg. 5 6 4 3 2 1\n6 will set ascending as true and report will fail but can be removed and report will be safe");
    let filename = env::args().nth(1).context("No input found")?;
    let contents = fs::read_to_string(filename)?;

    let mut safe: i32 = 0;
    for line in contents.lines() {
        let mut ascending: Option<bool> = None;
        let mut last_number_option: Option<i32> = None;
        let mut damped: bool = false;
        safe += 1;

        println!("Report: {:?}", line);
        for numbers in line.split_whitespace() {
            let current_number: i32;
            match last_number_option {
                // Processing numbers after the first
                Some(last_number) => {
                    current_number = numbers.parse().unwrap();

                    match ascending {
                        // Processing numbers after second 
                        Some(true) => {
                            // Doesn't follows ascending rule
                            if current_number <= last_number {
                                if damped == true {
                                    println!("second error: {}", current_number);
                                    safe -= 1;
                                    break;
                                }
                                else {
                                    println!("damped: {}", current_number);
                                    damped = true;
                                    continue;
                                }
                            }
                            // Follows ascending rule but too large a gap
                            else if current_number - last_number > 3 {
                                if damped == true {
                                    println!("second error: {}", current_number);
                                    safe -= 1;
                                    break;
                                }
                                else {
                                    println!("damped: {}", current_number);
                                    damped = true;
                                    continue;
                                }
                            }
                            // Follows ascending rule and within range
                            else {
                                last_number_option = Some(current_number);
                            }
                        },
                        Some(false) => {
                            // Doesn't follows ascending rule
                            if last_number <= current_number {
                                if damped == true {
                                    println!("second error: {}", current_number);
                                    safe -= 1;
                                    break;
                                }
                                else {
                                    println!("damped: {}", current_number);
                                    damped = true;
                                    continue;
                                }
                            }
                            // Follows ascending rule but too large a gap
                            else if last_number - current_number > 3 {
                                if damped == true {
                                    println!("second error: {}", current_number);
                                    safe -= 1;
                                    break;
                                }
                                else {
                                    println!("damped: {}", current_number);
                                    damped = true;
                                    continue;
                                }
                            }
                            // Follows ascending rule and within range
                            else {
                                last_number_option = Some(current_number);
                            }

                        },
                        // Processing the second (could be third) number
                        None => {
                            let diff = current_number - last_number;
                            if diff > 3 {
                                if damped == true {
                                    println!("second error: {}", current_number);
                                    safe -= 1;
                                    break;
                                }
                                println!("damped: {}", current_number);
                                damped = true;
                                continue;
                            }
                            else if diff < -3 {
                                if damped == true {
                                    println!("second error: {}", current_number);
                                    safe -= 1;
                                    break;
                                }
                                println!("damped: {}", current_number);
                                damped = true;
                                continue;
                            }
                            else if diff == 0 {
                                if damped == true {
                                    println!("second error: {}", current_number);
                                    safe -= 1;
                                    break;
                                }
                                println!("damped: {}", current_number);
                                damped = true;
                                continue;
                            }
                            ascending = Some(diff > 0);
                            last_number_option = Some(current_number);
                            continue;
                        },
                    }
                },
                // Processing the first number
                None => {
                    last_number_option = Some(numbers.parse().unwrap());
                    continue;
                },
            }
        }
    }

    println!("{}", safe);
    Ok(())
}
