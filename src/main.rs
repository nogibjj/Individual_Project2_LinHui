extern crate rusqlite;

use rusqlite::{Connection, params};


fn main() -> Result<(), rusqlite::Error> {
    // Connect to the SQLite database
    let conn = Connection::open("../AAPL.db")?;

    // CRUD Operations

    // 1. Create: Add a new record.
    let create_data = ("2023-10-01", "150.00", "155.00", "149.00", 
            "154.50", "154.40", "5000000");
    conn.execute("INSERT INTO my_table (Date, Open, High, 
            Low, [Close], [Adj Close], Volume) VALUES (?, ?, ?, ?, ?, ?, ?)", 
                 params![create_data.0, create_data.1, create_data.2, 
                 create_data.3, create_data.4, create_data.5, create_data.6])?;

    // 2. Read: Fetch and display the first 5 records.
    let mut stmt = conn.prepare("SELECT * FROM my_table LIMIT 5")?;
    let read_data: Result<Vec<_>, _> = stmt.query_map((), |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, f64>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, f64>(5)?,
            row.get::<_, i32>(6)?
        ))
    })?.collect();
    for row in read_data? {
        println!("{:?}", row);
    }

    // 3. Update: Modify the 'Open' value of the record with the date '2023-10-01'.
    conn.execute("UPDATE my_table SET Open = '151.00' WHERE Date = '2023-10-01'", ())?;

    // // 4. Delete: Remove the record with the date '2023-10-01'.
    conn.execute("DELETE FROM my_table WHERE Date = '2023-10-01'", ())?;

    // Additional SQL Queries

    // 1. Query to fetch the total number of records.
    let total_records: i32 = conn.query_row("SELECT COUNT(*) FROM my_table", 
            (), |row| row.get(0))?;
    println!("Total number of records: {}", total_records);

    Ok(())
}
