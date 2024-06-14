use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn read_csv(file_path: &str) -> Result<DataFrame, PolarsError> {
    // Open the CSV file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Create a CsvReader and read the CSV file into a DataFrame
    let df = CsvReader::new(reader).finish()?;

    Ok(df)
}

pub fn print_head(df: &DataFrame, n: usize) {
    println!("{}", df.head(Some(n)));
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_read_csv_valid_file() {
//         let df = read_csv("datasets/iris.csv");
//         assert!(df.is_ok());

//         if let Ok(df) = df {
//             print_head(&df, 5);
//         }
//     }

//     #[test]
//     fn test_read_csv_invalid_file() {
//         let df = read_csv("datasets/badfile.csv");
//         assert!(df.is_err());
//     }
// }
