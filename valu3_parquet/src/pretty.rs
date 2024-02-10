///! Pretty printing for the Table struct.
use prettytable::{Cell, Row, Table as PrettyTable};
use crate::Table;

impl Table {
    /// Prints the table in a human-readable format.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use valu3_parquet::Table;
    /// 
    /// let mut table = Table::new();
    /// table.add("id", vec![1, 2, 3]);
    /// table.add("name", vec!["Alice", "Bob", "Charlie"]);
    /// table.print_table();
    /// ```
    /// Prints:
    /// ```text
    /// +----+ ------ +
    /// | id | name   |
    /// +----+ ------ +
    /// | 1  | Alice  |
    /// | 2  | Bob    |
    /// | 3  | Charlie|
    /// +----+ ------ +
    /// ```
    pub fn print_table(&self) {
        let mut table = PrettyTable::new();

        table.add_row(Row::new(
            self.headers.iter().map(|x| Cell::new(x)).collect(),
        ));

        for col in &self.cols {
            table.add_row(Row::new(
                col.iter().map(|x| Cell::new(&x.to_string())).collect(),
            ));
        }

        table.printstd();
    }
}