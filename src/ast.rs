#[derive(Debug)]
pub enum SQLStatement {
    CreateTable(CreateStatement),
    DropTable(DropStatement),
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement),
}
#[derive(Debug)]
pub struct CreateStatement {
    table_name: String,
    columns: Vec<ColumnDefinition>,
}
#[derive(Debug)]
pub struct DropStatement {
    table_name: String,
}
#[derive(Debug)]
struct SelectStatement {
    table_name: String,
    columns: Vec<String>,
}
#[derive(Debug)]
pub struct InsertStatement {
    table_name: String,
    columns: Vec<String>,
    values: Vec<Value>,
}
#[derive(Debug)]
struct UpdateStatement {
    table_name: String,
    set_clauses: Vec<SetClause>,
    where_clause: Option<WhereClause>,
}
#[derive(Debug)]
pub struct DeleteStatement {
    table_name: String,
    where_clause: Option<WhereClause>,
}
#[derive(Debug)]
pub struct ColumnDefinition {
    name: String,
}
#[derive(Debug)]
pub struct Value {
    value: String,
}
#[derive(Debug)]
struct SetClause {
    field: String,
    value: Value,
}
#[derive(Debug)]
pub struct WhereClause {
    condition: Expression,
}
#[derive(Debug)]
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

impl Value {
    pub fn new(value: String) -> Self {
        Value { value }
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
