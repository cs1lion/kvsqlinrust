use crate::error::{Error, Result};
use crate::sql::executor::ResultSet;
use crate::sql::parser::Parser;
use crate::sql::plan::Plan;
use crate::sql::schema::Table;
use crate::sql::types::Row;

mod kv;
//抽象的 SQL 引擎层定义
pub trait Engine: Clone {
    type Transaction: Transaction;

    fn begin(&self) -> Result<Self::Transaction>;

    fn session(&self) -> Result<Session<Self>> {
        Ok(Session {
            engine: self.clone(),
        })
    }
}

pub trait Transaction {
    //提交事务
    fn commit(&self) -> Result<()>;
    //回滚事务
    fn rollback(&self) -> Result<()>;
    //创建行
    fn create_row(&mut self, table_name: String, row: Row) -> Result<()>;
    //扫描表
    fn scan_table(&self, table_name: String) -> Result<Vec<Row>>;
    //DDL相关操作
    fn create_table(&mut self, table: Table) -> Result<()>;
    //获取表信息
    fn get_table(&self, table_name: String) -> Result<Option<Table>>;
    //获取表信息，否则报错
    fn must_get_table(&self, table_name: String) -> Result<Table>{
        self.get_table(table_name.clone())?.
            ok_or(Error::Internal(format!("table {} does not exist", table_name)))
    }
}

//客户端Session定义
pub struct Session<E: Engine> {
    engine: E,
}
impl<E: Engine> Session<E> {
    //执行客户端语句
    pub fn execute(&mut self, sql: &str) -> Result<ResultSet> {
        match Parser::new(sql).parse()? {
            stmt => {
                let mut txn = self.engine.begin()?;
                //构建plan，执行SQL语句
                match Plan::build(stmt).execute(&mut txn) {
                    Ok(result) => {
                        txn.commit()?;
                        Ok(result)
                    }
                    Err(err) => {
                        txn.rollback()?;
                        Err(err)
                    }
                }
            }
        }
    }
}
