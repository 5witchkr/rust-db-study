use crate::types::{TableName, TableValue};

enum DML<T> {
    Insert {
        table_name: TableName,
        values: Vec<TableValue<T>>,
    },
    Select {
        table_name: TableName,
        columns: Vec<TableName>,
    },
    Update {
        table_name: TableName,
        values: Vec<TableValue<T>>,
    },
    Delete {
        table_name: TableName,
    },
}