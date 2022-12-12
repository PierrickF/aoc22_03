use std::fs::File;
use std::io::Error;

use crate::task_1::parse_file::parse_file;

pub mod task_1;

fn main() -> Result<(), Error> {
    parse_file(File::open("input")?);
    Ok(())
}
