use std::fs;
use mylib::{extract, trans_load, query};
use std::path::Path;

fn setup() {
    // Create the `data/` directory if it doesn’t exist
    let _ = fs::create_dir_all("data");
}

#[test]
fn test_extract() {
    setup();  // Ensure `data/` directory exists

    let url = "https://people.sc.fsu.edu/~jburkardt/data/csv/addresses.csv";  // Example small CSV file for testing
    let path = "data/test_extract.csv";

    let _ = fs::remove_file(path);

    let result = extract(url, path);

    assert!(result.is_ok());
    assert!(fs::metadata(path).is_ok());

    let _ = fs::remove_file(path);
}

#[test]
fn test_trans_load() {
    setup();

    let path = "data/test_trans_load.csv";
    let test_data = "\
name,alias,power,affiliation,status\n\
Iron Man,Tony Stark,Genius Avenger,Alive\n\
Thor,Thor Odinson,God Asgardian,Alive\n\
Hulk,Bruce Banner,Strength Avenger,Alive\n";

    fs::write(path, test_data).expect("Unable to write test data file");

    let result = trans_load(path);

    assert!(result.is_ok());
    assert!(fs::metadata("data/avengers.db").is_ok());

    let _ = fs::remove_file(path);
    let _ = fs::remove_file("data/avengers.db");
}

#[test]
fn test_query() {
    setup();

    let path = "data/test_query.csv";
    let test_data = "\
name,alias,power,affiliation,status\n\
Captain America,Steve Rogers,Shield Avenger,Alive\n\
Black Widow,Natasha Romanoff,Spy Avenger,Deceased\n\
Hawkeye,Clint Barton,Archer Avenger,Alive\n";

    fs::write(path, test_data).expect("Unable to write test data file");

    // Ensure the data is loaded fresh for each test
    trans_load(path).expect("Failed to load test data");

    let query_string = "SELECT * FROM Avengers LIMIT 2";
    let result = query(query_string);

    assert!(result.is_ok());
    let result_string = result.unwrap();
    assert!(result_string.contains("Captain America"));
    assert!(result_string.contains("Steve Rogers"));

    let _ = fs::remove_file(path);
    let _ = fs::remove_file("data/avengers.db");
}
