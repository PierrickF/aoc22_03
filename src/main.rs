use std::fs::File;
use std::io::Error;
use task_1::parse_file::parse_file as init_task_1;

pub mod task_1;

fn main() -> Result<(), Error> {
    init_task_1(File::open("input")?);
    Ok(())
}
