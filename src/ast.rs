#[derive(Debug, PartialEq)]
pub enum SQLStatement {
    CreateTable(CreateStatement),
    DropTable(DropStatement),
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
}
#[derive(Debug, PartialEq)]
pub struct CreateStatement {
    table_name: String,
    columns: Vec<ColumnDefinition>,
}
#[derive(Debug, PartialEq)]
pub struct DropStatement {
    table_name: String,
}
#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    table_name: String,
    columns: Vec<String>,
}
#[derive(Debug, PartialEq)]
pub struct InsertStatement {
    table_name: String,
    columns: Vec<String>,
    values: Vec<Value>,
}
#[derive(Debug, PartialEq)]
pub struct UpdateStatement {
    table_name: String,
    set_clauses: Vec<SetClause>,
    where_clause: Option<WhereClause>,
}
#[derive(Debug, PartialEq)]
pub struct DeleteStatement {
    table_name: String,
    where_clause: Option<WhereClause>,
}
#[derive(Debug, PartialEq)]
pub struct ColumnDefinition {
    name: String,
}
#[derive(Debug, PartialEq)]
pub enum Value {
    StrValue(String),
    NumValue(f64),
}
#[derive(Debug, PartialEq)]
pub struct SetClause {
    field: String,
    value: Value,
}
#[derive(Debug, PartialEq)]
pub struct WhereClause {
    condition: Expression,
}
#[derive(Debug, PartialEq)]
pub struct Expression {
    column: String,
    value: Value,
}

impl CreateStatement {
    pub fn new(table_name: String, columns: Vec<ColumnDefinition>) -> Self {
        CreateStatement {
            table_name,
            columns,
        }
    }
}

impl ColumnDefinition {
    pub fn new(name: String) -> Self {
        ColumnDefinition { name }
    }
}

impl DropStatement {
    pub fn new(table_name: String) -> Self {
        DropStatement { table_name }
    }
}

impl InsertStatement {
    pub fn new(table_name: String, columns: Vec<String>, values: Vec<Value>) -> Self {
        InsertStatement {
            table_name,
            columns,
            values,
        }
    }
}

impl DeleteStatement {
    pub fn new(table_name: String, where_clause: Option<WhereClause>) -> Self {
        DeleteStatement {
            table_name,
            where_clause,
        }
    }
}

impl WhereClause {
    pub fn new(condition: Expression) -> Self {
        WhereClause { condition }
    }
}

impl Expression {
    pub fn new(column: String, value: Value) -> Self {
        Expression { column, value }
    }
}

impl UpdateStatement {
    pub fn new(
        table_name: String,
        set_clauses: Vec<SetClause>,
        where_clause: Option<WhereClause>,
    ) -> Self {
        UpdateStatement {
            table_name,
            set_clauses,
            where_clause,
        }
    }
}

impl SetClause {
    pub fn new(field: String, value: Value) -> Self {
        SetClause { field, value }
    }
}

impl SelectStatement {
    pub fn new(table_name: String, columns: Vec<String>) -> Self {
        SelectStatement { table_name, columns }
    }
}

