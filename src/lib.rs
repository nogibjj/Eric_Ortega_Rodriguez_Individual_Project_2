use rusqlite::{Connection, Result};
use std::error::Error;
use csv::ReaderBuilder;

/// Extracts data from the specified database and table.
/// 
/// # Arguments
/// * `database` - The path to the SQLite database file.
/// * `table` - The name of the table to extract data from.
///
/// # Returns
/// A `Result` containing a vector of rows, where each row is a vector of strings.
pub fn extract(database: &str, table: &str) -> Result<Vec<Vec<String>>> {
    let conn = Connection::open(database)?;
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table))?;
    let rows = stmt.query_map([], |row| {
        // Collect all columns in each row as strings
        Ok((0..row.column_count())
            .map(|i| row.get::<usize, String>(i).unwrap_or_default())
            .collect())
    })?;

    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }

    Ok(data)
}

/// Loads data from a CSV file into the specified database and table.
/// 
/// # Arguments
/// * `dataset` - The path to the CSV file.
/// * `db_name` - The path to the SQLite database file.
/// * `table_name` - The name of the table to insert data into.
///
/// # Returns
/// A `Result` indicating success or failure.
pub fn load(dataset: &str, db_name: &str, table_name: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db_name)?;

    let mut rdr = ReaderBuilder::new().from_path(dataset)?;
    for result in rdr.records() {
        let record = result?;
        // Assuming the table has the correct number of columns; adjust placeholders if necessary
        let params: Vec<&dyn rusqlite::ToSql> = record.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
        conn.execute(
            &format!("INSERT INTO {} VALUES ({})", table_name, vec!["?"; record.len()].join(", ")),
            params.as_slice(),
        )?;
    }
    Ok(())
}

/// Queries data from the specified database and table.
/// 
/// # Arguments
/// * `database` - The path to the SQLite database file.
/// * `table` - The name of the table to query data from.
///
/// # Returns
/// A `Result` containing a vector of rows, where each row is a vector of strings.
pub fn query(database: &str, table: &str) -> Result<Vec<Vec<String>>> {
    let conn = Connection::open(database)?;
    let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table))?;
    let rows = stmt.query_map([], |row| {
        Ok((0..row.column_count())
            .map(|i| row.get::<usize, String>(i).unwrap_or_default())
            .collect())
    })?;

    let mut data = Vec::new();
    for row in rows {
        data.push(row?);
    }

    Ok(data)
}
