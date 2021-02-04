extern crate byteorder;

mod lib;

use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::Read;
use std::io::Write;

use lib::language::*;

fn main() {
    //collect the arguments
    let argv: Vec<_> = std::env::args().collect();

    if argv.len() != 2 {
        println!("File not provided");
        println!(" USAGE : stack-langc file");
        return;
    }

    let filename = argv[1].clone();
    let mut input = String::new();
    File::open(filename)
        .expect("Failed to open file")
        .read_to_string(&mut input)
        .expect("Failed to read file");

    //tokenize the given input
    let tok = lexer(input.as_str()); //pass as a &str
    let parsed_data = parser(tok);

    //let mut file = OpenOptions::new().write(true).create(true).open("a.out");
    let mut file = File::create("a.out").expect("Unable to create file to output");

    //write to file
    for i in parsed_data {
        //println!(" {:b} ",i);
        let mut buf = [0; 4];

        //convert to bytes
        LittleEndian::write_i32(&mut buf, i);
        file.write(&buf).expect("Unable to write to file");
    }
}
