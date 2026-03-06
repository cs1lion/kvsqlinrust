use std::collections::BTreeMap;
use crate::error::Result;
use crate::sql::engine::Transaction;
use crate::sql::executor::{Executor, ResultSet};
use crate::sql::parser::ast;
use crate::sql::parser::ast::{Expression, OrderDirection};
use crate::sql::plan::planner::Planner;
use crate::sql::schema::Table;

mod planner;
#[derive(Debug, PartialEq)]
pub enum Node {
    // 创建表
    CreateTable {
        schema: Table,
    },
    // 插入数据
    Insert {
        table_name: String,
        columns: Vec<String>,
        values: Vec<Vec<Expression>>,
    },
    // 扫描节点
    Scan {
        table_name: String,
        filter: Option<(String, Expression)>,
    },
    // 更新数据
    Update{
        table_name: String,
        source: Box<Node>,
        columns: BTreeMap<String, Expression>
    },
    // 删除数据
    Delete{
        table_name: String,
        source: Box<Node>,
    },
    // 排序节点
    Order{
        source: Box<Node>,
        order_by:Vec<(String, OrderDirection)>
    },
    // limit节点
    Limit{
        source: Box<Node>,
        limit: usize,
    },
    // offset节点
    Offset{
        source: Box<Node>,
        offset: usize,
    },
    // 投影节点
    Projection{
        source: Box<Node>,
        exprs: Vec<(Expression,Option<String>)>,
    },
    }

#[derive(Debug, PartialEq)]
pub struct Plan(pub Node);

impl Plan {
    pub fn build(stmt: ast::Statement) -> Result<Self> {
        Planner::new().build(stmt)
    }

    pub fn execute<T: Transaction + 'static>(self, txn: &mut T) -> Result<ResultSet> {
        <dyn Executor<T>>::build(self.0).execute(txn)
    }
}

#[cfg(test)]
mod tests {
    use crate::sql::plan::{Node, Plan};
    use crate::{error::Result, sql::parser::Parser};

    #[test]
    fn test_plan_create_table() -> Result<()> {
        let sql1 = "\
        create table tbl1 
        (
            a int default 100,
            b float not null,
            c varchar null,
            d bool default true);
        ";
        let stmt1 = Parser::new(sql1).parse()?;
        let p1 = Plan::build(stmt1)?;
        println!("{:?}", p1);
        let sql2 = "\
      create     table tbl1(
 a int   default 100,
b float not null,
c varchar null,
d bool default             true
      );
        ";
        let stmt2 = Parser::new(sql2).parse()?;
        let p2 = Plan::build(stmt2)?;
        println!("{:?}", p2);

        //assert_eq!(p1,p2);
        Ok(())
    }

    #[test]
    fn test_plan_insert() -> Result<()> {
        let sql1 = "insert into tbl1 values(1,2,3,'a',true);";
        let stmt1 = Parser::new(sql1).parse()?;
        let p1 = Plan::build(stmt1)?;
        println!("{:?}", p1);
        Ok(())
    }

    #[test]
    fn test_plan_select() -> Result<()> {
        let sql1 = "select * from tbl1;";
        let stmt1 = Parser::new(sql1).parse()?;
        let p1 = Plan::build(stmt1)?;
        assert_eq!(
            p1,
            Plan(Node::Scan {
                table_name: "tbl1".to_string(),
            filter: None,
            })
        );
        Ok(())
    }
}
