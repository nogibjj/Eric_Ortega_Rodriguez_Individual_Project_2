use rusqlite::{params, Connection, Result};
use std::error::Error;

fn setup_database() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    conn.execute("DROP TABLE IF EXISTS Avengers", [])?;
    conn.execute(
        "CREATE TABLE Avengers (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            superpower TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

#[test]
fn test_query() -> Result<(), Box<dyn Error>> {
    let conn = setup_database()?;
    conn.execute("INSERT INTO Avengers (name, superpower) VALUES (?1, ?2)", params!["Iron Man", "Technology"])?;
    let mut stmt = conn.prepare("SELECT name, superpower FROM Avengers WHERE name = ?1")?;
    let mut rows = stmt.query(params!["Iron Man"])?;
    if let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let superpower: String = row.get(1)?;
        assert_eq!(name, "Iron Man");
        assert_eq!(superpower, "Technology");
    } else {
        panic!("No data found for Iron Man");
    }
    Ok(())
}

#[test]
fn test_extract() -> Result<(), Box<dyn Error>> {
    let conn = setup_database()?;
    conn.execute("INSERT INTO Avengers (name, superpower) VALUES (?1, ?2)", params!["Thor", "Thunder"])?;
    let mut stmt = conn.prepare("SELECT name, superpower FROM Avengers")?;
    let mut rows = stmt.query([])?;
    if let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let superpower: String = row.get(1)?;
        assert_eq!(name, "Thor");
        assert_eq!(superpower, "Thunder");
    } else {
        panic!("No data found for Thor");
    }
    Ok(())
}

#[test]
fn test_trans_load() -> Result<(), Box<dyn Error>> {
    let mut conn = setup_database()?; // Make the connection mutable
    let tx = conn.transaction()?;
    tx.execute("INSERT INTO Avengers (name, superpower) VALUES (?1, ?2)", params!["Hulk", "Strength"])?;
    tx.commit()?;
    let mut stmt = conn.prepare("SELECT name FROM Avengers WHERE name = 'Hulk'")?;
    let mut rows = stmt.query([])?;
    if let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        assert_eq!(name, "Hulk");
    } else {
        panic!("No data found for Hulk");
    }
    Ok(())
}
