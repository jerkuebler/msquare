use rayon::prelude::*;
use itertools::Itertools;
use std::time::SystemTime;
use std::env;

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

fn check_permutations(input: &Vec<u32>, size: usize) -> Vec<Vec<u32>> {
    let mut partial: Vec<Vec<u32>> = Vec::new();
    for square in input.iter().permutations(9) {
        if check_square(&square, size) {
            partial.push(square.iter().cloned().cloned().collect());
        }
    }
    return partial;
}

fn log_to_csv(input: Vec<Vec<u32>>, loc: String) {

    let mut wtr = csv::Writer::from_path(loc)
        .expect("Couldn't start writer");
    for square in input {
        let record = square
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();
        wtr.write_record(record).expect("Couldn't write record");
    }
    wtr.flush().expect("Couldn't flush");
}

fn square_finder(max_val: u32, side: usize) -> Vec<Vec<u32>> {
    let mut squares: Vec<Vec<u32>> = Vec::new();

    let test = (1..max_val + 1)
        .map(|j| j)
        .combinations(9)
        .collect_vec();
    test
        .par_iter()
        .map(|sq| check_permutations(sq, side))
        .collect::<Vec<Vec<_>>>()
        .iter()
        .for_each(|a| squares.extend_from_slice(&a[..]));

    return squares;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let s_size: usize = args[1].parse::<usize>().expect("First arg not a valid side size");
    let mut i: u32 = args[2].parse::<u32>().expect("Second arg not a valid max value");
    println!("Side Size: {} \nMax Value: {}", s_size, i);

    loop {
        let now = SystemTime::now();
        let result: Vec<Vec<u32>> = square_finder(i, s_size);

        match now.elapsed() {
            Ok(elapsed) => {
                let run = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
                println!("Run Time: {}", run);
                println!("Magic Squares Found: {}", result.len());
                if result.len() > 1 {
                    let str: String = format!("ssize{}maxv{}run{:.1}.csv", s_size, i, run);
                    log_to_csv(result, str)
                }
                i += 1;
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
