[![CI](https://github.com/nogibjj/Individual_Project2_LinHui/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Individual_Project2_LinHui/actions/workflows/cicd.yml)

# Individual Project 2 - Rust CLI Binary with SQLite 

This project is a Rust-based command-line interface (CLI) application that interfaces with a SQLite database. It demonstrates comprehensive use of Rust's syntax, unique features, and CRUD (Create, Read, Update, Delete) operations on a SQLite database.

## Rust Source Code

The Rust code is structured to demonstrate a solid understanding of Rust's syntax and its unique features. Features include effective error handling, proper usage of Rust syntax, and the implementation of unique Rust characteristics.

## AAPL.db Database Structure

The AAPL.db SQLite database hosts financial data specifically related to Apple Inc. (AAPL) for the last year. It contains a table named my_table which stores daily stock market metrics for AAPL. The columns in this table represent:

- Date: The specific trading day.
- Open: The stock price at the market's open.
- High: The highest stock price during the trading day.
- Low: The lowest stock price during the trading day.
- Close: The stock price at the market's close.
- Volume: The number of shares traded on that day.
- Adj Close: The adjusted closing price, accounting for factors like dividends and stock splits.

## SQLite Database CRUD Operations

The application interfaces with a SQLite database to perform CRUD operations, which include:

- **Create**: 
A new record is added to the database. For instance, a record might be created with data such as Date: 2023-10-01, Open: 100.50, High: 105.00, Low: 99.50, Close: 104.00, Volume: 5000, and Adjusted: 103.50.

- **Read**: 
First 5 records were fetched to analyze the stock's performance for a glance.

- **Update**: 
The Close value for the record on 2023-10-01 was updated to reflect end-of-day adjustments.

- **Delete**:
The record for 2023-10-01 was deleted to maintain data integrity.

## Use of GitHub Copilot

Throughout the development of this project, GitHub Copilot was instrumental in providing code suggestions, especially when converting the initial Python codebase to Rust. Additionally, Copilot was used to help compose terms for the Makefile, streamlining the build and test processes.

## Optimized Rust Binary

The project includes a GitHub Actions workflow that automates the process of testing, building, and linting the Rust code. As part of this automation, an optimized Rust binary is generated as an artifact, which can be downloaded directly from the GitHub repository.

## Dependencies

The project relies on several Rust libraries and tools, such as rusqlite, as specified in Cargo.toml, and will be automatically compiled when it runs.

## How to Run

1. Clone the repository.
2. Navigate to the project directory and run the project with
```bash
cargo run
```

or release the project with 
```bash
cargo build --release
```

## GitHub Actions

A GitHub Actions workflow (cicd.yml) is set up to automate testing, building, and linting of the Rust code. This ensures code quality, correct building of the Rust code, and adherence to Rust coding standards through linting.

## Video Demo

