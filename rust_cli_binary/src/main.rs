use mylib::{extract, trans_load, query};

fn main() {
    // Paths and parameters for the new dataset
    let dataset_url = "https://github.com/fivethirtyeight/data/blob/refs/heads/master/avengers/avengers.csv";
    let dataset_path = "data/avengers.csv";
    let db_name = "data/avengers.db";
    let table_name = "Avengers";

    // Extract
    println!("Extracting data from the URL...");
    match extract(dataset_url, dataset_path) {
        Ok(path) => println!("Data extracted successfully to {}", path),
        Err(e) => eprintln!("Error extracting data: {}", e),
    }

    // Transform and load
    println!("Transforming and loading data...");
    if let Err(e) = trans_load(dataset_path) {
        eprintln!("Error transforming and loading data: {}", e);
    } else {
        println!("Data transformed and loaded successfully.");
    }

    // Query
    println!("Querying data...");
    let query_string = format!("SELECT * FROM {} LIMIT 5;", table_name);
    match query(&query_string) {
        Ok(results) => println!("Top 5 rows from the Avengers table:\n{}", results),
        Err(e) => eprintln!("Error querying data: {}", e),
    }
    // Delete rows with a specific condition
    println!("Deleting rows where status is 'Deceased'...");
    if let Err(e) = delete_rows("status = 'Deceased'") {
        eprintln!("Error deleting rows: {}", e);
    } else {
        println!("Rows deleted successfully.");
    }

}
