use reqwest::blocking::get;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::error::Error;
use rusqlite::{params, Connection, Result, Row};

// Extract function to download data from a URL and save it to a specified path
pub fn extract(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;
    let mut file = File::create(path)?;
    file.write_all(&response.bytes()?)?;
    Ok(path.to_string())
}

// Transform and load function to read CSV data, create a table, and load data into an SQLite database
pub fn trans_load(path: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let connection = Connection::open("data/avengers.db")?;
    connection.execute("DROP TABLE IF EXISTS Avengers", [])?;
    connection.execute(
        "CREATE TABLE Avengers (
            name TEXT,
            alias TEXT,
            power TEXT,
            affiliation TEXT,
            status TEXT
        )",
        [],
    )?;

    {
        let mut stmt = connection.prepare("INSERT INTO Avengers (name, alias, power, affiliation, status) VALUES (?, ?, ?, ?, ?)")?;
        for line in reader.lines().skip(1) {
            let line = line?;
            let fields: Vec<&str> = line.split(',').collect();
            stmt.execute(params![
                fields[0],
                fields[1],
                fields[2],
                fields[3],
                fields[4]
            ])?;
        }
    }

    connection.close().unwrap();
    Ok("data/avengers.db".to_string())
}

// Query function to execute a SQL query and return results as a formatted string
pub fn query(query: &str) -> Result<String, Box<dyn Error>> {
    let connection = Connection::open("data/avengers.db")?;
    let mut stmt = connection.prepare(query)?;

    let column_names: Vec<String> = stmt.column_names().iter().map(|&name| name.to_string()).collect();

    let mut rows = stmt.query(params![])?;

    let mut result = String::new();
    while let Some(row) = rows.next()? {
        for (i, column_name) in column_names.iter().enumerate() {
            let value = get_value_as_string(&row, i)?;
            result.push_str(&format!("{}: {}", column_name, value));
            if i < column_names.len() - 1 {
                result.push_str(", ");
            }
        }
        result.push('\n');
    }

    Ok(result)
}

// Helper function to convert SQLite row values to strings for consistent formatting
fn get_value_as_string(row: &Row, index: usize) -> Result<String> {
    match row.get_ref(index)? {
        rusqlite::types::ValueRef::Null => Ok("NULL".to_string()),
        rusqlite::types::ValueRef::Integer(i) => Ok(i.to_string()),
        rusqlite::types::ValueRef::Real(f) => Ok(f.to_string()),
        rusqlite::types::ValueRef::Text(s) => Ok(String::from_utf8_lossy(s).to_string()),
        rusqlite::types::ValueRef::Blob(_) => Ok("[BLOB]".to_string()),
    }
}

// Delete function to remove rows from the Avengers table based on a specified condition
pub fn delete_rows(condition: &str) -> Result<(), Box<dyn Error>> {
    let connection = Connection::open("data/avengers.db")?;
    let delete_query = format!("DELETE FROM Avengers WHERE {}", condition);
    connection.execute(&delete_query, params![])?;
    connection.close().unwrap();
    Ok(())
}
