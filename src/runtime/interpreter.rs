// 解释器 / Interpreter
// 执行Aevolang代码的解释器
// Interpreter for executing Aevolang code

use crate::grammar::core::{GrammarElement, Expr, Literal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 解释器 / Interpreter
pub struct Interpreter {
    /// 环境 / Environment (变量存储 / Variable storage)
    environment: HashMap<String, Value>,
}

impl Interpreter {
    /// 创建新解释器 / Create new interpreter
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    /// 执行代码 / Execute code
    pub fn execute(&mut self, ast: &[GrammarElement]) -> Result<Value, InterpreterError> {
        // TODO: 实现执行逻辑 / Implement execution logic
        Ok(Value::Null)
    }

    /// 评估表达式 / Evaluate expression
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Literal(lit) => Ok(self.eval_literal(lit)),
            Expr::Var(name) => {
                self.environment.get(name)
                    .cloned()
                    .ok_or(InterpreterError::UndefinedVariable(name.clone()))
            }
            Expr::Call(name, args) => {
                // TODO: 实现函数调用 / Implement function call
                Err(InterpreterError::NotImplemented)
            }
            Expr::Binary(op, left, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binary_op(*op, &left_val, &right_val)
            }
            Expr::If(cond, then_expr, else_expr) => {
                let cond_val = self.eval_expr(cond)?;
                if self.is_truthy(&cond_val) {
                    self.eval_expr(then_expr)
                } else {
                    self.eval_expr(else_expr)
                }
            }
        }
    }

    /// 评估字面量 / Evaluate literal
    fn eval_literal(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Int(i) => Value::Int(*i),
            Literal::Float(f) => Value::Float(*f),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Null => Value::Null,
        }
    }

    /// 评估二元运算 / Evaluate binary operation
    fn eval_binary_op(&self, op: crate::grammar::core::BinOp, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        use crate::grammar::core::BinOp::*;
        match op {
            Add => self.add_values(left, right),
            Sub => self.sub_values(left, right),
            Mul => self.mul_values(left, right),
            Div => self.div_values(left, right),
            Eq => Ok(Value::Bool(left == right)),
            Ne => Ok(Value::Bool(left != right)),
            Lt | Le | Gt | Ge => {
                // TODO: 实现比较运算 / Implement comparison operations
                Err(InterpreterError::NotImplemented)
            }
        }
    }

    /// 加法运算 / Add values
    fn add_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(InterpreterError::TypeError("Invalid types for addition".to_string())),
        }
    }

    /// 减法运算 / Subtract values
    fn sub_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err(InterpreterError::TypeError("Invalid types for subtraction".to_string())),
        }
    }

    /// 乘法运算 / Multiply values
    fn mul_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err(InterpreterError::TypeError("Invalid types for multiplication".to_string())),
        }
    }

    /// 除法运算 / Divide values
    fn div_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(InterpreterError::DivisionByZero)
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(InterpreterError::DivisionByZero)
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            _ => Err(InterpreterError::TypeError("Invalid types for division".to_string())),
        }
    }

    /// 判断真值 / Check truthiness
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// 值类型 / Value type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// 整数 / Integer
    Int(i64),
    /// 浮点数 / Float
    Float(f64),
    /// 字符串 / String
    String(String),
    /// 布尔值 / Boolean
    Bool(bool),
    /// 空值 / Null
    Null,
}

/// 解释器错误 / Interpreter error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpreterError {
    /// 未实现 / Not implemented
    NotImplemented,
    /// 未定义变量 / Undefined variable
    UndefinedVariable(String),
    /// 类型错误 / Type error
    TypeError(String),
    /// 除以零 / Division by zero
    DivisionByZero,
    /// 运行时错误 / Runtime error
    RuntimeError(String),
}

