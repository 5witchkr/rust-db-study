pub struct TableValue<T>(String, T);
pub struct TableName(String);
pub trait DBError {
    fn cause(msg: &str) -> Self;
    fn and_cause(self, msg: &str) -> Self;
}
