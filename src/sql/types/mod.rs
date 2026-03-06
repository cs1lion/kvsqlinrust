use std::cmp::Ordering;
use std::fmt::Display;
use bincode::{Decode, Encode};
use crate::sql::parser::ast::{Consts, Expression};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize,Decode,Encode)]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,Decode,Encode)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl Value {
    pub fn from_expression(expr: Expression) -> Self {
        match expr {
            Expression::Consts(Consts::Null) => Self::Null,
            Expression::Consts(Consts::Boolean(b)) => Self::Boolean(b),
            Expression::Consts(Consts::Integer(i)) => Self::Integer(i),
            Expression::Consts(Consts::Float(f)) => Self::Float(f),
            Expression::Consts(Consts::String(s)) => Self::String(s),
            _ => unreachable!()
        }
    }

    pub fn datatype(&self) -> Option<DataType> {
        match self {
            Self::Null => None,
            Self::Boolean(_) => Some(DataType::Boolean),
            Self::Integer(_) => Some(DataType::Integer),
            Self::Float(_) => Some(DataType::Float),
            Self::String(_) => Some(DataType::String),
        }
    }
}
impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self{
            Self::Null => write!(fmt, "Null"),
            Self::Boolean(b) if *b=> write!(fmt, "{}", "TRUE"),
            Self::Boolean(_) => write!(fmt, "{}", "FALSE"),
            Self::Integer(i) => write!(fmt, "{}", i),
            Self::Float(f) => write!(fmt, "{}", f),
            Self::String(s) => write!(fmt, "{}", s),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self,other) {
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Null, _) => Some(Ordering::Less),
            (_, Self::Null) => Some(Ordering::Greater),
            (Self::Boolean(b1), Self::Boolean(b2)) => b1.partial_cmp(b2),
            (Self::Integer(i1), Self::Integer(i2)) => i1.partial_cmp(i2),
            (Self::Integer(i1), Self::Float(f2)) => (*i1 as f64).partial_cmp(f2),
            (Self::Float(f1), Self::Float(f2)) => f1.partial_cmp(f2),
            (Self::Float(f2), Self::Integer(i3)) => f2.partial_cmp(&(*i3 as f64)),
            (Self::String(s1), Self::String(s2)) => s1.partial_cmp(s2),
            (_,_) => None,
        }
    }
}

pub type Row = Vec<Value>;
