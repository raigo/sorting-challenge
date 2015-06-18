extern crate time;
use time::precise_time_ns as t;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
//use std::io::Read;

fn main() {
    let file = match File::open(env::args().nth(1).unwrap()) {
        Ok(file) => file,
        Err(..) => panic!("Huston, we have a problem"),
    };

    let mut start = t();

    let reader = BufReader::new(&file);

    println!("ns: {}  open file", t() - start);
    start = t();

    let mut numbers : Vec<String> = Vec::with_capacity(10_000_000);
    unsafe {
        numbers.set_len(10_000_000);
    }

    println!("ns: {}  allocate vector", t() - start);
    start = t();

//    let str_buf0 = &mut String::with_capacity(80_000_010);
//    reader.read_to_string(str_buf0);
//    println!("buf len : {}", str_buf0.len());
//    println!("{}", str_buf0);

    let mut count = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let indx: usize = l.parse().unwrap();
                numbers[indx] = l;
            },
            Err(_) => {}
        }
        count += 1;
    }

    println!("ns: {}  read lines", t() - start);
    start = t();

    let mut str_buf = String::with_capacity(80_000_010);
    for number in &numbers {
        if !number.is_empty() {
            str_buf.push_str(number);
            str_buf.push_str("\n");
        }
    }

    println!("ns: {}  parse lines ", t() - start);
    start = t();

    let out_file = match OpenOptions::new().write(true).create(true)
            .open(env::args().nth(2).unwrap()) {
        Ok(out_file) => out_file,
        Err(..) => panic!("Can't create file for sorted result")
    };

    println!("ns: {}  create output file", t() - start);
    start = t();

    let mut writer = BufWriter::new(&out_file);
    let _ = writer.write_all(str_buf.as_bytes());
    println!("ns: {}  write to file", t() - start);

    println!("Done sorting, lines {} sorted", count);
}
