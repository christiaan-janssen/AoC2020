#![allow(dead_code)]
use std::time::Instant;

mod puzzles;
mod utils;

use crate::utils::file::lines_from_file;

use crate::puzzles::one_a::*;
use crate::puzzles::one_b::*;
use crate::puzzles::two_a::*;
use crate::puzzles::two_b::two_b;
use crate::puzzles::three_a::check_map;
use crate::puzzles::three_b::check_map_with_slope;

fn main() {
    let start = Instant::now();

    println!("1a: {}",one_a("../1.input"));
    println!("1b: {}",one_b("../1.input"));
    match two_a("../2.input") {
        Ok(r) => println!("2a: {:?}", r),
        Err(e) => println!("{:?}", e),
    }

    match two_b("../2.input") {
        Ok(r) => println!("2b: {:?}", r),
        Err(e) => println!("{:?}", e),
    }

    let map_vec = lines_from_file("../3.input");
    let trees = check_map(&map_vec);

    println!("3a: {}", trees);

    let awnser = check_map_with_slope(&map_vec, 1, 1) *
        check_map_with_slope(&map_vec, 3, 1) *
        check_map_with_slope(&map_vec, 5, 1) *
        check_map_with_slope(&map_vec, 7, 1) *
        check_map_with_slope(&map_vec, 1, 2);

    println!("{}", awnser);
    println!("Finished after {:?}", start.elapsed());

}
