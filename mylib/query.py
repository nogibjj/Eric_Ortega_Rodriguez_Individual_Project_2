import sqlite3

def query(database="avengers.db", table="Avengers"):
    """Query the database for the top 5 rows of the specified table"""
    try:
        conn = sqlite3.connect(database)
        cursor = conn.cursor()

        # Top 5 rows
        cursor.execute(f"SELECT * FROM {table} LIMIT 5")
        results = cursor.fetchall()

        return results
    except sqlite3.Error as e:
        print(f"Error occurred while querying the database: {e}")
        return []
    finally:
        if conn:
            conn.close()

def create_entry(database, table, entry_data):
    """Create a new entry in the table"""
    try:
        conn = sqlite3.connect(database)
        cursor = conn.cursor()

        # Insert a new record 
        cursor.execute(
            f"""
            INSERT INTO {table} (
                URL, Name_Alias, Appearances, Current, Gender, Probationary_Introl, 
                Full_Reserve_Avengers_Intro, Year, Years_since_joining, Honorary, 
                Death1, Return1, Death2, Return2, Death3, Return3, Death4, Return4, 
                Death5, Notes
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, 
            entry_data
        )

        conn.commit()
        print("Record created successfully.")
    except sqlite3.Error as e:
        print(f"Error occurred while creating record: {e}")
    finally:
        if conn:
            conn.close()


def read_entries(database, table, limit=5):
    """Read the top rows from the table"""
    try:
        conn = sqlite3.connect(database)
        cursor = conn.cursor()
 
        cursor.execute(f"SELECT * FROM {table} LIMIT ?", (limit,))
        results = cursor.fetchall()

        return results
    except sqlite3.Error as e:
        print(f"Error occurred while reading records: {e}")
        return []
    finally:
        if conn:
            conn.close()


def update_entry(database, table, column, new_value, condition_column, condition_value):
    """Update an existing entry in the table"""
    try:
        conn = sqlite3.connect(database)
        cursor = conn.cursor()

        cursor.execute(
            f"""
            UPDATE {table}
            SET {column} = ?
            WHERE {condition_column} = ?
            """, 
            (new_value, condition_value)
        )

        conn.commit()
        print("Record updated successfully.")
    except sqlite3.Error as e:
        print(f"Error occurred while updating record: {e}")
    finally:
        if conn:
            conn.close()


def delete_entry(database, table, condition_column, condition_value):
    """Delete an entry from the table based on a condition"""
    try:
        conn = sqlite3.connect(database)
        cursor = conn.cursor()

        # Delete
        cursor.execute(
            f"DELETE FROM {table} WHERE {condition_column} = ?", 
            (condition_value,)
        )

        conn.commit()
        print("Record deleted successfully.")
    except sqlite3.Error as e:
        print(f"Error occurred while deleting record: {e}")
    finally:
        if conn:
            conn.close()


if __name__ == "__main__":
    db_name = "avengers.db"
    table_name = "Avengers"

    # Example: Create a new entry
    new_entry = (
        'http://example.com', 'New Avenger', 100, 'YES', 'MALE', 'Intro Test', 
        'Full Member', 2022, 1, 'NO', 'NO', 'NO', 'NO', 'NO', 'NO', 'NO', 'NO', 
        'NO', 'NO', 'Test Notes'
    )
    create_entry(db_name, table_name, new_entry)

    # Example: Read top 5 entries
    print("Reading top 5 entries:")
    entries = read_entries(db_name, table_name)
    for entry in entries:
        print(entry)

    # Example: Updating an entry
    update_entry(db_name, table_name, 'Appearances', 150, 'Name_Alias', 'New Avenger')

    # Example: Deleting an entry
    delete_entry(db_name, table_name, 'Name_Alias', 'New Avenger')
