import sqlite3

# Connect to the SQLite database
conn = sqlite3.connect('AAPL.db')
cursor = conn.cursor()

# CRUD Operations

# 1. Create: Add a new record.
create_data = ('2023-10-01', '150.00', '155.00', \
            '149.00', '154.50', '154.40', '5000000')
cursor.execute('INSERT INTO my_table (Date, Open, High, \
            Low, Close, Adj, Volume) VALUES (?, ?, ?, ?, ?, ?, ?)', create_data)
conn.commit()

# 2. Read: Fetch and display the first 5 records.
cursor.execute("SELECT * FROM my_table LIMIT 5")
read_data = cursor.fetchall()
print(read_data)

# 3. Update: Modify the 'Open' value of the record with the date '2023-10-01'.
cursor.execute("UPDATE my_table SET Open = '151.00' WHERE Date = '2023-10-01'")
conn.commit()

# 4. Delete: Remove the record with the date '2023-10-01'.
cursor.execute("DELETE FROM my_table WHERE Date = '2023-10-01'")
conn.commit()


# Additional SQL Queries

# 1. Query to fetch the total number of records.
cursor.execute("SELECT COUNT(*) FROM my_table")
total_records = cursor.fetchone()
# write_to_log(f"Total number of records: {total_records[0]}")

conn.close()

