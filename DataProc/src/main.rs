extern crate polars;

use polars::prelude::*;
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {

    let start = Instant::now();
    
    // 1. Load the CSV file into a DataFrame
    let df = CsvReader::from_path("../AAPL.csv")?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()?;

    // 2. Calculate the average closing price
    let average_close = df.column("Close")
        .unwrap()
        .mean()
        .ok_or("Failed to compute mean.")?;

    println!("Average Close: {}", average_close);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);

    Ok(())
}
