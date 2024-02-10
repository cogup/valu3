use arrow::array::{Array, Int32Array, StringArray};
use arrow::datatypes::DataType;
use arrow::record_batch::RecordBatch;
use std::sync::Arc;
use valu3::prelude::*;

use crate::Table;

macro_rules! array_to_value {
    ($array:ident, $row:ident) => {
        if $array.is_nullable() && $array.is_null($row) {
            Value::Null
        } else {
            Value::from($array.value($row))
        }
    };
}

impl From<&RecordBatch> for Table {
    fn from(batch: &RecordBatch) -> Self {
        let mut table = Self::new();

        for col in 0..batch.num_columns() {
            let col_data: &Arc<dyn Array> = batch.column(col);
            let new_col_data = (0..batch.num_rows())
                .map(|row| match col_data.data_type() {
                    &DataType::Null => Value::Null,
                    &DataType::Boolean => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::BooleanArray>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Int8 => {
                        let array = col_data.as_any().downcast_ref::<Int32Array>().unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Int16 => {
                        let array = col_data.as_any().downcast_ref::<Int32Array>().unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Int32 => {
                        let array = col_data.as_any().downcast_ref::<Int32Array>().unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Int64 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::Int64Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::UInt8 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::UInt8Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::UInt16 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::UInt16Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::UInt32 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::UInt32Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::UInt64 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::UInt64Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Float32 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::Float32Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Float64 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::Float64Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Timestamp(_, _) => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::TimestampNanosecondArray>()
                            .unwrap();

                        if array.is_nullable() && array.is_null(row) {
                            Value::Null
                        } else {
                            let value = array.value(row);
                            DateTime::from(value).to_value()
                        }
                    }
                    &DataType::Date32 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::Date32Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Date64 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::Date64Array>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Time32(_) => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::Time32SecondArray>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Time64(_) => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::Time64NanosecondArray>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Duration(_) => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::DurationNanosecondArray>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Interval(_) => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::IntervalMonthDayNanoArray>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Utf8 => {
                        let array = col_data.as_any().downcast_ref::<StringArray>().unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::LargeUtf8 => {
                        let array = col_data
                            .as_any()
                            .downcast_ref::<arrow::array::LargeStringArray>()
                            .unwrap();
                        array_to_value!(array, row)
                    }
                    &DataType::Decimal128(_, _) => {
                        let array: &arrow::array::PrimitiveArray<arrow::datatypes::Decimal128Type> =
                            col_data
                                .as_any()
                                .downcast_ref::<arrow::array::Decimal128Array>()
                                .unwrap();
                        array_to_value!(array, row)
                    }
                    _ => Value::Null,
                })
                .collect::<Vec<_>>();

            table.add_col(new_col_data);
        }

        for col in 0..batch.num_columns() {
            table.add_header_string(batch.schema().field(col).name().clone());
        }

        table
    }
}

impl From<&Vec<RecordBatch>> for Table {
    fn from(batches: &Vec<RecordBatch>) -> Self {
        let mut table = Self::new();

        for batch in batches {
            let mut batch_table = Table::from(batch);

            table.headers.append(&mut batch_table.headers);
            table.cols.append(&mut batch_table.cols);
        }

        table
    }
}
