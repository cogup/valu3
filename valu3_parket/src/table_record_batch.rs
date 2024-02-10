use arrow::array::{Array, BooleanArray, Float64Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;
use std::vec;
use valu3::prelude::*;

use crate::Table;

#[derive(Debug)]
pub enum TableRecordBatchError {
    InvalidDataType(String),
    CreateRecordBatch(String),
    WriteParquet(String),
    CloseParquet(String),
    IO(std::io::Error),
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
    pub fn record_batch(&self) -> &RecordBatch {
        &self.record_batch
    }

    pub fn schema(&self) -> &Arc<Schema> {
        &self.schema
    }

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
    pub fn load_record_batch(&mut self) -> Result<(), TableRecordBatchError> {
        match TableRecordBatch::build(self) {
            Ok(table_record_batch) => {
                self.record_batch = Some(table_record_batch);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    pub fn to_parquet(&self, file_path: &str) -> Result<(), TableRecordBatchError> {
        match &self.record_batch {
            Some(record_batch) => record_batch.to_parquet_file(file_path),
            None => Err(TableRecordBatchError::RecordNotFound),
        }
    }

    pub fn get_batch_record(&mut self) -> Result<&RecordBatch, &TableRecordBatchError> {
        match &self.record_batch {
            Some(record_batch) => Ok(record_batch.record_batch()),
            None => Err(&TableRecordBatchError::RecordNotFound),
        }
    }

    pub fn get_schema(&self) -> Result<&Arc<Schema>, TableRecordBatchError> {
        match &self.record_batch {
            Some(record_batch) => Ok(record_batch.schema()),
            None => Err(TableRecordBatchError::RecordNotFound),
        }
    }
}
