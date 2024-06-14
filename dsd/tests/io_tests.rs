#![allow(unused_imports)]
use assert_cmd::Command;
use dsd::io::reader::{print_head, read_csv};
use std::fs::File;
use std::io::{self, Write};
use std::process::Command as StdCommand;

#[test]
fn test_read_csv_valid_file() {
    // Path to the sample CSV file for testing
    let file_path = "datasets/iris.csv";

    // Call the read_csv function
    let result = read_csv(file_path);

    // Assert that the result is Ok
    assert!(result.is_ok());

    // Optionally, print the DataFrame for inspection
    if let Ok(df) = result {
        // Print the first 5 rows of the DataFrame
        print_head(&df, 5);
    }
}

#[test]
fn test_read_csv_invalid_file() {
    // Path to a non-existent CSV file
    let file_path = "datasets/badfile.csv";

    // Call the read_csv function
    let result = read_csv(file_path);

    // Assert that the result is an error
    assert!(result.is_err());
}

#[test]
fn test_print_head() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary CSV file for testing
    let file_path = "datasets/iris.csv";
    let _df = read_csv(file_path).unwrap();

    // Capture the output of the print_head function
    let mut cmd = StdCommand::new("cargo");
    cmd.arg("test")
        .arg("--")
        .arg("test_read_csv_valid_file")
        .arg("--nocapture");

    // Execute the command and capture the output
    let output = cmd.output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    // Check that the output contains the expected first 5 rows
    assert!(output_str.contains("shape: (5,"));
    Ok(())
}
