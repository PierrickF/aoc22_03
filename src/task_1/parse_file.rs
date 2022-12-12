use std::io::{BufRead, BufReader, Read};

use crate::task_1::string_to_vector::string_to_vector;
use crate::task_1::split_vec::split_vec;
use crate::task_1::find_duplicate::find_duplicate;
use crate::task_1::dup_to_val::dup_to_val;

pub fn parse_file<R: Read>(input: R) {
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
