use rayon::prelude::*;
use itertools::Itertools;
use std::time::SystemTime;
use clap::{App, Arg, ArgMatches};

extern crate csv;
extern crate clap;

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

fn check_permutations(input: &Vec<u32>, size: usize) -> Vec<Vec<u32>> {
    let mut partial:Vec<Vec<u32>> = Vec::new();
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

fn search_m_square_iterative(max_val: u32, side: usize) -> Vec<Vec<u32>> {
    let mut squares: Vec<Vec<u32>> = Vec::new();

    let test = (1..max_val)
        .map(|j| j)
        .combinations(8)
        .map(|combo| combo.iter().chain(&[max_val]).cloned().collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    test
        .par_iter()
        .map(|sq| check_permutations(sq, side))
        .collect::<Vec<Vec<_>>>()
        .iter()
        .for_each(|a| squares.extend_from_slice(&a[..]));

    return squares;
}

fn search_m_square(max_val: u32, side: usize) -> Vec<Vec<u32>> {
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

fn run(config: Config) {
    println!("Side Size: {} | Note: Press Ctrl+C to stop in climb or iterative mode", config.size);
    let break_loop: bool;
    let mut i: u32;
    let search_func: fn(u32, usize) -> Vec<Vec<u32>>;

    if config.iterative {
        break_loop = false;
        i = config.size.pow(2) as u32;
        search_func = search_m_square_iterative;
    } else if config.max == 0 {
        break_loop = false;
        i = config.size.pow(2) as u32;
        search_func = search_m_square;
    } else {
        break_loop = true;
        i = config.max;
        search_func = search_m_square;
    }

    let cumulative = SystemTime::now();
    let mut _cumulative_m_sqr: usize = 0;

    loop {
        let now = SystemTime::now();

        let result: Vec<Vec<u32>> = search_func(i, config.size);

        let run = now.elapsed().expect("Couldn't get run time due to system clock error");
        let run_time = run.as_secs() as f64 + run.subsec_nanos() as f64 *1e-9;

        let cu = cumulative.elapsed().expect("Couldn't get cumulative time due to system clock error");
        let cu_time = cu.as_secs() as f64 + cu.subsec_nanos() as f64 *1e-9;

        let max = format!("Max Value: {} | ", i);
        let r_time = format!("Run Time: {:.3}s | ", run_time);
        let m_squares = format!("Magic Squares This Pass: {} | ", result.len());


        let mut report = format!("{}{}{}", max, r_time, m_squares);
        if config.iterative {
            _cumulative_m_sqr += result.len();
            let cu_time_str = format!("Cumulative Time: {:.3}s | ", cu_time);
            let cu_m_squares = format!("Cumulative Magic Squares: {} | ", _cumulative_m_sqr);
            report = format!("{}{}{}", report, cu_time_str, cu_m_squares);
        }
        println!("{}", report);
        if result.len() > 1 && config.csv {
            let str: String = format!("ssize{}maxv{}run{:.1}.csv", config.size, i, run_time);
            log_to_csv(result, str)
        }
        if break_loop {
            break
        } else {
            i += 1;
        }
    }
}

struct Config {
    size: usize,
    csv: bool,
    max: u32,
    iterative: bool,
}

fn parse_args(args: ArgMatches) -> Config {

    let size: usize = args
        .value_of("size")
        .unwrap()
        .parse::<usize>()
        .expect("Couldn't make size a usize");

    let csv: bool = args.is_present("csv");

    let max = args
        .value_of("max")
        .unwrap()
        .parse::<u32>()
        .expect("Couldn't make max size a u32");

    let iterative: bool = args.is_present("iterative");

    return Config {size, csv, max, iterative}
}

fn main() {
    let matches: ArgMatches = App::new("Magic Squares")
        .version("0.1")
        .author("Jerome Kuebler")
        .about("Finds magic squares of size (size * size) using values up to max num")
        .arg(Arg::with_name("max")
            .short("m")
            .help("Find magic squares using values up to the max. Setting to 0 will enable loop")
            .takes_value(true)
            .default_value("9")
        )
        .arg(Arg::with_name("size")
            .short("s")
            .help("Length of one side of the square")
            .required(true)
            .takes_value(true)
            .default_value("3")
        )
        .arg(Arg::with_name("csv")
            .short("c")
            .help("Log magic squares to CSV")
        )
        .arg(Arg::with_name("iterative")
            .short("i")
            .help("Increase max iteratively. Overrides max value."))
        .get_matches();

    let config = parse_args(matches);
    run(config);
}
