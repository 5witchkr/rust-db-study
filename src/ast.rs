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
struct DropStatement {
    table_name: String,
}
#[derive(Debug)]
struct SelectStatement {
    table_name: String,
    columns: Vec<String>,
}
#[derive(Debug)]
struct InsertStatement {
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
struct DeleteStatement {
    table_name: String,
    where_clause: Option<WhereClause>,
}
#[derive(Debug)]
pub struct ColumnDefinition {
    name: String,
}
#[derive(Debug)]
struct Value {
    value: String,
}
#[derive(Debug)]
struct SetClause {
    field: String,
    value: Value,
}
#[derive(Debug)]
struct WhereClause {
    condition: Expression,
}
#[derive(Debug)]
enum Expression {
    Column(String),
    Value(Value),
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
