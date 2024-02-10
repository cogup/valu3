///! This module provides the TableRecordBatch struct, which is a wrapper around the Arrow RecordBatch.
///! It also provides methods to convert a Table to a TableRecordBatch and to write a TableRecordBatch to a Parquet file.
///! It also provides methods to read a Parquet file and convert it to a Table.
/// 
///! Example:
///! ```
///! use valu3_parquet::Table;
///! use valu3::vec_value;
///! 
///! let mut table = Table::new();
///! table.add("id", vec_value![1, 2, 3]);
///! table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
///! 
///! table.load_record_batch().unwrap();
///! 
///! let file_path = "test_table_record_batch_parquet.parquet";
///! 
///! assert!(table.to_parquet(file_path).is_ok());
///! 
///! let table = Table::from_parquet(file_path).unwrap();
///! 
///! assert_eq!(
///!    table.get_headers(),
///!   &vec!["id".to_string(), "name".to_string()]
///! );
///! 
///! assert_eq!(
///!   table.get_cols(),
///!  &vec![
///!     vec_value![1, 2, 3],
///!    vec_value!["Alice", "Bob", "Charlie"]
///! ]
///! );
///! 
///! std::fs::remove_file(file_path).unwrap();
///! ``` 
use arrow::array::{Array, BooleanArray, Float64Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;
use std::sync::Arc;
use std::vec;
use valu3::prelude::*;

use crate::Table;

#[derive(Debug)]
pub enum TableRecordBatchError {
    /// Error creating record batch
    InvalidDataType(String),
    /// Error creating record batch
    CreateRecordBatch(String),
    /// Error writing record batch
    WriteParquet(String),
    /// Error closing parquet
    CloseParquet(String),
    /// Error reading parquet, writing parquet, or closing parquet
    IO(std::io::Error),
    /// Error reading parquet
    ParquetRead(parquet::errors::ParquetError),
    /// Error reading parquet
    ArrowError(arrow::error::ArrowError),
    /// Record batch not found
    RecordNotFound,
}

macro_rules! value_get_i32 {
    ($value:expr) => {
        match $value.get_i32() {
            Some(value) => value,
            None => match $value.get_f64() {
                Some(value) => value as i32,
                None => {
                    return Err(TableRecordBatchError::InvalidDataType(
                        format!(
                            "A list that starts with i32 can only work with i32 or float64. Current value: {}",
                            $value
                        ),
                    ))
                }
            },
        }
    };
}

macro_rules! value_get_f64 {
    ($value:expr) => {
        match $value.get_f64() {
            Some(value) => value,
            None => match $value.get_i32() {
                Some(value) => value as f64,
                None => {
                    return Err(TableRecordBatchError::InvalidDataType(
                        format!(
                            "A list that starts with float64 can only work with f64 or i32. Current value: {}",
                            $value
                        ),
                    ))
                }
            },
        }
    };
}

#[derive(Debug, Clone)]
pub struct TableRecordBatch {
    record_batch: RecordBatch,
    schema: Arc<Schema>,
}

impl TableRecordBatch {
    /// Create a new TableRecordBatch from a RecordBatch and a Schema
    pub fn record_batch(&self) -> &RecordBatch {
        &self.record_batch
    }

    /// Create a new TableRecordBatch from a RecordBatch and a Schema
    pub fn schema(&self) -> &Arc<Schema> {
        &self.schema
    }

    /// Create a new TableRecordBatch from a RecordBatch and a Schema
    pub fn build(value_table: &Table) -> Result<Self, TableRecordBatchError> {
        let mut schema_types: Vec<DataType> = Vec::new();

        let mut columns: Vec<Arc<dyn Array>> = Vec::new();

        macro_rules! columns_push {
            ($col_is_nullable:expr, $values:expr,$type:ident) => {
                if $col_is_nullable {
                    columns.push(Arc::new($type::from($values)));
                } else {
                    columns.push(Arc::new($type::from(
                        $values
                            .iter()
                            .map(|x| x.clone().unwrap())
                            .collect::<Vec<_>>(),
                    )));
                }
            };
        }

        for col in value_table.cols.iter() {
            let mut col_iter = col.iter();
            let mut total_prepend_nulls = 0;

            while let Some(item) = col_iter.next() {
                match item {
                    Value::String(value) => {
                        schema_types.push(DataType::Utf8);

                        let mut col_is_nullable = total_prepend_nulls > 0;

                        let mut values = vec![None; total_prepend_nulls];
                        values.push(Some(value.to_string()));

                        while let Some(value) = col_iter.next() {
                            match &value {
                                Value::String(value) => {
                                    values.push(Some(value.to_string()));
                                }
                                _ => {
                                    if !col_is_nullable {
                                        col_is_nullable = true;
                                    }

                                    values.push(None)
                                }
                            }
                        }

                        columns_push!(col_is_nullable, values, StringArray);

                        break;
                    }
                    Value::Number(value) => {
                        if value.is_float() {
                            schema_types.push(DataType::Float64);

                            let mut col_is_nullable = total_prepend_nulls > 0;

                            let mut values = vec![None; total_prepend_nulls];
                            values.push(Some(value_get_f64!(value)));

                            while let Some(value) = col_iter.next() {
                                match &value {
                                    Value::Number(value) => {
                                        values.push(Some(value_get_f64!(value)));
                                    }
                                    _ => {
                                        if !col_is_nullable {
                                            col_is_nullable = true;
                                        }

                                        values.push(None)
                                    }
                                }
                            }

                            columns_push!(col_is_nullable, values, Float64Array);

                            break;
                        } else {
                            schema_types.push(DataType::Int32);

                            let mut col_is_nullable = total_prepend_nulls > 0;

                            let mut values = vec![None; total_prepend_nulls];
                            values.push(Some(value_get_i32!(value)));

                            while let Some(value) = col_iter.next() {
                                match &value {
                                    Value::Number(value) => {
                                        values.push(Some(value_get_i32!(value)));
                                    }
                                    _ => {
                                        if !col_is_nullable {
                                            col_is_nullable = true;
                                        }

                                        values.push(None)
                                    }
                                }
                            }

                            columns_push!(col_is_nullable, values, Int32Array);

                            break;
                        }
                    }
                    Value::Boolean(value) => {
                        schema_types.push(DataType::Boolean);

                        let mut col_is_nullable = total_prepend_nulls > 0;

                        let mut values = vec![None; total_prepend_nulls];
                        values.push(Some(*value));

                        while let Some(value) = col_iter.next() {
                            match &value {
                                Value::Boolean(value) => {
                                    values.push(Some(*value));
                                }
                                _ => {
                                    if !col_is_nullable {
                                        col_is_nullable = true;
                                    }

                                    values.push(None)
                                }
                            }
                        }

                        columns_push!(col_is_nullable, values, BooleanArray);

                        break;
                    }
                    Value::Object(_) | Value::Array(_) => {
                        schema_types.push(DataType::Utf8);

                        let mut col_is_nullable = total_prepend_nulls > 0;

                        let mut values = vec![None; total_prepend_nulls];
                        values.push(Some(item.to_json(JsonMode::Inline)));

                        while let Some(value) = col_iter.next() {
                            match &value {
                                Value::Object(_) => {
                                    values.push(Some(value.to_json(JsonMode::Inline)));
                                }
                                _ => {
                                    if !col_is_nullable {
                                        col_is_nullable = true;
                                    }

                                    values.push(None)
                                }
                            }
                        }

                        columns_push!(col_is_nullable, values, StringArray);

                        break;
                    }
                    _ => {
                        total_prepend_nulls += 1;
                    }
                };
            }
        }

        let schema = {
            let fields = value_table
                .headers
                .iter()
                .enumerate()
                .map(|(index, header)| Field::new(header, schema_types[index].clone(), true))
                .collect::<Vec<_>>();

            Arc::new(Schema::new(fields))
        };

        Ok(match RecordBatch::try_new(schema.clone(), columns) {
            Ok(record_batch) => Self {
                record_batch,
                schema,
            },
            Err(error) => {
                return Err(TableRecordBatchError::CreateRecordBatch(format!(
                    "Error creating record batch: {}",
                    error
                )))
            }
        })
    }

    pub fn to_parquet_file(&self, file_path: &str) -> Result<(), TableRecordBatchError> {
        let file = match std::fs::File::create(file_path) {
            Ok(file) => file,
            Err(error) => return Err(TableRecordBatchError::IO(error)),
        };

        let mut writer = match parquet::arrow::ArrowWriter::try_new(file, self.schema.clone(), None)
        {
            Ok(writer) => writer,
            Err(error) => {
                return Err(TableRecordBatchError::CreateRecordBatch(format!(
                    "Error creating record batch: {}",
                    error
                )))
            }
        };

        if let Err(err) = writer.write(&self.record_batch) {
            return Err(TableRecordBatchError::WriteParquet(format!(
                "Error writing record batch: {}",
                err
            )));
        }

        if let Err(err) = writer.close() {
            return Err(TableRecordBatchError::CloseParquet(format!(
                "Error closing parquet: {}",
                err
            )));
        }

        Ok(())
    }
}

impl Table {
    /// Load the TableRecordBatch from the Table
    /// 
    /// # Example
    /// 
    /// ```
    /// use valu3_parquet::Table;
    /// use valu3::vec_value;
    /// 
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    /// table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
    /// 
    /// table.load_record_batch().unwrap();
    /// ```
    pub fn load_record_batch(&mut self) -> Result<(), TableRecordBatchError> {
        match TableRecordBatch::build(self) {
            Ok(table_record_batch) => {
                self.record_batch = Some(table_record_batch);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    /// Convert the Table to a Parquet file
    /// 
    /// # Example
    /// 
    /// ```
    /// use valu3_parquet::Table;
    /// use valu3::vec_value;
    /// 
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    /// table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
    /// 
    /// table.load_record_batch().unwrap();
    /// 
    /// let file_path = "test_table_record_batch_parquet.parquet";
    /// 
    /// assert!(table.to_parquet(file_path).is_ok());
    /// 
    /// std::fs::remove_file(file_path).unwrap();
    /// ```
    pub fn to_parquet(&self, file_path: &str) -> Result<(), TableRecordBatchError> {
        match &self.record_batch {
            Some(record_batch) => record_batch.to_parquet_file(file_path),
            None => Err(TableRecordBatchError::RecordNotFound),
        }
    }

    /// Convert a Parquet file to a Table
    /// 
    /// # Example
    /// 
    /// ```
    /// use valu3_parquet::Table;
    /// 
    /// let mut table = Table::from_parquet("test_table_record_batch_parquet.parquet").unwrap();
    /// 
    /// assert_eq!(
    ///   table.get_headers(),
    ///  &vec!["id".to_string(), "name".to_string()]
    /// );
    /// 
    /// assert_eq!(
    ///  table.get_cols(),
    /// &vec![
    ///   vec_value![1, 2, 3],
    /// vec_value!["Alice", "Bob", "Charlie"]
    /// ]
    /// );
    /// ```
    pub fn from_parquet(file_path: &str) -> Result<Self, TableRecordBatchError> {
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(error) => return Err(TableRecordBatchError::IO(error)),
        };

        let parquet_reader = match ParquetRecordBatchReaderBuilder::try_new(file) {
            Ok(parquet_reader) => match parquet_reader.with_batch_size(8192).build() {
                Ok(parquet_reader) => parquet_reader,
                Err(error) => return Err(TableRecordBatchError::ParquetRead(error)),
            },
            Err(error) => return Err(TableRecordBatchError::ParquetRead(error)),
        };

        let mut batches: Vec<RecordBatch> = Vec::new();

        for batch in parquet_reader {
            batches.push(match batch {
                Ok(batch) => batch,
                Err(error) => return Err(TableRecordBatchError::ArrowError(error)),
            });
        }

        Ok(Self::from(&batches))
    }

    /// Get the RecordBatch from the Table
    /// 
    /// # Example
    /// 
    /// ```
    /// use valu3_parquet::Table;
    /// use valu3::vec_value;
    /// 
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    /// table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
    /// 
    /// table.load_record_batch().unwrap();
    /// 
    /// let record_batch = table.get_batch_record().unwrap();
    /// 
    /// assert_eq!(record_batch.num_columns(), 2);
    /// assert_eq!(record_batch.num_rows(), 3);
    /// ```
    pub fn get_batch_record(&mut self) -> Result<&RecordBatch, &TableRecordBatchError> {
        match &self.record_batch {
            Some(record_batch) => Ok(record_batch.record_batch()),
            None => Err(&TableRecordBatchError::RecordNotFound),
        }
    }

    /// Get the Schema from the Table
    /// 
    /// # Example
    /// 
    /// ```
    /// use valu3_parquet::Table;
    /// use valu3::vec_value;
    /// 
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    /// table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
    /// 
    /// table.load_record_batch().unwrap();
    /// 
    /// let schema = table.get_schema().unwrap();
    /// 
    /// assert_eq!(schema.fields().len(), 2);
    /// assert_eq!(schema.field(0).name(), "id");
    /// assert_eq!(schema.field(1).name(), "name");
    /// ```
    pub fn get_schema(&self) -> Result<&Arc<Schema>, TableRecordBatchError> {
        match &self.record_batch {
            Some(record_batch) => Ok(record_batch.schema()),
            None => Err(TableRecordBatchError::RecordNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use valu3::vec_value;

    #[test]
    fn test_table_record_batch() {
        let mut table = Table::new();
        table.add("id", vec_value![1, 2, 3]);
        table.add("name", vec_value!["Alice", "Bob", "Charlie"]);

        let record_batch = TableRecordBatch::build(&table).unwrap();

        assert_eq!(record_batch.schema().fields().len(), 2);
        assert_eq!(record_batch.schema().field(0).name(), "id");
        assert_eq!(record_batch.schema().field(1).name(), "name");

        let record_batch = record_batch.record_batch();

        assert_eq!(record_batch.num_columns(), 2);
        assert_eq!(record_batch.num_rows(), 3);

        let id = record_batch.column(0);
        let name = record_batch.column(1);

        assert_eq!(id.len(), 3);
        assert_eq!(name.len(), 3);

        let id = id.as_any().downcast_ref::<Int32Array>().unwrap();
        let name = name.as_any().downcast_ref::<StringArray>().unwrap();

        assert_eq!(id.value(0), 1);
        assert_eq!(id.value(1), 2);
        assert_eq!(id.value(2), 3);

        assert_eq!(name.value(0), "Alice");
        assert_eq!(name.value(1), "Bob");
        assert_eq!(name.value(2), "Charlie");
    }

    #[test]
    fn test_table_record_batch_parquet() {
        let mut table = Table::new();
        table.add("id", vec_value![1, 2, 3]);
        table.add("name", vec_value!["Alice", "Bob", "Charlie"]);

        table.load_record_batch().unwrap();

        let file_path = "test_table_record_batch_parquet.parquet";

        assert!(table.to_parquet(file_path).is_ok());

        let table = Table::from_parquet(file_path).unwrap();

        assert_eq!(
            table.get_headers(),
            &vec!["id".to_string(), "name".to_string()]
        );

        assert_eq!(
            table.get_cols(),
            &vec![
                vec_value![1, 2, 3],
                vec_value!["Alice", "Bob", "Charlie"]
            ]
        );

        std::fs::remove_file(file_path).unwrap();
    }
}
