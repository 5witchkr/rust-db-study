use crate::types::{TableName, TableValue};

enum DDL<T> {
    CreateTable {
        table_name: TableName,
        columns: Vec<TableValue<T>>,
    },
    DropTable {
        table_name: TableName,
    },
}