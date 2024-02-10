//! # Apache Arrow Parquet Integration with Valu3
//!
//! This library provides an integration layer between Apache Arrow's Parquet 
//! format and the `valu3` library, facilitating seamless data manipulation 
//! and transformation between Parquet files and `valu3`'s flexible `Value` structures. 
//! It offers convenient methods for working with tabular data, allowing for 
//! easy conversions and operations on datasets.
//!
//! ## Features
//! 
//! - `prettytable`: Enables pretty printing of tables using the `prettytable` crate.
//! 
//! ## Examples
//!
//! ### Case 1
//!
//! ```
//! use valu3_parquet::Table;
//!
//! let mut table = Table::new();
//! table.add("id", vec_value![1, 2, 3]);
//! table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
//!
//! assert_eq!(table.count_rows(), 3);
//! ```
//!
//! ### Case 2
//!
//! ```
//! use valu3_parquet::Table;
//!
//! let mut table = Table::new();
//! table.add("id", vec_value![1, 2, 3]);
//! table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
//! table.add("score", vec_value![85.0, 92.5, 78.3]);
//!
//! // Access individual values
//! let name = table.get_value(1, 0).unwrap();
//!
//! assert_eq!(name.to_string(), "Bob");
//! ```
//!
//! ### Case 3
//!
//! ```
//! use valu3_parquet::Table;
//!
//! let mut table = Table::new();
//! table.add("id", vec_value![1, 2, 3]);
//! table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
//! table.add("score", vec_value![85.0, 92.5, 78.3]);
//!
//! // Convert table to Parquet file
//! table.to_parquet("data.parquet").expect("Failed to write Parquet file");
//!
//! // Read Parquet file back into a table
//! let new_table = Table::from_parquet("data.parquet").expect("Failed to read Parquet file");
//!
//! // Perform operations on the new table
//! assert_eq!(table.get_headers(), new_table.get_headers());
//! assert_eq!(table.get_cols(), new_table.get_cols());
//! ```
mod table;
mod from;
mod table_record_batch;
pub use table::*;
pub use from::*;
pub use table_record_batch::*;

#[cfg(feature = "prettytable")]
mod pretty;
#[cfg(feature = "prettytable")]
pub use pretty::*;

