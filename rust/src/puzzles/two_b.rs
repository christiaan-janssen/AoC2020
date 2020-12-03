use super::two_a::*;
use crate::utils::file::lines_from_file;


pub fn two_b(path: &str) -> Result<i32, std::io::Error>{
    let mut valid: i32 = 0;
    let lines = lines_from_file(path);

    for line in lines {
        let pw = Password::new(line);
        if pw.valid_toboggan() {
            valid += 1;
        }
    }
    println!("{:?}", valid);
    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_valid_password() {
        let pw = Password::new("1-3 a: abcde".to_string());
        assert_eq!(true, pw.valid_toboggan());
    }

    #[test]
    fn test_for_valid_password2() {
        let pw = Password::new("1-3 b: cdefg".to_string());
        assert_eq!(false, pw.valid_toboggan());
    }

    #[test]
    fn test_for_valid_password3() {
        let pw = Password::new("2-9 c: ccccccccc".to_string());
        assert_eq!(false, pw.valid_toboggan());
    }
}
