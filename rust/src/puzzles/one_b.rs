use crate::utils::file::*;

pub fn one_b(path: &str) -> i32 {
    let data = read_lines_to_i32(path);
    let out = match data {
        Ok(v) => { fix_expense_b(v)},
        Err(e) => {println!("Error reading file: {}", e);0},
    };
    out
}

pub fn fix_expense_b(report: Vec<i32>) -> i32 {
    let mut answer =0;
    for a in &report {
        for b in &report {
            for c in &report {
                if a + b + c == 2020 {
                    answer = a * b * c
                }
            }
        }
    }
    answer
}


#[test]
fn test_expense_report2() {
    assert_eq!(241861950, fix_expense_b(vec![1721, 979, 366, 299, 675, 1456]));
}