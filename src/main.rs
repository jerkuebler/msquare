use rayon::prelude::*;
use itertools::Itertools;
use std::time::SystemTime;

fn check_square(input: &Vec<u32>, size: usize) -> bool {
    let squared: Vec<u32> = input.iter()
        .map(|&i| i*1)
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



fn main() {
    let now = SystemTime::now();
    let s_size: usize = 3;
    let i: u32 = 13;
    let mut result:std::vec::Vec<_> = Vec::new();

    let test = (1..i + 1).permutations(9);
    for chunk in test.chunks(100000).into_iter(){
        result.extend(
            chunk
                .collect_vec()
                .par_iter()
                .filter(|sq| check_square(sq, s_size))
                .collect::<Vec<_>>()
                .into_iter()
                .cloned()
        );
    }

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
