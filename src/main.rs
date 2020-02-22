use rayon::prelude::*;
use itertools::Itertools;
use std::time::SystemTime;
use std::env;

fn check_square(input: &Vec<&u32>, size: usize) -> bool {

    let diag1: u32 = input[0..size.pow(2)].iter()
        .step_by(size + 1)
        .fold(0, |sum, x| sum + **x);

    let diag2: u32 = input[size-1..(size.pow(2) - 1)].iter()
        .step_by(size - 1)
        .fold(0, |sum, x| sum + **x);

    if diag1 != diag2{
        return false;
    }

    for i in 0..size {
        let horz: u32 = input[i * size .. size * (i + 1)].iter()
            .fold(0, |sum, x| sum + **x);

        let ver: u32 = input[i..size.pow(2)].iter()
            .step_by(size)
            .fold(0, |sum, x| sum + **x);

        if horz != ver || horz != diag1 {
            return false;
        }
    }

    return true;
}

fn check_permutations(input: &Vec<u32>, size: usize) -> Vec<Vec<&u32>> {
    let mut partial = Vec::new();
    for square in input.iter().permutations(9) {
        if check_square(&square, size) {
            partial.push(square);
        }
    }
    return partial;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let s_size: usize = args[1].parse::<usize>().expect("First arg not a valid side size");
    let i: u32 = args[2].parse::<u32>().expect("Second arg not a valid max value");
    println!("Side Size: {} \nMax Value: {}", s_size, i);

    let now = SystemTime::now();
    let mut result: Vec<Vec<&u32>> = Vec::new();

    let test = (1..i + 1)
        .map(|j| j*j)
        .combinations(9)
        .collect_vec();
    test
        .par_iter()
        .map(|sq| check_permutations(sq, s_size))
        .collect::<Vec<Vec<_>>>()
        .iter()
        .for_each(|a| result.extend_from_slice(a));

    match now.elapsed() {
        Ok(elapsed) => {
            println!("Run Time: {}", elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
            println!("Magic Squares Found: {}", result.len());
            println!("Magic Squares: {:?}", result);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
