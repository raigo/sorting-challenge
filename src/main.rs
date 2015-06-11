#![allow(unused_must_use)]
extern crate time;
use time::precise_time_ns as t;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::fmt;

fn main() {
    let file = match File::open(env::args().nth(1).unwrap()) {
        Ok(file) => file,
        Err(..) => panic!("Huston, we have a problem"),
    };

    let mut start = t();

    let reader = BufReader::new(&file);

    println!("ns: {}  open file", t() - start);
    start = t();

    let mut numbers = vec![false; 10000000];

    println!("ns: {}  allocate vector", t() - start);
    start = t();

    let mut count = 0;
    for line in reader.lines().filter_map(|result|result.ok()) {
        let indx: usize = line.parse().unwrap();
        numbers[indx] = true;
        count = count + 1;
    }

    println!("ns: {}  read lines", t() - start);
    start = t();

    let mut inx = 0;
    let mut str_buf = String::with_capacity(80001000);
    for element in numbers {
        if element {
            str_buf.push_str(&fmt::format(format_args!("{:07}\n", inx)));
        }
        inx = inx + 1;
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
    writer.write_all(str_buf.as_bytes());
    println!("ns: {}  write to file", t() - start);

    println!("Done sorting, lines {} sorted", count);
}
