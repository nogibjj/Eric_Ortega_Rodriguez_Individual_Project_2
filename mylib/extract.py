import sqlite3

def extract(database="avengers.db", table="Avengers"):
    """Extract data directly from the specified SQLite database and table"""
    
    # Connecting SQLite database
    conn = sqlite3.connect(database)
    cursor = conn.cursor()
    
    cursor.execute(f"SELECT * FROM {table}")
    
    # Fetch all rows from the table
    results = cursor.fetchall()
    
    # Close the connection
    conn.close()

    return results

def create_empty_table(database="empty_avengers.db", table="Avengers"):
    """Create an empty table in the specified SQLite database"""
    conn = sqlite3.connect(database)
    cursor = conn.cursor()

    # Create table 
    cursor.execute(
        f"""
        CREATE TABLE IF NOT EXISTS {table} (
            URL TEXT,
            Name_Alias TEXT,
            Appearances INTEGER,
            Current TEXT,
            Gender TEXT,
            Probationary_Introl TEXT,
            Full_Reserve_Avengers_Intro TEXT,
            Year INTEGER,
            Years_since_joining INTEGER,
            Honorary TEXT,
            Death1 TEXT,
            Return1 TEXT,
            Death2 TEXT,
            Return2 TEXT,
            Death3 TEXT,
            Return3 TEXT,
            Death4 TEXT,
            Return4 TEXT,
            Death5 TEXT,
            Notes TEXT
        )
        """
    )
    conn.commit()
    conn.close()

def test_extract_empty_table():
    """Test extracting data from an empty table."""
    db_name = "empty_avengers.db"
    table_name = "Avengers"
    
    # Create the database and empty table
    create_empty_table(database=db_name, table=table_name)
    
    # Extract data from the empty table
    data = extract(database=db_name, table=table_name)
    
    # Verify that the data is empty
    assert len(data) == 0  # Expect no rows in an empty table

if __name__ == "__main__":
    # Example of how to call the extract function and print the result
    data = extract(database="avengers.db", table="Avengers")
    for row in data:
        print(row)

    # Run the test for extracting from an empty table
    test_extract_empty_table()
