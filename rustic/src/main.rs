use csv::ReaderBuilder;
use ndarray::{array, Array2};
use ndarray_csv::Array2Reader;
use std::fs::File;
use std::error::Error;

// make reader discern data types and store in respective arrays

const DATA: &str = "C:/Users/Robert/projects/nn/data/grid.csv";

fn read_csv(path_to_file: &str) -> Result<Array2<u64>, Box<dyn Error>> {
    let file = File::open(path_to_file)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    Ok(reader.deserialize_array2((5, 2))?)
}


fn main() {
    let arr = match read_csv(DATA){
        Ok(a) => a,
        Err(e) => panic!("couldn't read file: {e}")
    };
    println!("{:?}", arr);
}
