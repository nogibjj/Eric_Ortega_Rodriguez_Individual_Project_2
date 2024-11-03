use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::io::{BufReader, BufRead};
use rusqlite::{params, Connection, Result, Row};

pub fn extract(url: &str, path: &str) -> Result<String, Box<dyn Error>> {
    let response = get(url)?;
    let mut file = File::create(path)?;
    file.write_all(&response.bytes()?)?;
    Ok(path.to_string())
}

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

    // Scope to ensure `stmt` goes out of scope before `connection.close()`
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
    } // `stmt` goes out of scope here

    connection.close().unwrap();
    Ok("data/avengers.db".to_string())
}

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

fn get_value_as_string(row: &Row, index: usize) -> Result<String> {
    match row.get_ref(index)? {
        rusqlite::types::ValueRef::Null => Ok("NULL".to_string()),
        rusqlite::types::ValueRef::Integer(i) => Ok(i.to_string()),
        rusqlite::types::ValueRef::Real(f) => Ok(f.to_string()),
        rusqlite::types::ValueRef::Text(s) => Ok(String::from_utf8_lossy(s).to_string()),
        rusqlite::types::ValueRef::Blob(_) => Ok("[BLOB]".to_string()),
    }
}
