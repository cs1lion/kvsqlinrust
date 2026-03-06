use bincode::{Decode, Encode};
use crate::sql::types::{DataType, Row, Value};
use serde::{Deserialize, Serialize};
use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Serialize, Deserialize,Encode,Decode)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}
impl Table{
    //校验表的有效性
    pub fn validate(&self)->Result<()>{
        //校验是否有列信息
        if self.columns.is_empty() {
            return Err(Error::Internal(format!(
                "table {} does not have columns",
                self.name
            )));
        }
        //校验是否有主键
        match self.columns.iter().filter(|c|c.primary_key).count(){
            1=>{},
            0=>return Err(Error::Internal(format!(
                "table {} has no primary keys",
                self.name,
            ))),
            _=>return Err(Error::Internal(format!(
                "table {} has more than one primary keys",
                self.name,
            ))),
        }
        // 校验列信息
        for col in &self.columns{
            // 主键不能为空
            if col.primary_key && col.nullable{
            return Err(Error::Internal(format!(
                "Primary key {} is not nullable in table {}",
                col.name,
                self.name,
            ))
            );
            }
            // 校验默认值是否和列类型匹配
            if let Some(default) = &col.default{
                match default.datatype(){
                    Some(dt)=>{
                        if dt!=col.datatype{
                            return Err(Error::Internal(format!(
                                "default value type for column {} in table {} mismatch ",
                                col.name,
                                self.name,
                            )));
                        }
                    }
                    None=>{},
                }
            }
        }
        Ok(())
    }

    pub fn get_primary_key(&self,row:&Row)->Result<Value>{
        let pos = self.columns.iter().position(|c|c.primary_key).expect("primary key not found");
        Ok(row[pos].clone())
    }

    pub fn get_column_index(&self,col_name: &str)->Result<usize>{
        self.columns.iter().position(|c|c.name==col_name).ok_or(Error::Internal(format!("column {} not found",col_name)))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize,Decode,Encode)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
    pub nullable: bool,
    pub default: Option<Value>,
    pub primary_key: bool,
}
