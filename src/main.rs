use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};

fn main() -> Result<(), Error> {
    parse_file(File::open("input")?);
    Ok(())
}

fn parse_file<R: Read>(input: R) {
    let br = BufReader::new(input);
    let mut total: usize = 0;
    for line in br.lines() {
        let str_line = line.expect("Failed to read line");
        let vec = string_to_vector(str_line);
        let (vec1, vec2) = split_vec(&vec);
        if find_duplicate((&vec1, &vec2)) != Err(false) {
            let dup = find_duplicate((&vec1, &vec2)).unwrap();
            total = dup_to_val(dup) + total;
        }
    }
    println!("{total}");
}

fn string_to_vector(line: String) -> Vec<char> {
    let mut iter = line.chars();
    let mut vec: Vec<char> = Vec::new();
    for _ in 0..line.chars().count() {
        vec.push(iter.next().unwrap());
    }
    vec
}

fn find_duplicate(vecs: (&Vec<char>, &Vec<char>)) -> Result<char, bool> {
    for i in vecs.0 {
        if vecs.1.contains(&i) {
            return Ok(*i);
        } else {
            continue
        }
    }
    Err(false)
}

fn split_vec(vec: &Vec<char>) -> (Vec<char>, Vec<char>) {
    let vec1: Vec<char> = vec[0..vec.len() / 2].to_vec();
    let vec2: Vec<char> = vec[vec.len() / 2..vec.len()].to_vec();
    (vec1, vec2)
}

fn dup_to_val(dup: char) -> usize {
    let mut low_alph: Vec<char> = ('a'..='z').collect();
    let mut upp_alph: Vec<char> = ('A'..='Z').collect();
    low_alph.append(&mut upp_alph);
    let val = low_alph.iter().position(|&r| r == dup).unwrap() + 1;
    val
}
