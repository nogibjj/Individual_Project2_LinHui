# Mini Project 8 

This project consists of two implementations for calculating the average closing price of stock data: one written in Python and the other in Rust.

## Dataset: Apple’s stock price in the past year

Column:

- Date
- Open price
- High price
- Low price
- Adjusted close price
- Trading volumn

## Code Functionality

Both the Python and Rust implementations read a CSV file containing stock data, specifically focusing on the "Close" column, to compute the average closing price.

- **Python**: The code uses the pandas library to read the CSV file, extract the "Close" column, and calculate its mean.

- **Rust**: The Rust implementation employs the polars crate to achieve similar functionality, reading the CSV and computing the mean of the "Close" column.


## Cargo File

The Cargo.toml file is specific to the Rust implementation. It's the manifest file for Rust's package manager, Cargo. The file contains metadata about the Rust project, such as its name, version, authors, and dependencies. In this project, the Cargo.toml file specifies dependencies like polars and other related configurations to ensure the Rust code runs correctly.


## Performance Comparison

From preliminary tests, it's evident that the Rust implementation offers a speed advantage over the Python version:

- Python: The code executed in approximately 0.0072 seconds.
- Rust: The Rust code took around 0.0222 seconds.


Though Rust's performance in this test is slightly slower, Rust generally offers more consistent and predictable performance, especially in more extensive and complex workloads. The efficiency of Rust arises from its systems-level capabilities, no garbage collector, and optimization-friendly nature. In scenarios with larger datasets or more complex processing, the benefits of Rust would likely become even more pronounced.














