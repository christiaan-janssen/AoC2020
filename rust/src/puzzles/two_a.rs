use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Password {
    min_occurrence: i32,
    max_occurrence: i32,
    letter: char,
    password: String,

}

impl Password {
    pub fn new(policy: String) -> Self {
        let input: Vec<_> = policy.split(" ").collect();
        let a: Vec<_> = input[0].split("-").collect();
        Self {
            min_occurrence: a[0].parse::<i32>().unwrap(),
            max_occurrence: a[1].parse::<i32>().unwrap(),
            letter: input[1].to_string().remove(0),
            password: input[2].to_string(),
        }
    }

    pub fn valid(&self) -> bool {
        let mut count: i32 = 0;
        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }
        let rng = self.min_occurrence ..= self.max_occurrence;
        if rng.contains(&count) {
            return true
        }
    false
    }

    pub fn valid_toboggan(&self) -> bool {
        let pw: Vec<char> = self.password.chars().collect::<Vec<_>>();
        if pw[self.min_occurrence as usize -1] == self.letter && pw[self.max_occurrence as usize -1] == self.letter {
            return false
        } else if pw[self.min_occurrence as usize -1] == self.letter || pw[self.max_occurrence as usize -1] == self.letter {
            return true
        }
        false
    }
}

pub fn two_a(path: &str) -> Result<i32, std::io::Error>{
    let mut valid: i32 = 0;
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let pw = Password::new(line.unwrap());
        if pw.valid() {
            valid += 1;
        }
    }
    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_valid_password() {
        let pw = Password::new("1-3 a: abcde".to_string());
        assert_eq!(true, pw.valid());
    }

    #[test]
    fn test_for_valid_password2() {
        let pw = Password::new("1-3 b: cdefg".to_string());
        assert_eq!(false, pw.valid());
    }

    #[test]
    fn test_for_valid_password3() {
        let pw = Password::new("2-9 c: ccccccccc".to_string());
        assert_eq!(true, pw.valid());
    }
}
