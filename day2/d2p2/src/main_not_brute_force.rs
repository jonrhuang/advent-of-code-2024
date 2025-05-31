use anyhow::{Context, Result};
use std::env;
use std::fs;

#[derive(Copy, Clone)]
enum AscendType {
    Ascending,
    Descending,
}

fn main() -> Result<()> {
    let filename = env::args().nth(1).context("No input found")?;
    let contents = fs::read_to_string(filename)?;

    let mut safe: i32 = 0;
    for line in contents.lines() {
        let report = create_report(line); 

        let error: Option<i32> = process_report(&report);
        
        // If fails once, then dampen by removing one value and testing again
        match error {
            // Error in index 1, can either remove 0th or 1st to dampen
            Some(1) => {
                let mut report_removed_ind_zero = report.clone();
                report_removed_ind_zero.remove(0);
                let removed_ind_zero_err = process_report(&report_removed_ind_zero);

                let mut report_removed_ind_one = report.clone();
                report_removed_ind_one.remove(1);
                let removed_ind_one_err = process_report(&report_removed_ind_one);

                // If removing one value still results in an error then report is unsafe
                if removed_ind_zero_err == None || 
                    removed_ind_one_err == None {
                    println!("report: {:?} safe", report);
                    safe += 1;
                }
            },
            // Error in index 2, can either remove 0th or 1st (meaning 0 to 1 set incorrect ascending rule)
            //                                     2nd (meaning 2 can be removed) 
            Some(2) => {
                let mut report_removed_ind_zero = report.clone();
                report_removed_ind_zero.remove(0);
                let removed_ind_zero_err = process_report(&report_removed_ind_zero);

                let mut report_removed_ind_one = report.clone();
                report_removed_ind_one.remove(1);
                let removed_ind_one_err = process_report(&report_removed_ind_one);

                let mut report_removed_ind_two = report.clone();
                report_removed_ind_two.remove(2);
                let removed_ind_two_err = process_report(&report_removed_ind_two);

                if removed_ind_zero_err == None || 
                    removed_ind_one_err == None || 
                    removed_ind_two_err == None {
                    println!("report: {:?} safe", report);
                    safe += 1;
                }
            },
            Some(n) => {
                let mut report_removed_ind = report.clone();
                report_removed_ind.remove(n as usize);
                let removed_ind_err = process_report(&report_removed_ind);

                if removed_ind_err == None {
                    println!("report: {:?} safe", report);
                    safe += 1;
                }
            },
            None => {
                println!("report: {:?} safe", report);
                safe += 1;
            },
        } 
    }

    println!("{}", safe);
    Ok(())
}

fn process_report(report: &Vec<i32>) -> Option<i32> {
    // println!("\n\nProcessing report: {:?}", report);
    let mut ascend_flag: Option<AscendType> = None;
    let mut last_number: Option<i32> = None;
    for (index, num) in report.into_iter().enumerate() {
        let current_number = *num;
        match ascend_flag {
            Some(rule) => {
                let safe = levels_check(rule, last_number.unwrap(), current_number);
                if safe == false {
                    // println!("returning!! last num: {}. current num: {}. current index: {}", last_number.unwrap(), current_number, index);
                    return Some(index as i32);
                }
                else {
                    last_number = Some(current_number);
                } 
            },
            None => {
                if let None = last_number {
                    // processing first number, set as last_number and move to second number
                    last_number = Some(current_number);
                    continue;
                }
                if current_number > last_number.unwrap() {
                    if current_number - last_number.unwrap() > 3 {
                        // println!("returning!! last num: {}. current num: {}. current index: {}", last_number.unwrap(), current_number, index);
                        return Some(index as i32)
                    }
                    // println!("setting ascend flag to ascending");
                    ascend_flag = Some(AscendType::Ascending);
                }
                else if current_number < last_number.unwrap() {
                    if last_number.unwrap() - current_number > 3 {
                        // println!("returning!! last num: {}. current num: {}. current index: {}", last_number.unwrap(), current_number, index);
                        return Some(index as i32)
                    }
                    // println!("setting ascend flag to descending");
                    ascend_flag = Some(AscendType::Descending);
                }
                else {
                    // println!("returning!! last num: {}. current num: {}. current index: {}", last_number.unwrap(), current_number, index);
                    return Some(index as i32);
                }
                last_number = Some(current_number);
                continue;
            },
        }
    }
    // println!("Safe!");
    return None;
}

fn levels_check(ascend_or_descend: AscendType, first: i32, second: i32) -> bool {
    match ascend_or_descend {
        AscendType::Ascending => {
            if second <= first {
                false
            }
            else if second - first > 3 {
                false
            }
            else {
                true
            }
        },
        AscendType::Descending => {
            if first <= second {
                false
            }
            else if first - second > 3 {
                false
            }
            else {
                true
            }
        },
    }
} 

fn create_report(line: &str) -> Vec<i32> {
    let mut report: Vec<i32> = Vec::new();
    for number in line.split_whitespace() {
        report.push(number.parse().unwrap());
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_report_test() {
        let report_string = "1 2 3 4 5";
        let report = create_report(report_string);
        let report_check = vec![1, 2, 3, 4, 5];
        assert_eq!(report, report_check);
    }

    #[test]
    fn levels_check_ascend_true() {
        let ascend_or_descend = AscendType::Ascending;
        let first = 1;
        let second = 2;
        let result = levels_check(ascend_or_descend, first, second);
        assert_eq!(result, true);
    }

    #[test]
    fn levels_check_ascend_false() {
        let ascend_or_descend = AscendType::Ascending;
        let first = 2;
        let second = 1;
        let result = levels_check(ascend_or_descend, first, second);
        assert_eq!(result, false);
    }

    #[test]
    fn levels_check_ascend_false_out_of_range() {
        let ascend_or_descend = AscendType::Ascending;
        let first = 1;
        let second = 5;
        let result = levels_check(ascend_or_descend, first, second);
        assert_eq!(result, false);
    }

    #[test]
    fn levels_check_ascend_false_eq() {
        let ascend_or_descend = AscendType::Ascending;
        let first = 1;
        let second = 1;
        let result = levels_check(ascend_or_descend, first, second);
        assert_eq!(result, false);
    }

    #[test]
    fn levels_check_descend_true() {
        let ascend_or_descend = AscendType::Descending;
        let first = 2;
        let second = 1;
        let result = levels_check(ascend_or_descend, first, second);
        assert_eq!(result, true);
    }

    #[test]
    fn levels_check_descend_false() {
        let ascend_or_descend = AscendType::Descending;
        let first = 1;
        let second = 2;
        let result = levels_check(ascend_or_descend, first, second);
        assert_eq!(result, false);
    }

    #[test]
    fn levels_check_descend_false_out_of_range() {
        let ascend_or_descend = AscendType::Descending;
        let first = 5;
        let second = 1;
        let result = levels_check(ascend_or_descend, first, second);
        assert_eq!(result, false);
    }

    #[test]
    fn levels_check_descend_false_eq() {
        let ascend_or_descend = AscendType::Descending;
        let first = 1;
        let second = 1;
        let result = levels_check(ascend_or_descend, first, second);
        assert_eq!(result, false);
    }

    #[test]
    fn process_report_ascending_safe() {
        let report = vec![1, 2, 3, 4, 5];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, None);
    }

    #[test]
    fn process_report_ascending_unsafe_descend_ind_2() {
        let report = vec![1, 3, 2, 4, 5];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn process_report_ascending_unsafe_descend_ind_last() {
        let report = vec![1, 3, 5, 8, 6];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn process_report_ascending_unsafe_range_ind_1() {
        let report = vec![1, 5, 2, 4, 5];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn process_report_ascending_unsafe_range_ind_2() {
        let report = vec![1, 2, 7, 8, 9];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn process_report_ascending_unsafe_range_ind_last() {
        let report = vec![1, 2, 4, 6, 12];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn process_report_descending_safe() {
        let report = vec![5, 4, 3, 2, 1];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, None);
    }

    #[test]
    fn process_report_descending_unsafe_ascend_ind_2() {
        let report = vec![5, 4, 6, 2, 1];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn process_report_descending_unsafe_ascend_ind_last() {
        let report = vec![5, 4, 3, 1, 2];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn process_report_descending_unsafe_range_ind_1() {
        let report = vec![9, 5, 4, 3, 2];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn process_report_descending_unsafe_range_ind_2() {
        let report = vec![9, 8, 3, 2, 1];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn process_report_descending_unsafe_range_ind_last() {
        let report = vec![9, 8, 7, 6, 2];
        let result = process_report(&report);
        println!("printing result: {:?}", result);
        assert_eq!(result, Some(4));
    }

}