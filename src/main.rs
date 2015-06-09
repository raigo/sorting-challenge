use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::fmt;

fn main() {
    println!("Hello, world!");
    if env::args().nth(1) == None || env::args().nth(2) == None {
        println!("Usage: sorting filename_source filename_to_sort");
        return;
    }
    println!("{}", env::args().nth(1).expect("Usage: sorting filename"));
    let file = match File::open(env::args().nth(1).unwrap()) {
        Ok(file) => file,
        Err(..) => panic!("Huston, we have a problem"),
    };
    let reader = BufReader::new(&file);

    let mut numbers = vec![false; 10000000];
    let mut count = 0;
    for line in reader.lines().filter_map(|result|result.ok()) {
        let indx: usize = line.parse().unwrap();
        numbers[indx] = true;
        count = count + 1;
    }

    let mut inx = 1;
    let mut str_buf = String::with_capacity(80000000);
    for element in numbers {
        if element {
            str_buf.push_str(&fmt::format(format_args!("{:07}\n", inx)));
        }
        inx = inx + 1;
    }

    let out_file = match OpenOptions::new().write(true).create(true)
            .open(env::args().nth(2).unwrap()) {
        Ok(out_file) => out_file,
        Err(..) => panic!("Can't create file sorted.txt")
    };
    let mut writer = BufWriter::new(&out_file);
    let res = writer.write_all(str_buf.as_bytes());
    println!("{:?}", res);

    println!("Done sorting, lines {} sorted", count);
}
