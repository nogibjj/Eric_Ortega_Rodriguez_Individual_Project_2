use mylib::{extract, load, query};

fn main() {
    // Paths and parameters for the new dataset
    let dataset_path = "data/avengers.csv";
    let db_name = "avengers.db";
    let table_name = "Avengers";

    // Extract
    println!("Extracting data from the database...");
    match extract(db_name, table_name) {
        Ok(data) => {
            println!("Extracted data successfully.");
            for row in data.iter().take(5) {
                println!("{:?}", row);
            }
        }
        Err(e) => eprintln!("Error extracting data: {}", e),
    }

    // Transform and load
    println!("Transforming and loading data...");
    if let Err(e) = load(dataset_path, db_name, table_name) {
        eprintln!("Error loading data: {}", e);
    } else {
        println!("Data loaded successfully.");
    }

    // Query
    println!("Querying data...");
    match query(db_name, table_name) {
        Ok(results) => {
            println!("Top 5 rows from the Avengers table:");
            for row in results.iter().take(5) {
                println!("{:?}", row);
            }
        }
        Err(e) => eprintln!("Error querying data: {}", e),
    }
}
