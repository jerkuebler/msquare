use rayon::prelude::*;
use itertools::Itertools;
use std::time::SystemTime;
use std::fs::File;
use clap::{App, Arg, ArgMatches};

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
    let mut partial:Vec<Vec<&u32>> = Vec::new();
    for square in input.iter().permutations(9) {
        if check_square(&square, size) {
            partial.push(square);
        }
    }
    return partial;
}

fn csv_header(wtr: &mut csv::Writer<File>){
    let header = vec!["Iteration Max Value",
                      "Iteration Run Time",
                      "Iteration Magic Squares",
                      "Total Run Time",
                      "Total Magic Squares"];

    wtr.write_record(header).expect("Couldn't write header");
    wtr.flush().expect("Couldn't flush header to file");
}

fn append_values(input: Vec<String>, wtr: &mut csv::Writer<File>) {
    wtr.write_record(input).expect("Couldn't write record");
    wtr.flush().expect("Couldn't flush");
}

fn split_combos(combos: &[Vec<u32>], side: usize) -> Vec<Vec<&u32>> {
    let mut squares: Vec<Vec<&u32>> = Vec::new();

    combos
        .par_iter()
        .map(|sq| check_permutations(sq, side))
        .collect::<Vec<Vec<_>>>()
        .iter()
        .for_each(|a| squares.extend_from_slice(&a[..]));

    return squares;
}

fn search_m_square_iterative(max_val: u32, side: usize) -> Vec<Vec<u32>> {

    let test = (1..max_val)
        .map(|j| j)
        .combinations(side.pow(2) - 1)
        .map(|combo| combo.iter().chain(&[max_val]).cloned().collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let result = split_combos(&test, side);

    return result
        .iter()
        .cloned()
        .map(|combo| combo.iter().map(|j| **j).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
}

fn search_m_square(max_val: u32, side: usize) -> Vec<Vec<u32>> {

    let test = (1..max_val + 1)
        .map(|j| j)
        .combinations(side.pow(2))
        .collect_vec();

    let result = split_combos(&test, side);

    return result
        .iter()
        .cloned()
        .map(|combo| combo.iter().map(|j| **j).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
}

fn run(config: Config) {
    println!("Side Size: {} | Note: Press Ctrl+C to stop in climb or iterative mode", config.size);
    let mut i: u32;

    let mut wtr = csv::Writer::from_path("test.csv".to_string())
        .expect("Couldn't start writer");
    if config.csv {
        csv_header(&mut wtr);
    }

    if config.max < config.size.pow(2) as u32 {
        i = config.size.pow(2) as u32;
    } else {
        i = config.max
    }

    let cumulative = SystemTime::now();
    let mut _cumulative_m_sqr: usize = 0;

    loop {
        let now = SystemTime::now();

        let result: Vec<Vec<u32>> = (config.search_func)(i, config.size);

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
            if config.csv {
                let new_row = vec![i.to_string(), run_time.to_string(), result.len().to_string(), cu_time.to_string(), _cumulative_m_sqr.to_string()];
                append_values(new_row, &mut wtr);
            }
        }
        println!("{}", report);
        if config.break_loop {
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
    break_loop: bool,
    search_func: fn(u32, usize) -> Vec<Vec<u32>>,
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

    let break_loop: bool;
    let search_func: fn(u32, usize) -> Vec<Vec<u32>>;

    if iterative {
        break_loop = false;
        search_func = search_m_square_iterative;
    } else if max == 0 {
        break_loop = false;
        search_func = search_m_square;
    } else {
        break_loop = true;
        search_func = search_m_square;
    }

    return Config {size, csv, max, iterative, break_loop, search_func}
}

fn main() {
    let matches: ArgMatches = App::new("Magic Squares")
        .version("0.1")
        .author("Jerome Kuebler")
        .about("Finds magic squares of size (size * size) using values up to max num")
        .arg(Arg::with_name("max")
            .short("m")
            .help("Find magic squares using values up to the max. Setting to 0 will enable climb mode")
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
            .help("Log results to CSV")
        )
        .arg(Arg::with_name("iterative")
            .short("i")
            .help("Increase max iteratively. Overrides max value."))
        .arg(Arg::with_name("logging")
            .short("l")
            .help("Logs results to CSV"))
        .get_matches();

    let config = parse_args(matches);
    run(config);
}
