use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn main() -> Result<(), Error> {
    parse_file(File::open("input")?);
    Ok(())
}

fn parse_file<R: Read>(input: R) {
    let br = BufReader::new(input);
    for line in br.lines() {
        let str_line = line.expect("Failed to read line");
        let vec = string_to_vector(str_line);
        let (vec1, vec2) = split_vec(&vec);
        println!("\n{:?}", vec);
        println!("{:?}", vec1);
        println!("{:?}", vec2);
    }
}

fn string_to_vector(line: String) -> Vec<char> {
    let mut iter = line.chars();
    let mut vec: Vec<char> = Vec::new();
    for _ in 0..line.chars().count() {
        vec.push(iter.next().unwrap());
    }
    vec
}

fn parse_line(vec: Vec<char>) -> u32 {
    23
}

fn split_vec(vec: &Vec<char>) -> (Vec<char>, Vec<char>) {
    let vec1: Vec<char> = vec[0..vec.len()/2].to_vec();
    let vec2: Vec<char> = vec[vec.len()/2..vec.len()].to_vec();
    (vec1, vec2)
}
