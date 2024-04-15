pub struct TableValue<T>(String, T);
pub struct TableName(String);
pub trait DBError {
    fn cause(msg: String) -> Self;
}
