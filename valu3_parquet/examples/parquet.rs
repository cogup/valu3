use valu3::{prelude::*, vec_value};
use valu3_parquet::Table;

enum Key {
    Name,
    Age,
    IsStudent,
}

impl Into<&str> for Key {
    fn into(self) -> &'static str {
        match self {
            Key::Name => "name",
            Key::Age => "age",
            Key::IsStudent => "is_student",
        }
    }
}

fn main() {
    let mut table = Table::new();

    table.add(Key::Name.into(), vec_value!["Alice", "Bob", "Charlie"]);
    table.add(Key::Age.into(), vec_value![25, 30, 35]);
    table.add(Key::IsStudent.into(), vec_value![true, false, true]);

    table.load_record_batch().unwrap();

    table.to_parquet("example.parquet").unwrap();

    let table = Table::from_parquet("example.parquet").unwrap();

    table.print_table();
}
