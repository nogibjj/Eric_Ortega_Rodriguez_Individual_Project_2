from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import query

# Paths and parameters for the new dataset
dataset_path = "data/avengers.csv"
db_name = "avengers.db"
table_name = "Avengers"

# Extract
print("Extracting data from the database...")
# Extract data directly from the database
data = extract(database=db_name, table=table_name)

# Transform and load
print("Transforming and loading data...")
# Load the CSV data into the database if necessary
load(dataset=dataset_path, db_name=db_name, table_name=table_name)

# Query
print("Querying data...")
results = query(database=db_name, table=table_name)

# Print query results
print("Top 5 rows from the Avengers table:")
for row in results:
    print(row)
