use std::fs;
use std::result::Result;
use std::thread;
use std::time::Duration;
use mylib::{extract, trans_load, query};

// Setup function to ensure the data directory exists
fn setup() {
    let _ = fs::create_dir_all("data");
}

#[test]
fn test_extract() {
    setup();  // Ensure `data/` directory exists

    let url = "https://people.sc.fsu.edu/~jburkardt/data/csv/addresses.csv";  // Example small CSV file for testing
    let path = "data/test_extract.csv";

    // Remove the test file if it exists from a previous run
    let _ = fs::remove_file(path);

    let result = extract(url, path);

    assert!(result.is_ok(), "Failed to extract data: {:?}", result);
    assert!(fs::metadata(path).is_ok(), "Extracted file not found");

    // Clean up the test file after the test
    let _ = fs::remove_file(path);
}

#[test]
fn test_trans_load() {
    setup();

    let (path, test_data) = (
        "data/test_trans_load.csv",
        "name,alias,power,affiliation,status\n\
         Iron Man,Tony Stark,Genius,Avenger,Alive\n\
         Thor,Thor Odinson,God,Asgardian,Alive\n\
         Hulk,Bruce Banner,Strength,Avenger,Alive\n"
    );

    // Write test data to a CSV file
    fs::write(path, test_data).expect("Unable to write test data file");

    // Attempt to load data into the SQLite database
    let result = trans_load(path);

    assert!(result.is_ok(), "trans_load failed: {:?}", result);

    assert!(fs::metadata("data/avengers.db").is_ok(), "Database file not found");

    // Clean up files after the test
    let _ = fs::remove_file(path);
    let _ = fs::remove_file("data/avengers.db");
}

// Function to retry trans_load with retries
fn retry_trans_load(path: &str, retries: u32) -> Result<String, Box<dyn std::error::Error>> {
    for _ in 0..retries {
        match trans_load(path) {
            Ok(result) => return Ok(result),
            Err(e) => {
                println!("Retrying trans_load due to error: {:?}", e);
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
    trans_load(path)  // Final attempt
}

#[test]
fn test_query() {
    setup();

    let (path, test_data) = (
        "data/test_query.csv",
        "name,alias,power,affiliation,status\n\
         Captain America,Steve Rogers,Shield,Avenger,Alive\n\
         Black Widow,Natasha Romanoff,Spy,Avenger,Deceased\n\
         Hawkeye,Clint Barton,Archer,Avenger,Alive\n"
    );

    // Write test data to a CSV file
    fs::write(path, test_data).expect("Unable to write test data file");

    // Clean up any pre-existing database file to avoid conflicts
    let _ = fs::remove_file("data/avengers.db");

    // Use retry_trans_load to load data into the database with retries
    retry_trans_load(path, 3).expect("Failed to load test data after retries");

    // Run query on the database
    let query_string = "SELECT * FROM Avengers LIMIT 2";
    let query_result = query(query_string);

    assert!(query_result.is_ok(), "Query execution failed: {:?}", query_result);
    let result_string = query_result.unwrap();
    assert!(result_string.contains("Captain America"));
    assert!(result_string.contains("Steve Rogers"));

    // Clean up test files
    let _ = fs::remove_file(path);
    let _ = fs::remove_file("data/avengers.db");
}
