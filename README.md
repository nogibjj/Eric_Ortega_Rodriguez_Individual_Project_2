# Individual Project 2
# Eric Ortega Rodriguez 
# Rust CLI Binary with SQLite
![alt text](image-5.png)
This project is a Command Line Interface (CLI) application built with Rust that performs CRUD (Create, Read, Update, Delete) operations on a SQLite database. It demonstrates Rust's performance, memory safety, and concurrency features while interacting with a database. 

## Project Structure

- **Rust source code**: Written in Rust, the code leverages Rust's syntax and unique features, including error handling with `Result` and `Option`, memory safety, and type inference.
- **SQLite Database**: The application uses SQLite for local data storage and supports CRUD operations, allowing users to create, read, update, and delete records from the database.
- **Optimized Rust Binary**: This project includes a process for generating an optimized Rust binary using GitLab CI/CD, enabling a downloadable binary artifact.
- **Use of LLM**: An LLM (Language Model) assisted in coding, generating suggestions, and optimizing code snippets to enhance productivity and efficiency. 
- **GitLab/GitHub Actions**: A CI/CD workflow has been created to automatically test, build, and lint the code.

## Features

- **CRUD Operations**: Provides functions to create, read, update, and delete entries in an SQLite database.
- **High Performance**: Compiled as an optimized Rust binary for speed and low memory usage.
- **Automated CI/CD**: Includes a GitHub/GitLab Actions workflow for continuous integration, testing, building, and linting.

## Getting Started

### Prerequisites

- **Rust**: Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).
- **SQLite**: SQLite should be installed on your system to manage the database.

### Installation

1. Clone this repository:

    ```bash
    git clone <https://github.com/nogibjj/Eric_Ortega_Rodriguez_Individual_Project_2>
    cd <repository-directory>
    ```

2. Build the Rust binary:

    ```bash
    cargo build --release
    ```

3. Run the CLI application:

    ```bash
    ./target/release/<binary-name>  # Need to update 
    ```

### Usage

The CLI provides several commands to interact with the SQLite database. Here are some examples:

- **Create a Record**:

    ```bash
    ./target/release/<binary-name> create <data>
    ```

- **Read Records**:

    ```bash
    ./target/release/<binary-name> read
    ```

- **Update a Record**:

    ```bash
    ./target/release/<binary-name> update <id> <new-data>
    ```

- **Delete a Record**:

    ```bash
    ./target/release/<binary-name> delete <id>
    ```

Replace `<binary-name>` with the actual name of your binary.

## LLM Assistance

Throughout the development process, a Large Language Model (ChatGPT) was used to assist with the following:

- Optimizing Rust syntax
- Creating efficient error handling patterns
- Providing code suggestions for Rust best practices and memory management
- Generating boilerplate code for SQLite connection and CRUD operations
- Overall formatting

## Optimized Binary Artifact

This project includes a GitLab CI/CD pipeline that builds an optimized binary, which can be downloaded as an artifact. The optimized binary enhances performance and reduces memory usage, making the CLI more efficient.


## Short Video Demo

A video demonstration showcasing the Rust CLI application, its features, and a step-by-step usage guide can be found [here](). 