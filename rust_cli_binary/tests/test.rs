use std::fs;
use mylib::{extract, trans_load, query};

#[test]
fn test_extract() {
    let url = "https://people.sc.fsu.edu/~jburkardt/data/csv/addresses.csv";  // Example small CSV file for testing
    let path = "data/test_extract.csv";

    // Ensure any pre-existing file is removed before the test
    let _ = fs::remove_file(path);

    // Run the extract function
    let result = extract(url, path);

    // Check that the result is successful and the file exists
    assert!(result.is_ok());
    assert!(fs::metadata(path).is_ok());

    // Clean up the test file
    let _ = fs::remove_file(path);
}

#[test]
fn test_trans_load() {
    let path = "data/test_trans_load.csv";
    let test_data = "\
name,alias,power,affiliation,status\n\
Iron Man,Tony Stark,Genius Avenger,Alive\n\
Thor,Thor Odinson,God Asgardian,Alive\n\
Hulk,Bruce Banner,Strength Avenger,Alive\n";

    // Write test data to a temporary CSV file
    fs::write(path, test_data).expect("Unable to write test data file");

    // Run the trans_load function
    let result = trans_load(path);

    // Check that the result is successful and database file is created
    assert!(result.is_ok());
    assert!(fs::metadata("data/avengers.db").is_ok());

    // Clean up the test file and database
    let _ = fs::remove_file(path);
    let _ = fs::remove_file("data/avengers.db");
}

#[test]
fn test_query() {
    let path = "data/test_query.csv";
    let test_data = "\
name,alias,power,affiliation,status\n\
Captain America,Steve Rogers,Shield Avenger,Alive\n\
Black Widow,Natasha Romanoff,Spy Avenger,Deceased\n\
Hawkeye,Clint Barton,Archer Avenger,Alive\n";

    // Write test data to a temporary CSV file
    fs::write(path, test_data).expect("Unable to write test data file");

    // Load data into database
    trans_load(path).expect("Failed to load test data");

    // Define a sample query to test
    let query_string = "SELECT * FROM Avengers LIMIT 2";
    let result = query(query_string);

    // Check that the query result is successful and contains expected data
    assert!(result.is_ok());
    let result_string = result.unwrap();
    assert!(result_string.contains("Captain America"));
    assert!(result_string.contains("Steve Rogers"));

    // Clean up the test file and database
    let _ = fs::remove_file(path);
    let _ = fs::remove_file("data/avengers.db");
}
