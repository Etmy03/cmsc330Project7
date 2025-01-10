// To run via cargo from top-level project directory:
// # with command line arg 
// cargo run --quiet --bin probx_wc -- gettysburg.txt 
// # no command line arg
// cargo run --quiet --bin probx_wc -- 

use std::env;
use std::process::exit;         // bare use of exit(num) function to exit program
//use std::env::args;             // bare use of args() function to retrieve commandline args
use std::fs::File;              // for file io
use std::io::{prelude::*, BufReader}; 

// Above import does two things
// 1. Imports everying in std::io::prelude to give access to common IO
//    traits
// 2. Imports BufReader struct which implements line-by-line input
//    processing


// YOUR CODE BELOW for implementing the word count program
fn main() {
    //command line arguments
    let args: Vec<String> = env::args().collect();

    let filename = 
    if args.len() > 1 {
        &args[1]
    } 
    else {
        eprintln!("usage: target/debug/prob2_wc <filename>"); //error message 1
        exit(1);
    };

    let my_file = match File::open(filename) {
        Ok(my_file) => my_file,
        Err(err) => {
            eprintln!("Couldn't open file test-data/no-such-file.txt: {}", err);//error message 2
            exit(1);
        }
    };

    //to read the file line-by-line
    let reader = BufReader::new(my_file);

    let mut l_count = 0;
    let mut w_count = 0;
    let mut c_count = 0;

    for my_line in reader.lines() {
        if let Ok(my_line) = my_line {
            l_count += 1;
            w_count += my_line.split_whitespace().count();
            c_count += my_line.len() + 1;
        }
    }

    //final print message
    println!("{:4} {:4} {:4} {}", l_count, w_count, c_count, filename);

}