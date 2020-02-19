use rayon::prelude::*;
use itertools::Itertools;
use std::time::SystemTime;

fn check_square(input: &Vec<&u32>, size: usize) -> bool {
    let squared: Vec<u32> = input.iter()
        .map(|i| **i*1)
        .collect();

    let diag1: u32 = squared[0..size.pow(2)].iter().step_by(size + 1).sum();
    let diag2: u32 = squared[size-1..(size.pow(2) - 1)].iter().step_by(size - 1).sum();

    if diag1 != diag2{
        return false;
    }

    for i in 0..size {
        let horz: u32 = squared[i * size .. size * (i + 1)].iter().sum();
        let ver: u32 = squared[i..size.pow(2)].iter().step_by(size).sum();
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
    let now = SystemTime::now();
    let s_size: usize = 3;
    let i: u32 = 17;
    let mut result: Vec<Vec<&u32>> = Vec::new();

    let test = (1..i + 1).combinations(9).collect_vec();
    test
        .par_iter()
        .map(|sq| check_permutations(sq, s_size))
        .collect::<Vec<Vec<_>>>()
        .iter()
        .for_each(|a| result.extend_from_slice(a));

    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
            println!("{}", result.len());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    println!("{:?}", result);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");
}
