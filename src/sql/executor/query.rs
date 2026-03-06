use std::cmp::Ordering;
use std::collections::HashMap;
use crate::sql::engine::Transaction;
use crate::sql::executor::{Executor, ResultSet};
use crate::sql::parser::ast::{Expression, OrderDirection};

pub struct Scan {
    table_name: String,
    filter: Option<(String, Expression)>,
}

impl Scan {
    pub fn new(table_name: String, filter: Option<(String, Expression)>) -> Box<Self> {
        Box::new(Self { table_name, filter })
    }
}

impl<T: Transaction> Executor<T> for Scan {
    fn execute(self: Box<Self>, txn: &mut T) -> crate::error::Result<ResultSet> {
        let table = txn.must_get_table(self.table_name.clone())?;
        let rows = txn.scan_table(self.table_name.clone(),self.filter)?;
        Ok(ResultSet::Scan {
            columns: { table.columns.into_iter().map(|c| c.name.clone()).collect() },
            rows,
        })
    }
}

pub struct Order<T:Transaction>{
    source: Box<dyn Executor<T>>,
    order_by: Vec<(String,OrderDirection)>
}

impl<T:Transaction> Order<T>{
    pub fn new(source: Box<dyn Executor<T>>, order_by: Vec<(String,OrderDirection)>) -> Box<Self>{
        Box::new(Self{
            source,
            order_by,
        })
    }
}

impl<T:Transaction> Executor<T> for Order<T>{
    fn execute(self: Box<Self>, txn: &mut T) -> crate::error::Result<ResultSet> {
        match self.source.execute(txn)? {
            ResultSet::Scan {columns, mut rows } => {
                //找到orderby列和表列的对应
               let mut order_col_index = HashMap::new();
                for (i,(col_name,_)) in self.order_by.iter().enumerate(){
                      match columns.iter().position(|c| *c == *col_name){
                      Some(pos) => {
                        order_col_index.insert(i,pos);
                      }
                          None => return Err(crate::error::Error::Internal(format!("Order column {} not found in table",col_name))),
                      }

                }
                rows.sort_by(|col1,col2|{
                    for (i,(_,ord)) in self.order_by.iter().enumerate(){
                        let pos = order_col_index.get(&i).unwrap();
                        let x = &col1[*pos];
                        let y = &col2[*pos];
                        match x.partial_cmp(y){
                            Some(Ordering::Equal) => {},
                            Some(o) => return if *ord == OrderDirection::Asc {o} else {o.reverse()},
                            None => {},
                        }
                    }
                    Ordering::Equal
                });
                Ok(ResultSet::Scan {
                    columns,
                    rows,
                })
            },
            _ => Err(crate::error::Error::Internal("Order only support Scan".to_string()))
        }
    }
}

pub struct Limit<T:Transaction>{
    source: Box<dyn Executor<T>>,
    limit: usize,
}
impl<T:Transaction> Limit<T>{
    pub fn new (source: Box<dyn Executor<T>>, limit: usize) -> Box<Self>{
        Box::new(Self{
            source,
            limit,
        })
    }
}

impl<T:Transaction> Executor<T> for Limit<T> {
    fn execute(self: Box<Self>, txn: &mut T) -> crate::error::Result<ResultSet> {
        match self.source.execute(txn)? {
            ResultSet::Scan {columns,rows} =>{
                Ok(ResultSet::Scan {
                    columns,
                    rows: rows.into_iter().take(self.limit).collect(),
                })
            },
            _=> Err(crate::error::Error::Internal("Limit only support Scan".to_string()))
            }
        }
    }

pub struct Offset<T:Transaction>{
    source: Box<dyn Executor<T>>,
    offset: usize,
}
impl<T:Transaction> Offset<T>{
    pub fn new (source: Box<dyn Executor<T>>, offset: usize) -> Box<Self>{
        Box::new(Self{
            source,
            offset,
        })
    }
}
impl<T:Transaction> Executor<T> for Offset<T> {
    fn execute(self: Box<Self>, txn: &mut T) -> crate::error::Result<ResultSet> {
        match self.source.execute(txn)? {
            ResultSet::Scan {columns,rows } => {
                Ok(ResultSet::Scan {
                    columns,
                    rows: rows.into_iter().skip(self.offset).collect(),
                })
            },
            _=> Err(crate::error::Error::Internal("Offset only support Scan".to_string()))
            }
        }
    }

pub struct Projection<T:Transaction>{
    source: Box<dyn Executor<T>>,
    exprs: Vec<(Expression,Option<String>)>,
}

impl<T:Transaction> Projection<T> {
    pub fn new(source: Box<dyn Executor<T>>,exprs:Vec<(Expression,Option<String>)>) -> Box<Self>{
        Box::new(Self{
            source,
            exprs,
        })

    }
}

impl<T:Transaction> Executor<T> for Projection<T>{
    fn execute(self: Box<Self>, txn: &mut T) -> crate::error::Result<ResultSet> {
        match self.source.execute(txn)?{
            ResultSet::Scan{columns,rows}=>{
                let mut selected = Vec::new();
                let mut new_columns = Vec::new();
                for (expr,alias) in self.exprs{
                    if let Expression::Field(col_name) = expr{
                       let pos =  match columns.iter().position(|c|*c == col_name){
                            Some(pos) => pos,
                            None => return Err(crate::error::Error::Internal(format!("Column {} not in result set",col_name)))
                        };
                        selected.push(pos);
                        new_columns.push(if alias.is_some(){alias.unwrap()} else {col_name})
                    }
                }
                let mut new_rows = Vec::new();
                for row in rows.into_iter(){
                    let mut new_row = Vec::new();
                    for i in selected.iter(){
                        new_row.push(row[*i].clone())
                    }
                    new_rows.push(new_row);
                }
                Ok(ResultSet::Scan {
                columns: new_columns,
                rows: new_rows,
                })
            }
            _ => Err(crate::error::Error::Internal("Projection only support Scan".to_string()))
        }
    }
}