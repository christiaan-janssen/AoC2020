mod puzzles;
mod utils;

use crate::puzzles::one_a::*;
use crate::puzzles::one_b::*;
use crate::puzzles::two_a::*;
use crate::puzzles::two_b::two_b;


fn main() {
    println!("1a: {}",one_a("../1.input"));
    println!("1b: {}",one_b("../1.input"));
    match two_a("../2.input") {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e),
    }

    match two_b("../2.input") {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e),
    }

}
