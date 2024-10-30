import os
import sqlite3
from mylib.query import query, create_entry

def setup_test_db():
    """Create a temporary test database with sample data"""
    db_name = "test_avengers.db"
    conn = sqlite3.connect(db_name)
    cursor = conn.cursor()

    # Avengers table
    cursor.execute(
        """
        CREATE TABLE IF NOT EXISTS Avengers (
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

    # Insert sample data
    entry_data = (
        'http://example.com', 'New Avenger', 100, 'YES', 'MALE', 'Intro Test', 
        'Full Member', 2022, 1, 'NO', 'NO', 'NO', 'NO', 'NO', 'NO', 'NO', 'NO', 
        'NO', 'NO', 'Test Notes'
    )
    create_entry(db_name, "Avengers", entry_data)

    conn.commit()
    conn.close()

def test_query():
    """Test the query function"""
    db_name = "test_avengers.db"
    table_name = "Avengers"

    # Setup the test database
    if not os.path.exists(db_name):
        setup_test_db()

    # Test the query function
    result = query(database=db_name, table=table_name)
    
    # Assertions
    assert result is not None
    assert len(result) == 5

if __name__ == "__main__":
    # Run the test
    test_query()
    print("Test completed successfully.")
