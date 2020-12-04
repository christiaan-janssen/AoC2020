use crate::utils::file::*;

pub fn one_a(path: &str) -> i32 {
    let data = read_lines_to_i32(path);
    match data {
        Ok(v) => { fix_expense_a(v)},
        Err(e) => {println!("Error reading file: {}", e);0},
    }
}

pub fn fix_expense_a(report: Vec<i32>) -> i32 {
    let mut answer = 0;
    for a in &report {
        for b in &report {
            if a + b == 2020 {
                answer = a * b
            }
        }
    }
    answer
}


#[test]
fn test_expense_report() {
    assert_eq!(514579, fix_expense_a(vec![1721, 979, 366, 299, 675, 1456]));
}


