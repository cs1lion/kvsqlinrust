use crate::error::Result;
use crate::sql::engine::Transaction;
use crate::sql::executor::mutation::Insert;
use crate::sql::executor::query::Scan;
use crate::sql::plan::Node;
use crate::sql::types::Row;
use schema::CreateTable;
use std::ptr::addr_eq;

mod mutation;
mod query;
mod schema;
//执行器定义
pub trait Executor<T: Transaction> {
    fn execute(self:Box<Self>, txn: &mut T) -> Result<ResultSet>;
}

impl<T: Transaction> dyn Executor<T> {
    pub fn build(node: Node) -> Box<dyn Executor<T>> {
        match node {
            Node::CreateTable { schema } => CreateTable::new(schema),
            Node::Insert {
                table_name,
                columns,
                values,
            } => Insert::new(table_name, columns, values),
            Node::Scan { table_name } => Scan::new(table_name),
        }
    }
}

//执行结果集
#[derive(Debug)]
pub enum ResultSet {
    CreateTable { table_name: String },
    Insert { count: usize },
    Scan { columns: Vec<String>, rows: Vec<Row> },
}
