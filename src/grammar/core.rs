// 核心语法定义 / Core grammar definitions
// 定义语言的最小核心语法元素
// Defines the minimal core grammar elements of the language

use serde::{Deserialize, Serialize};

/// 语法元素类型 / Grammar element type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GrammarElement {
    /// 原子 / Atom (symbol, number, string)
    Atom(String),
    /// 列表 / List
    List(Vec<GrammarElement>),
    /// 自然语言片段 / Natural language fragment
    NaturalLang(String),
    /// 表达式 / Expression
    Expr(Box<Expr>),
}

/// 表达式类型 / Expression type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// 字面量 / Literal value
    Literal(Literal),
    /// 变量引用 / Variable reference
    Var(String),
    /// 函数调用 / Function call
    Call(String, Vec<Expr>),
    /// 二元运算 / Binary operation
    Binary(BinOp, Box<Expr>, Box<Expr>),
    /// 条件表达式 / Conditional expression
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    /// 模式匹配 / Pattern matching
    Match(Box<Expr>, Vec<(Pattern, Expr)>),
}

/// 字面量类型 / Literal type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
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
    /// 列表 / List
    List(Vec<Expr>),
    /// 字典 / Dictionary
    Dict(Vec<(String, Expr)>),
}

/// 二元运算符 / Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    /// 加法 / Add
    Add,
    /// 减法 / Subtract
    Sub,
    /// 乘法 / Multiply
    Mul,
    /// 除法 / Divide
    Div,
    /// 等于 / Equal
    Eq,
    /// 不等于 / Not equal
    Ne,
    /// 小于 / Less than
    Lt,
    /// 大于 / Greater than
    Gt,
    /// 小于等于 / Less than or equal
    Le,
    /// 大于等于 / Greater than or equal
    Ge,
}

/// 模式 / Pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    /// 字面量模式 / Literal pattern
    Literal(Literal),
    /// 变量绑定模式 / Variable binding pattern
    Var(String),
    /// 通配符模式 / Wildcard pattern
    Wildcard,
    /// 列表模式 / List pattern
    List(Vec<Pattern>),
    /// 字典模式 / Dictionary pattern
    Dict(Vec<(String, Pattern)>),
}

/// 核心语法常量 / Core grammar constants
pub mod constants {
    /// 基础语法关键字 / Basic grammar keywords
    pub const DEF: &str = "def";
    pub const LET: &str = "let";
    pub const IF: &str = "if";
    pub const THEN: &str = "then";
    pub const ELSE: &str = "else";
    pub const FUNCTION: &str = "function";
    pub const RETURN: &str = "return";
}
