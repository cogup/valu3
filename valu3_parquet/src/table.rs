use crate::*;
use std::collections::HashMap;
use valu3::prelude::*;

/// Represents a table with headers and columns of values.
#[derive(Debug, Clone, Default)]
pub struct Table {
    pub(crate) headers: Vec<String>,
    pub(crate) cols: Vec<Vec<Value>>,
    pub(crate) record_batch: Option<TableRecordBatch>,
}

impl Table {
    /// Creates a new empty table.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let table = Table::new();
    /// ```
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            cols: Vec::new(),
            record_batch: None,
        }
    }

    /// Returns the number of rows in the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// assert_eq!(table.count_rows(), 0);
    /// ```
    pub fn count_rows(&self) -> usize {
        match self.cols.get(0) {
            Some(col) => col.len(),
            None => 0,
        }
    }

    /// Returns the number of columns in the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// assert_eq!(table.count_cols(), 0);
    /// ```
    pub fn add_col(&mut self, col: Vec<Value>) {
        self.cols.push(col);
    }

    /// Adds a column to the table and associates it with the provided header.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_col_to_header("id", vec_value![1, 2, 3]);
    /// ```
    pub fn add_col_to_header(&mut self, header: &str, col: Vec<Value>) {
        let index = self.add_header(header);

        self.add_col_to_header_index(index, col)
    }

    /// Adds a column to the table and associates it with the header at the specified index.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_col_to_header_index(0, vec_value![1, 2, 3]);
    /// ```
    pub fn add_col_to_header_index(&mut self, header_index: usize, col: Vec<Value>) {
        match self.cols.get_mut(header_index) {
            Some(cur_col) => cur_col.append(&mut col.clone()),
            None => {
                self.cols.push(col);
            }
        }
    }

    /// Pushes a value into a specified column of the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.push_item(0, 1.to_value());
    /// ```
    pub fn push_item(&mut self, col: usize, value: Value) {
        self.cols[col].push(value);
    }

    /// Pushes a value into the column associated with the specified header.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.push_item_in_header("id", 1.to_value());
    /// ```
    pub fn push_item_in_header(&mut self, header: &str, value: Value) {
        let index = self.headers.iter().position(|x| x == header);

        if let Some(index) = index {
            self.push_item(index, value);
        } else {
            let index = self.add_header(header);
            self.push_item(index, value);
        }
    }

    /// Adds a header to the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_header("id");
    /// ```
    pub fn add_header(&mut self, header: &str) -> usize {
        self.add_header_string(header.to_string())
    }

    /// Adds a header to the table using a String.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_header_string("id".to_string());
    /// ```
    pub fn add_header_string(&mut self, header: String) -> usize {
        self.headers.push(header);
        self.headers.len() - 1
    }

    /// Changes the value at the specified row and column indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.change_value(0, 0, 1.to_value());
    /// ```
    pub fn change_value(&mut self, col: usize, row: usize, value: Value) {
        self.cols[col][row] = value;
    }

    /// Adds a column with the specified header and values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    /// ```
    pub fn add(&mut self, header: &str, value: Vec<Value>) {
        self.add_header(header);
        self.add_col(value);
    }

    /// Adds a column with the specified header and values.
    pub fn extend(&mut self, table: &Table) {
        for (index, header) in table.headers.iter().enumerate() {
            let col = table.get_col(index).unwrap().clone();

            self.add(header, col);
        }
    }

    /// Loads a record batch into the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    ///
    /// assert_eq!(table.get("id").unwrap(), &vec_value![1, 2, 3]);
    /// ```
    pub fn get(&self, header: &str) -> Option<&Vec<Value>> {
        let index = self.headers.iter().position(|x| x == header);

        match index {
            Some(index) => self.get_col(index),
            None => None,
        }
    }

    /// Retrieves the header at the specified index.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_header("id");
    /// assert_eq!(table.get_header(0).unwrap(), "id");
    /// ```
    pub fn get_header(&self, index: usize) -> Option<&String> {
        self.headers.get(index)
    }

    /// Retrieves a reference to the table's headers.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add_header("id");
    /// assert_eq!(table.get_headers(), &vec!["id".to_string()]);
    /// ```
    pub fn get_headers(&self) -> &Vec<String> {
        &self.headers
    }

    /// Retrieves a reference to the table's columns.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    /// use valu3::prelude::*;
    /// use valu3::vec_value;
    ///
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    /// assert_eq!(table.get_cols(), &vec![vec_value![1, 2, 3]]);
    /// ```
    pub fn get_cols(&self) -> &Vec<Vec<Value>> {
        &self.cols
    }

    /// Retrieves the record batch associated with the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.load_record_batch().unwrap();
    /// assert!(table.get_batch_record().is_some());
    /// ```
    pub fn get_col(&self, index: usize) -> Option<&Vec<Value>> {
        self.cols.get(index)
    }

    /// Retrieves the value at the specified row and column indices.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    /// use valu3::prelude::*;
    /// use valu3::vec_value;
    ///
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    /// assert_eq!(table.get_value(0, 0).unwrap(), &1.to_value());
    /// ```
    pub fn get_value(&self, col: usize, row: usize) -> Option<&Value> {
        self.cols.get(col)?.get(row)
    }

    /// Converts the table to a HashMap representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use valu3_parquet::Table;
    ///
    /// let mut table = Table::new();
    /// table.add("id", vec_value![1, 2, 3]);
    /// table.add("name", vec_value!["Alice", "Bob", "Charlie"]);
    /// table.add("score", vec_value![85.0, 92.5, 78.3]);
    /// table.add("active", vec_value![true, false, true]);
    /// table.to_map();
    /// ```
    pub fn to_map(&self) -> HashMap<String, Vec<Value>> {
        let mut map = HashMap::new();

        for (index, col) in self.cols.iter().enumerate() {
            let header = self.get_header(index).unwrap();

            map.insert(header.clone(), col.clone());
        }

        map
    }
}

impl ToValueBehavior for Table {
    fn to_value(&self) -> Value {
        Value::from(self.to_map())
    }
}

#[cfg(test)]
mod tests {
    use valu3::vec_value;

    use super::*;

    #[test]
    fn test_value_table() {
        let mut table = Table::new();

        table.add("id", vec_value![1, 2, 3]);
        table.add("name", vec_value!["Bob", "Carol", "Ted"]);
        table.add("active", vec_value![true, false, true]);
        table.add("amount", vec_value![100.0, 150.0, 200.0]);

        table.load_record_batch().unwrap();

        let batch = table.get_batch_record().unwrap();

        let new_table = Table::from(batch);

        assert_eq!(table.headers, new_table.headers);
        assert!(table
            .cols
            .iter()
            .zip(new_table.cols.iter())
            .all(|(a, b)| a == b));
    }

    #[test]
    fn test_table_to_map() {
        let mut table = Table::new();

        table.add("id", vec_value![1, 2, 3]);
        table.add("name", vec_value!["Bob", "Carol", "Ted"]);
        table.add("active", vec_value![true, false, true]);
        table.add("amount", vec_value![100.0, 150.0, 200.0]);

        let map = table.to_map();

        let compare_map = HashMap::from([
            ("id".to_string(), vec_value![1, 2, 3]),
            ("name".to_string(), vec_value!["Bob", "Carol", "Ted"]),
            ("active".to_string(), vec_value![true, false, true]),
            ("amount".to_string(), vec_value![100.0, 150.0, 200.0]),
        ]);

        assert_eq!(map, compare_map);
    }

    #[test]
    fn test_table_to_value() {
        let mut table = Table::new();

        table.add("id", vec_value![1, 2, 3]);
        table.add("name", vec_value!["Bob", "Carol", "Ted"]);
        table.add("active", vec_value![true, false, true]);
        table.add_header("amount");
        table.add_col_to_header("amount", vec_value![100.0, 150.0, 200.0]);

        let value = table.to_value();

        let compare_map = HashMap::from([
            ("id".to_string(), vec_value![1, 2, 3]),
            ("name".to_string(), vec_value!["Bob", "Carol", "Ted"]),
            ("active".to_string(), vec_value![true, false, true]),
            ("amount".to_string(), vec_value![100.0, 150.0, 200.0]),
        ])
        .to_value();

        assert_eq!(value, compare_map);
    }
}
