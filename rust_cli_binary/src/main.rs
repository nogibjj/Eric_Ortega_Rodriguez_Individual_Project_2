use mylib::extract::extract;
use mylib::transform_load::load;
use mylib::query::query;

fn main() {
    // Paths and parameters for the new dataset
    let dataset_path = "data/avengers.csv";
    let db_name = "avengers.db";
    let table_name = "Avengers";

    // Extract
    println!("Extracting data from the database...");
    // Extract data directly from the database
    let data = extract(db_name, table_name);
    
    // Transform and load
    println!("Transforming and loading data...");
    // Load the CSV data into the database if necessary
    load(dataset_path, db_name, table_name);

    // Query
    println!("Querying data...");
    let results = query(db_name, table_name);

    // Print query results
    println!("Top 5 rows from the Avengers table:");
    for row in results.iter().take(5) {
        println!("{:?}", row);
    }
}
