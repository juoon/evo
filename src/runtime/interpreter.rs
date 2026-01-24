// 解释器 / Interpreter
// 执行Evo-lang代码的解释器
// Interpreter for executing Evo-lang code

use crate::grammar::core::{BinOp, Expr, GrammarElement, Literal, Pattern};
use crate::parser::AdaptiveParser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 解释器 / Interpreter
pub struct Interpreter {
    /// 环境 / Environment (变量存储 / Variable storage)
    environment: HashMap<String, Value>,
    /// 函数定义 / Function definitions
    functions: HashMap<String, Function>,
    /// 模块缓存 / Module cache
    modules: HashMap<String, Module>,
    /// Lambda注册表 / Lambda registry (用于存储Lambda函数体和捕获的环境)
    lambda_registry: HashMap<String, (Vec<String>, GrammarElement, HashMap<String, Value>)>,
    /// Lambda计数器 / Lambda counter (用于生成唯一ID)
    lambda_counter: u64,
    /// 当前执行的函数所属的模块名（用于递归调用时查找模块内函数）
    /// Current executing function's module name (for finding functions in module during recursive calls)
    current_module: Option<String>,
}

/// 函数定义 / Function definition
#[derive(Debug, Clone)]
struct Function {
    /// 参数名列表 / Parameter names
    params: Vec<String>,
    /// 函数体 / Function body
    body: GrammarElement,
    /// 捕获的环境 / Captured environment (for closures)
    captured_env: Option<std::collections::HashMap<String, Value>>,
    /// 所属模块名 / Module name (None for functions defined in main scope)
    module_name: Option<String>,
}

/// 模块 / Module
#[derive(Debug, Clone)]
struct Module {
    /// 模块名称 / Module name
    name: String,
    /// 模块变量 / Module environment
    environment: HashMap<String, Value>,
    /// 模块函数 / Module functions
    functions: HashMap<String, Function>,
}

impl Interpreter {
    /// 创建新解释器 / Create new interpreter
    pub fn new() -> Self {
        let mut interpreter = Self {
            environment: HashMap::new(),
            functions: HashMap::new(),
            modules: HashMap::new(),
            lambda_registry: HashMap::new(),
            lambda_counter: 0,
            current_module: None,
        };
        // 注册内置函数 / Register built-in functions
        interpreter.register_builtins();
        interpreter
    }

    /// 注册内置函数 / Register built-in functions
    fn register_builtins(&mut self) {
        // 内置函数会在函数调用时处理
    }

    /// 执行代码 / Execute code
    pub fn execute(&mut self, ast: &[GrammarElement]) -> Result<Value, InterpreterError> {
        let mut last_value = Value::Null;

        for element in ast {
            last_value = self.eval_element(element)?;
        }

        Ok(last_value)
    }

    /// 执行单个表达式 / Execute single expression
    pub fn execute_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        self.eval_expr(expr)
    }

    /// 评估语法元素 / Evaluate grammar element
    fn eval_element(&mut self, element: &GrammarElement) -> Result<Value, InterpreterError> {
        match element {
            GrammarElement::Expr(expr) => self.eval_expr(expr),
            GrammarElement::List(list) => self.eval_list(list),
            GrammarElement::Atom(atom) => {
                // 检查是否是关键字或变量
                match atom.as_str() {
                    "def" | "function" | "let" | "if" => Err(InterpreterError::runtime_error(
                        "Special forms must be in a list".to_string(),
                        None,
                    )),
                    _ => {
                        // 尝试作为变量查找
                        self.environment
                            .get(atom)
                            .cloned()
                            .ok_or_else(|| InterpreterError::undefined_variable(atom.clone(), None))
                    }
                }
            }
            GrammarElement::NaturalLang(_) => Err(InterpreterError::runtime_error(
                "Natural language not supported in execution".to_string(),
                None,
            )),
        }
    }

    /// 评估表达式列表（用于处理多个表达式） / Evaluate expression list
    fn eval_expr_list(&mut self, exprs: &[Expr]) -> Result<Value, InterpreterError> {
        let mut last_value = Value::Null;
        for expr in exprs {
            last_value = self.eval_expr(expr)?;
        }
        Ok(last_value)
    }

    /// 评估列表 / Evaluate list
    fn eval_list(&mut self, list: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if list.is_empty() {
            return Ok(Value::Null);
        }

        // 检查是否是特殊形式（支持 Atom 和 Expr(Var(...)) 两种形式）
        let keyword: Option<&str> = match &list[0] {
            GrammarElement::Atom(s) => Some(s.as_str()),
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    Some(s.as_str())
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(keyword) = keyword {
            match keyword {
                "def" | "function" => self.eval_def(&list[1..]),
                "let" => self.eval_let(&list[1..]),
                "set!" => self.eval_set(&list[1..]),
                "if" => self.eval_if_special(&list[1..]),
                "lambda" => self.eval_lambda(&list[1..]),
                "for" => self.eval_for_special(&list[1..]),
                "while" => self.eval_while_special(&list[1..]),
                "match" => {
                    // 在 eval_list 中处理 match 表达式（作为解析失败的兜底）
                    // 正常情况下 match 应该在解析阶段被转换为 Expr::Match
                    // 但由于某种原因解析失败了，我们在这里兜底处理
                    return Err(InterpreterError::runtime_error(
                        "Match expression parsing failed: should be handled at parse time"
                            .to_string(),
                        None,
                    ));
                }
                "list" | "vec" => {
                    // 列表字面量：解析为 Literal::List
                    let mut items = Vec::new();
                    for elem in &list[1..] {
                        items.push(self.eval_element(elem)?);
                    }
                    Ok(Value::List(items))
                }
                "dict" => {
                    // 字典字面量：解析为 Literal::Dict
                    if list.len() % 2 != 1 {
                        return Err(InterpreterError::runtime_error(
                            "Dictionary literal requires even number of key-value pairs"
                                .to_string(),
                            None,
                        ));
                    }
                    let mut dict = std::collections::HashMap::new();
                    for i in (1..list.len()).step_by(2) {
                        let key_elem = &list[i];
                        let value_elem = &list[i + 1];
                        let key = match key_elem {
                            GrammarElement::Atom(s) => s.clone(),
                            GrammarElement::Expr(boxed_expr) => {
                                if let Expr::Var(s) = boxed_expr.as_ref() {
                                    s.clone()
                                } else if let Expr::Literal(Literal::String(s)) =
                                    boxed_expr.as_ref()
                                {
                                    s.clone()
                                } else {
                                    return Err(InterpreterError::runtime_error(
                                        "Dictionary key must be a string or atom".to_string(),
                                        None,
                                    ));
                                }
                            }
                            _ => {
                                return Err(InterpreterError::runtime_error(
                                    "Dictionary key must be a string or atom".to_string(),
                                    None,
                                ));
                            }
                        };
                        let value = self.eval_element(value_elem)?;
                        dict.insert(key, value);
                    }
                    Ok(Value::Dict(dict))
                }
                _ => {
                    // 尝试作为函数调用
                    // 注意：参数中包含 list/dict 字面量时，需要先评估它们
                    // Note: When arguments contain list/dict literals, they need to be evaluated first
                    let func_name = keyword.to_string();

                    // 检查函数名是否是环境中的 Lambda 值（当函数名是变量时）
                    // Check if function name is a Lambda value in environment (when function name is a variable)
                    // 这包括函数参数中的 lambda（如 map 函数的 func 参数）
                    // This includes lambdas in function parameters (like the func parameter in map function)
                    if let Some(Value::Lambda { id, params }) =
                        self.environment.get(&func_name).cloned()
                    {
                        // 函数名是 Lambda 值，需要先评估参数，然后调用 Lambda
                        // Function name is Lambda value, need to evaluate arguments first, then call Lambda
                        let mut arg_values = Vec::new();
                        for elem in &list[1..] {
                            arg_values.push(self.eval_element(elem)?);
                        }
                        // 将 Value 转换为 Expr
                        // Convert Value to Expr
                        // 注意：Lambda 值无法转换为 Expr，需要特殊处理
                        // Note: Lambda values cannot be converted to Expr, need special handling
                        let mut arg_exprs = Vec::new();
                        for val in arg_values {
                            if let Value::Lambda { .. } = val {
                                // Lambda 值需要存储到环境中
                                // Lambda values need to be stored in environment
                                let temp_name = format!("__lambda_arg_{}", arg_exprs.len());
                                self.environment.insert(temp_name.clone(), val);
                                arg_exprs.push(Expr::Var(temp_name));
                            } else {
                                arg_exprs.push(self.value_to_expr(val)?);
                            }
                        }
                        return self.call_lambda(&id, &params, &arg_exprs);
                    }

                    // 检查是否需要先评估参数（包含 list/dict 字面量时）
                    // Check if we need to evaluate arguments first (when they contain list/dict literals)
                    // 注意：如果函数名是 lambda，不需要检查字面量，因为 lambda 调用会直接处理参数
                    // Note: If function name is lambda, don't check for literals, as lambda call will handle arguments directly
                    let needs_evaluation = if self
                        .environment
                        .get(&func_name)
                        .map(|v| matches!(v, Value::Lambda { .. }))
                        .unwrap_or(false)
                    {
                        // 函数名是 lambda，不需要检查字面量
                        // Function name is lambda, don't check for literals
                        false
                    } else {
                        list[1..].iter().any(|e| {
                            if let GrammarElement::List(l) = e {
                                if !l.is_empty() {
                                    if let GrammarElement::Atom(s) = &l[0] {
                                        return s == "list"
                                            || s == "vec"
                                            || s == "dict"
                                            || s == "map";
                                    }
                                }
                            }
                            false
                        })
                    };

                    let args = if needs_evaluation {
                        // 包含字面量，需要先评估参数
                        // Contains literals, need to evaluate arguments first
                        // 但是，如果参数是变量且环境中是 Lambda 值，需要特殊处理
                        // However, if argument is a variable and environment has Lambda value, need special handling
                        let mut arg_exprs = Vec::new();
                        for elem in &list[1..] {
                            // 先检查是否是变量，如果是变量且环境中是 Lambda 值，则直接使用变量名
                            // First check if it's a variable, if it's a variable and environment has Lambda value, use variable name directly
                            let is_lambda_var = if let GrammarElement::Atom(var_name) = elem {
                                self.environment
                                    .get(var_name)
                                    .map(|v| matches!(v, Value::Lambda { .. }))
                                    .unwrap_or(false)
                            } else {
                                false
                            };

                            if is_lambda_var {
                                // 变量是 Lambda 值，直接使用变量名（在 eval_call 中会从环境中获取）
                                // Variable is Lambda value, use variable name directly (will be retrieved from environment in eval_call)
                                if let GrammarElement::Atom(var_name) = elem {
                                    arg_exprs.push(Expr::Var(var_name.clone()));
                                } else {
                                    // Should not happen, but handle it
                                    let value = self.eval_element(elem)?;
                                    if let Value::Lambda { .. } = value {
                                        let temp_name =
                                            format!("__lambda_temp_{}", arg_exprs.len());
                                        self.environment.insert(temp_name.clone(), value);
                                        arg_exprs.push(Expr::Var(temp_name));
                                    } else {
                                        arg_exprs.push(self.value_to_expr(value)?);
                                    }
                                }
                            } else {
                                // 先评估参数，然后根据值的类型处理
                                // Evaluate argument first, then handle based on value type
                                let value = self.eval_element(elem)?;
                                if let Value::Lambda { .. } = value {
                                    // Lambda 值需要存储到环境中
                                    // Lambda values need to be stored in environment
                                    let temp_name = format!("__lambda_temp_{}", arg_exprs.len());
                                    self.environment.insert(temp_name.clone(), value);
                                    arg_exprs.push(Expr::Var(temp_name));
                                } else {
                                    // 对于非 Lambda 值，直接转换为 Expr
                                    // For non-Lambda values, directly convert to Expr
                                    arg_exprs.push(self.value_to_expr(value)?);
                                }
                            }
                        }
                        arg_exprs
                    } else {
                        // 没有字面量，尝试直接转换
                        // No literals, try direct conversion
                        // 检查是否包含 lambda 表达式
                        // Check if arguments contain lambda expressions
                        let has_lambda = list[1..].iter().any(|e| {
                            if let GrammarElement::List(l) = e {
                                !l.is_empty()
                                    && match &l[0] {
                                        GrammarElement::Atom(s) => s == "lambda",
                                        GrammarElement::Expr(boxed_expr) => {
                                            if let Expr::Var(s) = boxed_expr.as_ref() {
                                                s == "lambda"
                                            } else {
                                                false
                                            }
                                        }
                                        _ => false,
                                    }
                            } else {
                                false
                            }
                        });

                        if has_lambda {
                            // 包含 lambda 表达式，需要先评估所有参数，然后直接调用函数
                            // Contains lambda expressions, need to evaluate all arguments first, then call function directly
                            let mut arg_values = Vec::new();
                            for elem in &list[1..] {
                                arg_values.push(self.eval_element(elem)?);
                            }
                            // 直接使用 Value 调用函数（需要修改 eval_call 或创建新函数）
                            // 暂时，我们将 Value::Lambda 存储到环境中，然后传递引用
                            // For now, we store Value::Lambda in environment and pass reference
                            let mut arg_exprs = Vec::new();
                            for (idx, val) in arg_values.iter().enumerate() {
                                if let Value::Lambda { .. } = val {
                                    // Lambda 值需要存储到环境中
                                    // Lambda values need to be stored in environment
                                    let temp_name = format!("__lambda_temp_{}", idx);
                                    self.environment.insert(temp_name.clone(), val.clone());
                                    arg_exprs.push(Expr::Var(temp_name));
                                } else {
                                    // 其他值转换为 Expr
                                    // Other values convert to Expr
                                    arg_exprs.push(self.value_to_expr(val.clone())?);
                                }
                            }
                            arg_exprs
                        } else {
                            // 没有 lambda 表达式，尝试直接转换
                            // No lambda expressions, try direct conversion
                            // 但是，如果参数是变量且环境中是 Lambda 值，需要先评估
                            // However, if argument is a variable and environment has Lambda value, need to evaluate first
                            let mut converted_args = Vec::new();
                            for elem in &list[1..] {
                                // 先检查是否是变量，如果是变量且环境中是 Lambda 值，则直接使用变量名
                                // First check if it's a variable, if it's a variable and environment has Lambda value, use variable name directly
                                let is_lambda_var = if let GrammarElement::Atom(var_name) = elem {
                                    self.environment
                                        .get(var_name)
                                        .map(|v| matches!(v, Value::Lambda { .. }))
                                        .unwrap_or(false)
                                } else {
                                    false
                                };

                                if is_lambda_var {
                                    // 变量是 Lambda 值，直接使用变量名（在 eval_call 中会从环境中获取）
                                    // Variable is Lambda value, use variable name directly (will be retrieved from environment in eval_call)
                                    if let GrammarElement::Atom(var_name) = elem {
                                        converted_args.push(Expr::Var(var_name.clone()));
                                    } else {
                                        // Should not happen, but handle it
                                        let value = self.eval_element(elem)?;
                                        if let Value::Lambda { .. } = value {
                                            let temp_name =
                                                format!("__lambda_temp_{}", converted_args.len());
                                            self.environment.insert(temp_name.clone(), value);
                                            converted_args.push(Expr::Var(temp_name));
                                        } else {
                                            converted_args
                                                .extend(self.values_to_exprs(vec![value])?);
                                        }
                                    }
                                } else {
                                    // 先评估参数，然后根据值的类型处理
                                    // Evaluate argument first, then handle based on value type
                                    let value = self.eval_element(elem)?;
                                    if let Value::Lambda { .. } = value {
                                        // Lambda 值需要存储到环境中
                                        // Lambda values need to be stored in environment
                                        let temp_name =
                                            format!("__lambda_temp_{}", converted_args.len());
                                        self.environment.insert(temp_name.clone(), value);
                                        converted_args.push(Expr::Var(temp_name));
                                    } else {
                                        // 对于非 Lambda 值，直接转换为 Expr
                                        // For non-Lambda values, directly convert to Expr
                                        converted_args.extend(self.values_to_exprs(vec![value])?);
                                    }
                                }
                            }
                            converted_args
                        }
                    };
                    self.eval_call(&func_name, &args)
                }
            }
        } else {
            // 普通列表，返回最后一个元素的值
            let mut last_value = Value::Null;
            for element in list {
                last_value = self.eval_element(element)?;
            }
            Ok(last_value)
        }
    }

    /// 将语法元素转换为表达式 / Convert grammar element to expression
    /// 注意：这个函数不能完全处理所有情况，主要用于简单的参数转换
    /// Note: This function cannot fully handle all cases, mainly used for simple argument conversion
    fn element_to_expr(&self, element: &GrammarElement) -> Result<Expr, ()> {
        match element {
            GrammarElement::Expr(e) => Ok(*e.clone()),
            GrammarElement::Atom(s) => Ok(Expr::Var(s.clone())),
            GrammarElement::List(l) => {
                // 对于 List，我们需要先评估它，然后转换为字面量
                // 但这需要 mut self，所以这里我们只处理简单情况
                // 对于复杂情况（如 (list ...)），应该在解析阶段就转换为 Expr
                // For List, we should evaluate it first and then convert to literal
                // But this requires mut self, so we only handle simple cases here
                // Complex cases (like (list ...)) should be converted to Expr at parsing stage
                if l.is_empty() {
                    Ok(Expr::Literal(Literal::Null))
                } else {
                    // 如果是 list/vec 字面量，尝试转换
                    if let GrammarElement::Atom(s) = &l[0] {
                        if s == "list" || s == "vec" {
                            // 这是一个列表字面量，但由于需要递归调用 element_to_expr，
                            // 而这个函数不能处理嵌套列表，我们返回错误，让调用者处理
                            // This is a list literal, but since we need recursive calls to element_to_expr,
                            // and this function can't handle nested lists, we return an error
                            return Err(());
                        }
                    }
                    // 其他情况（函数调用等）也返回错误，应该已经在解析阶段处理
                    // Other cases (function calls, etc.) also return error, should have been handled at parsing stage
                    Err(())
                }
            }
            GrammarElement::NaturalLang(_) => Err(()),
        }
    }

    /// 将值列表转换为表达式列表 / Convert value list to expression list
    fn values_to_exprs(&self, values: Vec<Value>) -> Result<Vec<Expr>, InterpreterError> {
        let mut exprs = Vec::new();
        for value in values {
            exprs.push(self.value_to_expr(value)?);
        }
        Ok(exprs)
    }

    /// 将表达式转换为语法元素 / Convert expression to grammar element
    fn expr_to_element(&self, expr: &Expr) -> Result<GrammarElement, InterpreterError> {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(i) => Ok(GrammarElement::Expr(Box::new(Expr::Literal(Literal::Int(
                    *i,
                ))))),
                Literal::Float(f) => Ok(GrammarElement::Expr(Box::new(Expr::Literal(
                    Literal::Float(*f),
                )))),
                Literal::String(s) => Ok(GrammarElement::Expr(Box::new(Expr::Literal(
                    Literal::String(s.clone()),
                )))),
                Literal::Bool(b) => Ok(GrammarElement::Expr(Box::new(Expr::Literal(
                    Literal::Bool(*b),
                )))),
                Literal::Null => Ok(GrammarElement::Expr(Box::new(Expr::Literal(Literal::Null)))),
                Literal::List(items) => {
                    let mut elements = Vec::new();
                    for item in items {
                        elements.push(self.expr_to_element(item)?);
                    }
                    Ok(GrammarElement::List(elements))
                }
                Literal::Dict(pairs) => {
                    let mut elements = Vec::new();
                    for (key, val) in pairs {
                        elements.push(GrammarElement::Atom(key.clone()));
                        elements.push(self.expr_to_element(val)?);
                    }
                    Ok(GrammarElement::List(elements))
                }
            },
            Expr::Var(name) => Ok(GrammarElement::Atom(name.clone())),
            Expr::Call(name, args) => {
                let mut elements = vec![GrammarElement::Atom(name.clone())];
                for arg in args {
                    elements.push(self.expr_to_element(arg)?);
                }
                Ok(GrammarElement::List(elements))
            }
            Expr::Binary(op, left, right) => {
                let op_name = match op {
                    BinOp::Add => "op:+",
                    BinOp::Sub => "op:-",
                    BinOp::Mul => "op:*",
                    BinOp::Div => "op:/",
                    BinOp::Mod => "op:%",
                    BinOp::Eq => "op:=",
                    BinOp::Ne => "op:!=",
                    BinOp::Lt => "op:<",
                    BinOp::Le => "op:<=",
                    BinOp::Gt => "op:>",
                    BinOp::Ge => "op:>=",
                };
                Ok(GrammarElement::List(vec![
                    GrammarElement::Atom(op_name.to_string()),
                    self.expr_to_element(left)?,
                    self.expr_to_element(right)?,
                ]))
            }
            Expr::If(cond, then_expr, else_expr) => Ok(GrammarElement::List(vec![
                GrammarElement::Atom("if".to_string()),
                self.expr_to_element(cond)?,
                self.expr_to_element(then_expr)?,
                self.expr_to_element(else_expr)?,
            ])),
            Expr::Match(_, _) => Err(InterpreterError::runtime_error(
                "Match expressions cannot be converted back to GrammarElement".to_string(),
                None,
            )),
            Expr::For { .. } => Err(InterpreterError::runtime_error(
                "For expressions cannot be converted back to GrammarElement".to_string(),
                None,
            )),
            Expr::While { .. } => Err(InterpreterError::runtime_error(
                "While expressions cannot be converted back to GrammarElement".to_string(),
                None,
            )),
            Expr::Try { .. } => Err(InterpreterError::runtime_error(
                "Try expressions cannot be converted back to GrammarElement".to_string(),
                None,
            )),
            Expr::Lambda { params, body } => {
                // 将lambda转换为列表形式，以便在需要时重新解析
                let mut elements = vec![GrammarElement::Atom("lambda".to_string())];

                // 参数列表
                let mut param_elements = Vec::new();
                for param in params {
                    param_elements.push(GrammarElement::Atom(param.clone()));
                }
                elements.push(GrammarElement::List(param_elements));

                // 函数体
                elements.push(self.expr_to_element(body)?);

                Ok(GrammarElement::List(elements))
            }
            Expr::Begin(exprs) => {
                let mut elements = vec![GrammarElement::Atom("begin".to_string())];
                for expr in exprs {
                    elements.push(self.expr_to_element(expr)?);
                }
                Ok(GrammarElement::List(elements))
            }
            Expr::Assign(var, expr) => Ok(GrammarElement::List(vec![
                GrammarElement::Atom("set!".to_string()),
                GrammarElement::Atom(var.clone()),
                self.expr_to_element(expr)?,
            ])),
        }
    }

    /// 将值转换为表达式 / Convert value to expression
    fn value_to_expr(&self, value: Value) -> Result<Expr, InterpreterError> {
        match value {
            Value::Int(i) => Ok(Expr::Literal(Literal::Int(i))),
            Value::Float(f) => Ok(Expr::Literal(Literal::Float(f))),
            Value::String(s) => Ok(Expr::Literal(Literal::String(s))),
            Value::Bool(b) => Ok(Expr::Literal(Literal::Bool(b))),
            Value::Null => Ok(Expr::Literal(Literal::Null)),
            Value::List(items) => {
                // 递归转换列表中的每个元素
                // Recursively convert each element in the list
                let mut expr_items = Vec::new();
                for item in items {
                    expr_items.push(self.value_to_expr(item)?);
                }
                Ok(Expr::Literal(Literal::List(expr_items)))
            }
            Value::Dict(dict) => {
                // 递归转换字典中的每个值
                // Recursively convert each value in the dict
                let mut pairs = Vec::new();
                for (key, val) in dict {
                    pairs.push((key, self.value_to_expr(val)?));
                }
                Ok(Expr::Literal(Literal::Dict(pairs)))
            }
            Value::Lambda { .. } => {
                // Lambda 值无法转换为 Expr，这是一个限制
                // Lambda values cannot be converted to Expr, this is a limitation
                Err(InterpreterError::runtime_error(
                    "Lambda values cannot be converted to expressions in function arguments"
                        .to_string(),
                    None,
                ))
            }
        }
    }

    /// 评估函数定义 / Evaluate function definition
    fn eval_def(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.len() < 3 {
            return Err(InterpreterError::runtime_error(
                "Function definition requires: name, params, body".to_string(),
                None,
            ));
        }

        // 获取函数名（支持 Atom 和 Expr(Var(...)) 两种形式）
        let name = match &rest[0] {
            GrammarElement::Atom(s) => s.clone(),
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    s.clone()
                } else {
                    return Err(InterpreterError::runtime_error(
                        "Function name must be an atom or variable".to_string(),
                        None,
                    ));
                }
            }
            _ => {
                return Err(InterpreterError::runtime_error(
                    "Function name must be an atom or variable".to_string(),
                    None,
                ))
            }
        };

        // 获取参数列表（支持 Atom 和 Expr(Var(...)) 两种形式）
        let params = match &rest[1] {
            GrammarElement::List(args_list) => args_list
                .iter()
                .map(|e| match e {
                    GrammarElement::Atom(s) => Ok(s.clone()),
                    GrammarElement::Expr(boxed_expr) => {
                        if let Expr::Var(s) = boxed_expr.as_ref() {
                            Ok(s.clone())
                        } else {
                            Err(InterpreterError::runtime_error(
                                format!(
                                    "Parameter must be an atom or variable, got: {:?}",
                                    boxed_expr
                                ),
                                None,
                            ))
                        }
                    }
                    _ => Err(InterpreterError::runtime_error(
                        format!("Parameter must be an atom or variable, got: {:?}", e),
                        None,
                    )),
                })
                .collect::<Result<Vec<_>, _>>()?,
            _ => {
                return Err(InterpreterError::runtime_error(
                    format!("Parameters must be a list, got: {:?}", &rest[1]),
                    None,
                ))
            }
        };

        // 获取函数体
        let body = rest[2].clone();

        // 注册函数
        self.functions.insert(
            name.clone(),
            Function {
                params,
                body,
                captured_env: None,
                module_name: None, // 主作用域的函数没有模块名
            },
        );

        Ok(Value::Null)
    }

    /// 评估let绑定 / Evaluate let binding
    fn eval_let(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.len() < 2 {
            return Err(InterpreterError::runtime_error(
                "Let requires at least: name, value".to_string(),
                None,
            ));
        }

        // 获取变量名（支持 Atom 和 Expr(Var(...)) 两种形式）
        let name = match &rest[0] {
            GrammarElement::Atom(s) => s.clone(),
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    s.clone()
                } else {
                    return Err(InterpreterError::runtime_error(
                        "Variable name must be an atom or variable".to_string(),
                        None,
                    ));
                }
            }
            _ => {
                return Err(InterpreterError::runtime_error(
                    "Variable name must be an atom or variable".to_string(),
                    None,
                ))
            }
        };

        // 评估值
        let value = self.eval_element(&rest[1])?;

        // 检查是否有body（body是可选的）
        let has_body = rest.len() > 2
            && !matches!(&rest[2], GrammarElement::Expr(boxed_expr) if matches!(boxed_expr.as_ref(), Expr::Literal(Literal::Null)));

        // 保存旧值（用于作用域）
        let old_value = self.environment.insert(name.clone(), value);

        // 如果有body，评估body并在评估后恢复旧值（变量只在body的作用域中可用）
        // 如果没有body，变量应该保持在作用域中（用于顶层绑定）
        let result = if has_body {
            // 评估函数体（支持多个表达式，返回最后一个表达式的值）
            let mut body_result = Value::Null;
            for body_elem in &rest[2..] {
                body_result = self.eval_element(body_elem)?;
            }

            // 恢复旧值（如果存在）
            if let Some(old) = old_value {
                self.environment.insert(name, old);
            } else {
                self.environment.remove(&name);
            }

            body_result
        } else {
            // 没有body，变量保持在作用域中，返回null
            Value::Null
        };

        Ok(result)
    }

    /// 评估set!赋值操作 / Evaluate set! assignment operation
    fn eval_set(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.len() < 2 {
            return Err(InterpreterError::runtime_error(
                "set! requires: name, value".to_string(),
                None,
            ));
        }

        // 获取变量名（支持 Atom 和 Expr(Var(...)) 两种形式）
        let name = match &rest[0] {
            GrammarElement::Atom(s) => s.clone(),
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    s.clone()
                } else {
                    return Err(InterpreterError::runtime_error(
                        "Variable name must be an atom or variable".to_string(),
                        None,
                    ));
                }
            }
            _ => {
                return Err(InterpreterError::runtime_error(
                    "Variable name must be an atom or variable".to_string(),
                    None,
                ))
            }
        };

        // 评估值
        let value = self.eval_element(&rest[1])?;

        // 检查变量是否存在于环境中（set! 只能修改已存在的变量）
        if !self.environment.contains_key(&name) {
            return Err(InterpreterError::runtime_error(
                format!(
                    "Variable '{}' is not defined. Use 'let' to define a new variable.",
                    name
                ),
                None,
            ));
        }

        // 更新变量值（不恢复旧值，这是赋值操作）
        self.environment.insert(name.clone(), value.clone());

        Ok(value)
    }

    /// 评估if特殊形式 / Evaluate if special form
    fn eval_if_special(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.is_empty() {
            return Err(InterpreterError::runtime_error(
                "If requires at least a condition".to_string(),
                None,
            ));
        }

        let condition = self.eval_element(&rest[0])?;

        if self.is_truthy(&condition) {
            if rest.len() > 1 {
                self.eval_element(&rest[1])
            } else {
                Ok(Value::Null)
            }
        } else {
            if rest.len() > 2 {
                self.eval_element(&rest[2])
            } else {
                Ok(Value::Null)
            }
        }
    }

    /// 评估for循环特殊形式（从GrammarElement） / Evaluate for loop special form (from GrammarElement)
    fn eval_for_special(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.len() < 3 {
            return Err(InterpreterError::runtime_error(
                "For requires: var, iterable, body".to_string(),
                None,
            ));
        }

        // 获取变量名
        let var = match &rest[0] {
            GrammarElement::Atom(s) => s.clone(),
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    s.clone()
                } else {
                    return Err(InterpreterError::runtime_error(
                        "For loop variable must be an identifier".to_string(),
                        None,
                    ));
                }
            }
            _ => {
                return Err(InterpreterError::runtime_error(
                    "For loop variable must be an identifier".to_string(),
                    None,
                ));
            }
        };

        // 评估迭代对象
        let iterable_value = self.eval_element(&rest[1])?;

        // 获取循环体（剩余的元素）
        let body_elem = if rest.len() == 3 {
            rest[2].clone()
        } else {
            GrammarElement::List(rest[2..].to_vec())
        };

        // 将循环体转换为Expr（如果可能），否则直接评估
        // 对于for循环，循环体需要能够访问循环变量，所以我们需要特殊处理
        match self.element_to_expr(&body_elem) {
            Ok(body_expr) => {
                // 如果能够转换为Expr，使用eval_for
                self.eval_for(&var, &iterable_value, &body_expr)
            }
            Err(_) => {
                // 如果不能转换为Expr，直接评估GrammarElement
                // 这种情况下，循环体中的变量需要在环境中查找
                let items = match iterable_value {
                    Value::List(list) => list.clone(),
                    Value::Int(end) => (0..end as usize).map(|i| Value::Int(i as i64)).collect(),
                    _ => {
                        return Err(InterpreterError::type_error(
                            "For loop iterable must be a list or integer".to_string(),
                            None,
                        ));
                    }
                };

                let mut last_value = Value::Null;
                // 保存循环变量在循环外的旧值（如果存在）
                let outer_old_value = self.environment.get(&var).cloned();

                for item in items {
                    // 设置循环变量值
                    self.environment.insert(var.clone(), item);

                    // 执行循环体
                    last_value = self.eval_element(&body_elem)?;
                }

                // 恢复循环外的旧值（如果存在）
                if let Some(old) = outer_old_value {
                    self.environment.insert(var.clone(), old);
                } else {
                    // 只有在循环前变量不存在时才删除
                    self.environment.remove(&var);
                }

                Ok(last_value)
            }
        }
    }

    /// 评估while循环特殊形式（从GrammarElement） / Evaluate while loop special form (from GrammarElement)
    fn eval_while_special(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.len() < 2 {
            return Err(InterpreterError::runtime_error(
                "While requires: condition, body".to_string(),
                None,
            ));
        }

        // 获取循环体和条件
        let condition_elem = &rest[0];
        let body_elem = if rest.len() == 2 {
            rest[1].clone()
        } else {
            GrammarElement::List(rest[1..].to_vec())
        };

        // 尝试转换为Expr，否则直接评估GrammarElement
        let condition_expr_opt = self.element_to_expr(condition_elem).ok();
        let body_expr_opt = self.element_to_expr(&body_elem).ok();

        let mut last_value = Value::Null;

        loop {
            // 评估条件
            let cond_value = if let Some(ref expr) = condition_expr_opt {
                self.eval_expr(expr)?
            } else {
                self.eval_element(condition_elem)?
            };

            // 如果条件为假，退出循环
            if !self.is_truthy(&cond_value) {
                break;
            }

            // 执行循环体
            if let Some(ref expr) = body_expr_opt {
                last_value = self.eval_expr(expr)?;
            } else {
                last_value = self.eval_element(&body_elem)?;
            }
        }

        Ok(last_value)
    }

    /// 评估Lambda表达式 / Evaluate lambda expression
    /// 语法: (lambda (params...) body)
    fn eval_lambda(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.len() < 2 {
            return Err(InterpreterError::runtime_error(
                "Lambda requires: params and body".to_string(),
                None,
            ));
        }

        // 解析参数列表（支持 Atom 和 Expr(Var(...)) 两种形式）
        let params_elem = &rest[0];
        let params = match params_elem {
            GrammarElement::List(params_list) => params_list
                .iter()
                .filter_map(|e| match e {
                    GrammarElement::Atom(s) => Some(s.clone()),
                    GrammarElement::Expr(boxed_expr) => {
                        if let Expr::Var(s) = boxed_expr.as_ref() {
                            Some(s.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect(),
            GrammarElement::Atom(single_param) => vec![single_param.clone()],
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    vec![s.clone()]
                } else {
                    return Err(InterpreterError::runtime_error(
                        "Lambda params must be a list of atoms or variables".to_string(),
                        None,
                    ));
                }
            }
            _ => {
                return Err(InterpreterError::runtime_error(
                    "Lambda params must be a list of atoms or variables".to_string(),
                    None,
                ))
            }
        };

        // 获取函数体（剩余的所有元素作为body）
        let body = if rest.len() == 2 {
            rest[1].clone()
        } else {
            // 多个body元素，创建一个列表
            GrammarElement::List(rest[1..].to_vec())
        };

        // 捕获当前环境（用于闭包）
        // 只捕获不在参数列表中的变量，避免参数遮蔽
        let captured_env: HashMap<String, Value> = self
            .environment
            .iter()
            .filter(|(key, _)| !params.contains(key))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // 生成唯一的Lambda ID
        self.lambda_counter += 1;
        let lambda_id = format!("__lambda_{}", self.lambda_counter);

        // 注册Lambda函数体和捕获的环境
        self.lambda_registry
            .insert(lambda_id.clone(), (params.clone(), body, captured_env));

        // 返回Lambda值
        Ok(Value::Lambda {
            id: lambda_id,
            params,
        })
    }

    /// 评估表达式 / Evaluate expression
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Literal(lit) => self.eval_literal(lit),
            Expr::Var(name) => {
                // 首先检查是否是操作符（当操作符作为变量传递时）
                // First check if it's an operator (when operator is passed as variable)
                if name == "+"
                    || name == "-"
                    || name == "*"
                    || name == "/"
                    || name == "%"
                    || name == "="
                    || name == "=="
                    || name == "!="
                    || name == "<>"
                    || name == "<"
                    || name == ">"
                    || name == "<="
                    || name == ">="
                {
                    // 操作符作为值传递时，返回一个特殊的字符串值
                    // When operator is passed as value, return a special string value
                    return Ok(Value::String(name.clone()));
                }
                self.environment
                    .get(name)
                    .cloned()
                    .ok_or_else(|| InterpreterError::undefined_variable(name.clone(), None))
            }
            Expr::Call(name, args) => self.eval_call(name, args),
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
                    // 检查 else_expr 是否是 let 表达式的错误转换
                    // Check if else_expr is a mis-converted let expression
                    if let Expr::Call(name, args) = else_expr.as_ref() {
                        if name == "let" {
                            // 将 Call("let", args) 转换回 GrammarElement::List
                            // Convert Call("let", args) back to GrammarElement::List
                            let mut let_elements = vec![GrammarElement::Atom("let".to_string())];
                            for arg in args {
                                let_elements.push(self.expr_to_element(arg)?);
                            }
                            return self.eval_list(&let_elements);
                        }
                    }
                    self.eval_expr(else_expr)
                }
            }
            Expr::Match(value_expr, cases) => {
                let value = self.eval_expr(value_expr)?;
                self.eval_match(&value, cases)
            }
            Expr::For {
                var,
                iterable,
                body,
            } => {
                let iterable_value = self.eval_expr(iterable)?;
                self.eval_for(var, &iterable_value, body)
            }
            Expr::While { condition, body } => self.eval_while(condition, body),
            Expr::Try {
                try_body,
                catch_var,
                catch_body,
            } => self.eval_try(try_body, catch_var, catch_body),
            Expr::Lambda { params, body } => {
                // 生成唯一的Lambda ID
                self.lambda_counter += 1;
                let lambda_id = format!("__lambda_{}", self.lambda_counter);

                // 将body转换为GrammarElement，以便在调用时评估
                let body_elem = self.expr_to_element(body)?;

                // 捕获当前环境（用于闭包）
                // 只捕获不在参数列表中的变量，避免参数遮蔽
                let captured_env: HashMap<String, Value> = self
                    .environment
                    .iter()
                    .filter(|(key, _)| !params.contains(key))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

                // 注册Lambda函数体和捕获的环境
                self.lambda_registry
                    .insert(lambda_id.clone(), (params.clone(), body_elem, captured_env));

                // 返回Lambda值
                Ok(Value::Lambda {
                    id: lambda_id,
                    params: params.clone(),
                })
            }
            Expr::Begin(exprs) => {
                // 按顺序执行多个表达式，返回最后一个表达式的结果
                let mut result = Value::Null;
                for expr in exprs {
                    result = self.eval_expr(expr)?;
                }
                Ok(result)
            }
            Expr::Assign(var, expr) => {
                // 计算赋值表达式的值
                let value = self.eval_expr(expr)?;
                // 更新环境中的变量值
                self.environment.insert(var.clone(), value.clone());
                // 返回赋值后的值
                Ok(value)
            }
        }
    }

    /// 评估模式匹配 / Evaluate pattern matching
    fn eval_match(
        &mut self,
        value: &Value,
        cases: &[(Pattern, Expr)],
    ) -> Result<Value, InterpreterError> {
        for (pattern, expr) in cases {
            if self.pattern_matches(pattern, value)? {
                // 绑定模式中的变量
                self.bind_pattern_variables(pattern, value)?;
                let result = self.eval_expr(expr)?;
                // 恢复环境（移除绑定的变量）
                self.unbind_pattern_variables(pattern);
                return Ok(result);
            }
        }
        Err(InterpreterError::runtime_error(
            "No pattern matched in match expression".to_string(),
            None,
        ))
    }

    /// 检查模式是否匹配值 / Check if pattern matches value
    fn pattern_matches(&self, pattern: &Pattern, value: &Value) -> Result<bool, InterpreterError> {
        match (pattern, value) {
            (Pattern::Wildcard, _) => Ok(true),
            (Pattern::Var(_), _) => Ok(true), // 变量模式总是匹配
            (Pattern::Literal(lit), val) => match (lit, val) {
                (Literal::Int(i), Value::Int(j)) => Ok(i == j),
                (Literal::Float(f), Value::Float(g)) => Ok((f - g).abs() < f64::EPSILON),
                (Literal::String(s), Value::String(t)) => Ok(s == t),
                (Literal::Bool(b), Value::Bool(c)) => Ok(b == c),
                (Literal::Null, Value::Null) => Ok(true),
                _ => Ok(false),
            },
            (Pattern::List(patterns), Value::List(values)) => {
                if patterns.len() != values.len() {
                    return Ok(false);
                }
                for (pat, val) in patterns.iter().zip(values.iter()) {
                    if !self.pattern_matches(pat, val)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            (Pattern::Dict(patterns), Value::Dict(values)) => {
                // 检查所有模式键是否都在值中，且匹配
                for (key, pat) in patterns {
                    if let Some(val) = values.get(key) {
                        if !self.pattern_matches(pat, val)? {
                            return Ok(false);
                        }
                    } else {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// 绑定模式中的变量到环境 / Bind pattern variables to environment
    fn bind_pattern_variables(
        &mut self,
        pattern: &Pattern,
        value: &Value,
    ) -> Result<(), InterpreterError> {
        match (pattern, value) {
            (Pattern::Var(name), val) => {
                // 优化：直接插入，不需要克隆name（已经在pattern中）
                self.environment.insert(name.clone(), val.clone());
            }
            (Pattern::List(patterns), Value::List(values)) => {
                for (pat, val) in patterns.iter().zip(values.iter()) {
                    self.bind_pattern_variables(pat, val)?;
                }
            }
            (Pattern::Dict(patterns), Value::Dict(values)) => {
                for (key, pat) in patterns {
                    if let Some(val) = values.get(key) {
                        self.bind_pattern_variables(pat, val)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// 解绑模式中的变量 / Unbind pattern variables from environment
    fn unbind_pattern_variables(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Var(name) => {
                self.environment.remove(name);
            }
            Pattern::List(patterns) => {
                for pat in patterns {
                    self.unbind_pattern_variables(pat);
                }
            }
            Pattern::Dict(patterns) => {
                for (_, pat) in patterns {
                    self.unbind_pattern_variables(pat);
                }
            }
            _ => {}
        }
    }

    /// 评估For循环 / Evaluate for loop
    fn eval_for(
        &mut self,
        var: &str,
        iterable: &Value,
        body: &Expr,
    ) -> Result<Value, InterpreterError> {
        let items = match iterable {
            Value::List(list) => list.clone(),
            Value::Int(end) => {
                // 如果iterable是整数，创建范围 [0, end)
                (0..*end as usize).map(|i| Value::Int(i as i64)).collect()
            }
            _ => {
                return Err(InterpreterError::type_error(
                    "For loop iterable must be a list or integer".to_string(),
                    None,
                ));
            }
        };

        let mut last_value = Value::Null;
        // 保存循环变量在循环外的旧值（如果存在）
        let outer_old_value = self.environment.get(var).cloned();

        for item in items {
            // 设置循环变量值
            self.environment.insert(var.to_string(), item);

            // 执行循环体
            last_value = self.eval_expr(body)?;
        }

        // 恢复循环外的旧值（如果存在）
        if let Some(old) = outer_old_value {
            self.environment.insert(var.to_string(), old);
        } else {
            // 只有在循环前变量不存在时才删除
            self.environment.remove(var);
        }

        Ok(last_value)
    }

    /// 评估While循环 / Evaluate while loop
    fn eval_while(&mut self, condition: &Expr, body: &Expr) -> Result<Value, InterpreterError> {
        let mut last_value = Value::Null;

        loop {
            // 评估条件
            let cond_value = self.eval_expr(condition)?;

            // 如果条件为假，退出循环
            if !self.is_truthy(&cond_value) {
                break;
            }

            // 执行循环体
            last_value = self.eval_expr(body)?;
        }

        Ok(last_value)
    }

    /// 评估Try-Catch异常处理 / Evaluate try-catch exception handling
    fn eval_try(
        &mut self,
        try_body: &Expr,
        catch_var: &Option<String>,
        catch_body: &Expr,
    ) -> Result<Value, InterpreterError> {
        // 尝试执行try块
        match self.eval_expr(try_body) {
            Ok(value) => Ok(value),
            Err(error) => {
                // 如果有catch变量，将错误信息绑定到变量
                if let Some(var) = catch_var {
                    let error_message = Value::String(error.to_string());
                    let old_value = self.environment.insert(var.clone(), error_message);

                    // 执行catch块
                    let result = self.eval_expr(catch_body)?;

                    // 恢复旧值（如果存在）
                    if let Some(old) = old_value {
                        self.environment.insert(var.clone(), old);
                    } else {
                        self.environment.remove(var.as_str());
                    }

                    Ok(result)
                } else {
                    // 没有catch变量，直接执行catch块
                    self.eval_expr(catch_body)
                }
            }
        }
    }

    /// 评估字面量 / Evaluate literal
    fn eval_literal(&mut self, lit: &Literal) -> Result<Value, InterpreterError> {
        match lit {
            Literal::Int(i) => Ok(Value::Int(*i)),
            Literal::Float(f) => Ok(Value::Float(*f)),
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Null => Ok(Value::Null),
            Literal::List(exprs) => {
                // 优化：预分配容量，减少重新分配
                let mut list = Vec::with_capacity(exprs.len());
                for expr in exprs {
                    list.push(self.eval_expr(expr)?);
                }
                Ok(Value::List(list))
            }
            Literal::Dict(pairs) => {
                let mut dict = std::collections::HashMap::new();
                for (key, expr) in pairs {
                    let value = self.eval_expr(expr)?;
                    dict.insert(key.clone(), value);
                }
                Ok(Value::Dict(dict))
            }
        }
    }

    /// 评估二元运算 / Evaluate binary operation
    fn eval_binary_op(
        &self,
        op: crate::grammar::core::BinOp,
        left: &Value,
        right: &Value,
    ) -> Result<Value, InterpreterError> {
        use crate::grammar::core::BinOp::*;
        match op {
            Add => self.add_values(left, right),
            Sub => self.sub_values(left, right),
            Mul => self.mul_values(left, right),
            Div => self.div_values(left, right),
            Mod => self.mod_values(left, right),
            Eq => Ok(Value::Bool(left == right)),
            Ne => Ok(Value::Bool(left != right)),
            Lt | Le | Gt | Ge => self.compare_values(op, left, right),
        }
    }

    /// 加法运算 / Add values
    fn add_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + *b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(*a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::List(a), Value::List(b)) => {
                let mut result = a.clone();
                result.extend_from_slice(b);
                Ok(Value::List(result))
            }
            _ => Err(InterpreterError::type_error(
                "Invalid types for addition".to_string(),
                None,
            )),
        }
    }

    /// 减法运算 / Subtract values
    fn sub_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err(InterpreterError::type_error(
                "Invalid types for subtraction".to_string(),
                None,
            )),
        }
    }

    /// 乘法运算 / Multiply values
    fn mul_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err(InterpreterError::type_error(
                "Invalid types for multiplication".to_string(),
                None,
            )),
        }
    }

    /// 除法运算 / Divide values
    fn div_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(InterpreterError::division_by_zero(None))
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(InterpreterError::division_by_zero(None))
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            _ => Err(InterpreterError::type_error(
                "Invalid types for division".to_string(),
                None,
            )),
        }
    }

    /// 取模运算 / Modulo values
    fn mod_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(InterpreterError::division_by_zero(None))
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(InterpreterError::division_by_zero(None))
                } else {
                    Ok(Value::Float(a % b))
                }
            }
            // 支持混合类型：Int 和 Float
            // Support mixed types: Int and Float
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(InterpreterError::division_by_zero(None))
                } else {
                    Ok(Value::Float((*a as f64) % b))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(InterpreterError::division_by_zero(None))
                } else {
                    Ok(Value::Float(a % (*b as f64)))
                }
            }
            _ => {
                // 添加调试信息以帮助定位问题
                // Add debug information to help locate the issue
                let left_type = match left {
                    Value::Int(_) => "Int",
                    Value::Float(_) => "Float",
                    Value::String(_) => "String",
                    Value::Bool(_) => "Bool",
                    Value::Null => "Null",
                    Value::List(_) => "List",
                    Value::Dict(_) => "Dict",
                    Value::Lambda { .. } => "Lambda",
                };
                let right_type = match right {
                    Value::Int(_) => "Int",
                    Value::Float(_) => "Float",
                    Value::String(_) => "String",
                    Value::Bool(_) => "Bool",
                    Value::Null => "Null",
                    Value::List(_) => "List",
                    Value::Dict(_) => "Dict",
                    Value::Lambda { .. } => "Lambda",
                };
                Err(InterpreterError::type_error(
                    format!("Invalid types for modulo: {} and {}", left_type, right_type),
                    None,
                ))
            }
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
            Value::List(list) => !list.is_empty(),
            Value::Dict(dict) => !dict.is_empty(),
            Value::Lambda { .. } => true, // Lambda总是为真
        }
    }

    /// 评估函数调用 / Evaluate function call
    fn eval_call(&mut self, name: &str, args: &[Expr]) -> Result<Value, InterpreterError> {
        // 检查是否是 lambda 表达式的错误转换
        // Check if this is a mis-converted lambda expression
        if name == "lambda" {
            // 将 Call("lambda", args) 转换回 GrammarElement::List
            // Convert Call("lambda", args) back to GrammarElement::List
            let mut lambda_elements = vec![GrammarElement::Atom("lambda".to_string())];
            for arg in args {
                lambda_elements.push(self.expr_to_element(arg)?);
            }
            return self.eval_list(&lambda_elements);
        }

        // 首先检查是否是Lambda值的调用
        // First check if it's a call to a Lambda value
        if let Some(Value::Lambda { id, params }) = self.environment.get(name).cloned() {
            return self.call_lambda(&id, &params, args);
        }

        // 检查是否是操作符（如 +, -, * 等）
        // Check if it's an operator (like +, -, *, etc.)
        // 操作符可以作为函数名直接调用，也可以作为变量传递
        // Operators can be called directly as function names, or passed as variables
        if name == "+"
            || name == "-"
            || name == "*"
            || name == "/"
            || name == "%"
            || name == "="
            || name == "=="
            || name == "!="
            || name == "<>"
            || name == "<"
            || name == ">"
            || name == "<="
            || name == ">="
        {
            // 操作符作为函数调用，需要转换为 op: 前缀
            // Operator as function call, need to convert to op: prefix
            let op_name = format!("op:{}", name);
            return self.eval_builtin_operator(&op_name, args);
        }

        // 检查变量值是否是操作符字符串（当操作符作为参数传递时）
        // Check if variable value is an operator string (when operator is passed as argument)
        // 先检查环境中的值
        // First check value in environment
        if let Some(Value::String(op_str)) = self.environment.get(name) {
            if op_str == "+"
                || op_str == "-"
                || op_str == "*"
                || op_str == "/"
                || op_str == "%"
                || op_str == "="
                || op_str == "=="
                || op_str == "!="
                || op_str == "<>"
                || op_str == "<"
                || op_str == ">"
                || op_str == "<="
                || op_str == ">="
            {
                let op_name = format!("op:{}", op_str);
                return self.eval_builtin_operator(&op_name, args);
            }
        }

        // 如果 name 本身是操作符字符串（当操作符直接作为函数名时）
        // If name itself is an operator string (when operator is used directly as function name)
        if name == "+"
            || name == "-"
            || name == "*"
            || name == "/"
            || name == "%"
            || name == "="
            || name == "=="
            || name == "!="
            || name == "<>"
            || name == "<"
            || name == ">"
            || name == "<="
            || name == ">="
        {
            let op_name = format!("op:{}", name);
            return self.eval_builtin_operator(&op_name, args);
        }

        // 检查参数中是否有临时存储的 Lambda 值，需要先评估参数
        // Check if arguments contain temporarily stored Lambda values, need to evaluate arguments first
        let mut arg_values = Vec::new();
        for arg in args {
            if let Expr::Var(var_name) = arg {
                if var_name.starts_with("__lambda_temp_") || var_name.starts_with("__lambda_arg_") {
                    // 从环境中获取 Lambda 值
                    // Get Lambda value from environment
                    if let Some(lambda_val) = self.environment.get(var_name).cloned() {
                        arg_values.push(lambda_val);
                        // 清理临时变量
                        // Clean up temporary variable
                        self.environment.remove(var_name);
                    } else {
                        // 如果找不到，尝试评估为普通变量
                        // If not found, try to evaluate as normal variable
                        arg_values.push(self.eval_expr(arg)?);
                    }
                } else {
                    // 检查是否是环境中的 Lambda 值（当变量是函数参数时）
                    // Check if it's a Lambda value in environment (when variable is function parameter)
                    if let Some(Value::Lambda { .. }) = self.environment.get(var_name) {
                        // 直接从环境中获取 Lambda 值
                        // Get Lambda value directly from environment
                        arg_values.push(self.environment.get(var_name).cloned().unwrap());
                    } else {
                        arg_values.push(self.eval_expr(arg)?);
                    }
                }
            } else {
                arg_values.push(self.eval_expr(arg)?);
            }
        }

        // 检查是否是内置操作符
        if name.starts_with("op:") {
            // 操作符需要 Expr 参数，所以我们需要转换回去
            // Operators need Expr arguments, so we need to convert back
            let op_args: Vec<Expr> = arg_values
                .iter()
                .map(|v| self.value_to_expr(v.clone()))
                .collect::<Result<Vec<_>, _>>()?;
            return self.eval_builtin_operator(name, &op_args);
        }

        // 检查是否是用户定义函数（需要克隆以避免借用冲突）
        if let Some(func) = self.functions.get(name).cloned() {
            // 用户定义函数：直接传递 Value，在函数内部处理
            // User-defined functions: pass Value directly, handle inside function
            return self.call_user_function_with_values(&func, &arg_values);
        }

        // 如果找不到函数且函数名不包含命名空间，尝试在所有已导入的模块中查找
        // If function not found and name doesn't contain namespace, try to find in all imported modules
        if !name.contains('.') {
            // 先尝试当前模块
            if let Some(ref module_name) = self.current_module {
                if let Some(module) = self.modules.get(module_name) {
                    if let Some(func) = module.functions.get(name).cloned() {
                        // 找到模块内的函数，调用它
                        // Found function in module, call it
                        return self.call_user_function_with_values(&func, &arg_values);
                    }
                }
            }

            // 尝试所有已导入的模块
            // Try all imported modules
            for module in self.modules.values() {
                if let Some(func) = module.functions.get(name).cloned() {
                    // 找到模块内的函数，调用它
                    // Found function in module, call it
                    return self.call_user_function_with_values(&func, &arg_values);
                }
            }
        }

        // 检查是否是内置函数
        // Check if built-in function
        // 将 Value 转换回 Expr（Lambda 值需要特殊处理）
        // Convert Value back to Expr (Lambda values need special handling)
        let mut func_args = Vec::new();
        for val in arg_values {
            // Lambda 值无法转换为 Expr，需要存储到环境中
            // Lambda values cannot be converted to Expr, need to store in environment
            if let Value::Lambda { .. } = val {
                // 创建临时变量名
                // Create temporary variable name
                let temp_name = format!("__lambda_arg_{}", func_args.len());
                self.environment.insert(temp_name.clone(), val);
                func_args.push(Expr::Var(temp_name));
            } else {
                func_args.push(self.value_to_expr(val)?);
            }
        }
        self.eval_builtin_function(name, &func_args)
    }

    /// 评估内置操作符 / Evaluate built-in operator
    fn eval_builtin_operator(
        &mut self,
        name: &str,
        args: &[Expr],
    ) -> Result<Value, InterpreterError> {
        let op_str = name.strip_prefix("op:").unwrap();

        // 特殊处理：+ 运算符支持可变参数
        // Special handling: + operator supports variadic arguments
        if op_str == "+" {
            if args.is_empty() {
                return Err(InterpreterError::runtime_error(
                    "Operator + requires at least 1 argument".to_string(),
                    None,
                ));
            }

            // 评估所有参数
            // Evaluate all arguments
            let mut values = Vec::new();
            for arg in args {
                values.push(self.eval_expr(arg)?);
            }

            // 如果只有一个参数，直接返回
            // If only one argument, return it directly
            if values.len() == 1 {
                return Ok(values[0].clone());
            }

            // 从左到右依次相加
            // Add from left to right
            let mut result = values[0].clone();
            for value in values.iter().skip(1) {
                result = self.add_values(&result, value)?;
            }
            return Ok(result);
        }

        // 其他运算符需要2个参数
        // Other operators require 2 arguments
        if args.len() != 2 {
            return Err(InterpreterError::runtime_error(
                format!("Operator {} requires 2 arguments", name),
                None,
            ));
        }

        let left = self.eval_expr(&args[0])?;
        let right = self.eval_expr(&args[1])?;

        let op = match op_str {
            "-" => BinOp::Sub,
            "*" => BinOp::Mul,
            "/" => BinOp::Div,
            "%" => BinOp::Mod,
            "=" | "==" => BinOp::Eq,
            "!=" | "<>" => BinOp::Ne,
            "<" => BinOp::Lt,
            ">" => BinOp::Gt,
            "<=" => BinOp::Le,
            ">=" => BinOp::Ge,
            _ => {
                return Err(InterpreterError::runtime_error(
                    format!("Unknown operator: {}", op_str),
                    None,
                ))
            }
        };

        self.eval_binary_op(op, &left, &right)
    }

    /// 调用Lambda函数 / Call Lambda function
    fn call_lambda(
        &mut self,
        lambda_id: &str,
        _params: &[String],
        args: &[Expr],
    ) -> Result<Value, InterpreterError> {
        // 从注册表中获取Lambda函数体和捕获的环境
        let (registered_params, body, captured_env) = self
            .lambda_registry
            .get(lambda_id)
            .ok_or_else(|| {
                InterpreterError::runtime_error(
                    format!("Lambda {} not found in registry", lambda_id),
                    None,
                )
            })?
            .clone();

        // 使用注册表中的参数列表，而不是传入的参数列表
        // Use parameter list from registry instead of passed parameters
        let params = &registered_params;

        if args.len() != params.len() {
            return Err(InterpreterError::runtime_error(
                format!(
                    "Lambda expects {} arguments, got {}",
                    params.len(),
                    args.len()
                ),
                None,
            ));
        }

        // 评估参数
        let arg_values: Vec<Value> = args
            .iter()
            .map(|e| self.eval_expr(e))
            .collect::<Result<Vec<_>, _>>()?;

        // 保存当前环境（用于恢复）- 优化：只保存被修改的变量
        let mut saved_env = HashMap::new();
        let mut saved_params = HashMap::new();

        // 首先恢复捕获的环境（闭包变量）- 优化：使用引用避免不必要的克隆
        for (key, value) in &captured_env {
            // 只在环境中有旧值时才保存
            if self.environment.contains_key(key) {
                if let Some(old) = self.environment.insert(key.clone(), value.clone()) {
                    saved_env.insert(key.clone(), old);
                }
            } else {
                // 新变量，直接插入
                self.environment.insert(key.clone(), value.clone());
            }
        }

        // 然后设置参数（参数会遮蔽捕获的环境中的同名变量）
        for (param, value) in params.iter().zip(arg_values.iter()) {
            if let Some(old) = self.environment.insert(param.clone(), value.clone()) {
                saved_params.insert(param.clone(), old);
            }
        }

        // 执行Lambda函数体
        let result = self.eval_element(&body)?;

        // 恢复环境：先恢复参数，再恢复捕获的环境 - 优化：使用更高效的方式
        for param in params {
            if let Some(old) = saved_params.remove(param) {
                self.environment.insert(param.clone(), old);
            } else {
                self.environment.remove(param);
            }
        }

        // 恢复捕获的环境（只恢复之前存在的变量）
        let saved_env_keys: Vec<String> = saved_env.keys().cloned().collect();
        for (key, old_value) in saved_env {
            self.environment.insert(key, old_value);
        }

        // 移除捕获环境中新增的变量（Lambda执行时新增的）
        for key in captured_env.keys() {
            if !saved_env_keys.contains(key) && !params.contains(key) {
                // 这个变量是Lambda执行时新增的，不应该保留
                self.environment.remove(key);
            }
        }

        // 移除Lambda执行时新增的变量（这些变量不在捕获环境中，也不在参数中）
        // 注意：这里我们只移除那些在Lambda执行前不存在于环境中的变量
        // 由于我们已经恢复了saved_env中的变量，这里不需要额外处理

        Ok(result)
    }

    /// 调用用户定义函数（使用 Value 参数）/ Call user-defined function (with Value arguments)
    fn call_user_function_with_values(
        &mut self,
        func: &Function,
        arg_values: &[Value],
    ) -> Result<Value, InterpreterError> {
        if arg_values.len() != func.params.len() {
            return Err(InterpreterError::runtime_error(
                format!(
                    "Function expects {} arguments, got {}",
                    func.params.len(),
                    arg_values.len()
                ),
                None,
            ));
        }

        // 保存当前环境 - 优化：只保存被修改的变量
        let mut saved_env = HashMap::new();
        for (param, value) in func.params.iter().zip(arg_values.iter()) {
            // 只在环境中有旧值时才保存
            if let Some(old) = self.environment.insert(param.clone(), value.clone()) {
                saved_env.insert(param.clone(), old);
            }
        }

        // 保存并设置当前模块名（用于递归调用时查找模块内函数）
        let saved_module = self.current_module.clone();
        if let Some(ref module_name) = func.module_name {
            self.current_module = Some(module_name.clone());
        }

        // 执行函数体
        let result = self.eval_element(&func.body)?;

        // 恢复环境 - 优化：使用更高效的方式
        for param in &func.params {
            if let Some(old) = saved_env.remove(param) {
                self.environment.insert(param.clone(), old);
            } else {
                self.environment.remove(param);
            }
        }

        // 恢复当前模块名
        self.current_module = saved_module;

        Ok(result)
    }

    /// 调用用户定义函数 / Call user-defined function
    fn call_user_function(
        &mut self,
        func: &Function,
        args: &[Expr],
    ) -> Result<Value, InterpreterError> {
        if args.len() != func.params.len() {
            return Err(InterpreterError::runtime_error(
                format!(
                    "Function expects {} arguments, got {}",
                    func.params.len(),
                    args.len()
                ),
                None,
            ));
        }

        // 评估参数
        let arg_values: Vec<Value> = args
            .iter()
            .map(|e| self.eval_expr(e))
            .collect::<Result<Vec<_>, _>>()?;

        // 调用 with_values 版本
        self.call_user_function_with_values(func, &arg_values)
    }

    /// 评估内置函数 / Evaluate built-in function
    fn eval_builtin_function(
        &mut self,
        name: &str,
        args: &[Expr],
    ) -> Result<Value, InterpreterError> {
        match name {
            "import" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(InterpreterError::runtime_error(
                        "import requires 1 or 2 arguments: module_name [alias]".to_string(),
                        None,
                    ));
                }
                let module_name = self.module_name_from_expr(&args[0])?;
                let alias = if args.len() == 2 {
                    self.module_name_from_expr(&args[1])?
                } else {
                    module_name.clone()
                };
                self.import_module(&module_name, &alias)?;
                Ok(Value::Null)
            }
            "print" => {
                use std::io::Write;
                for (i, arg) in args.iter().enumerate() {
                    let value = self.eval_expr(arg)?;
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{}", value);
                }
                println!();
                // 强制刷新输出缓冲区 / Force flush output buffer
                std::io::stdout().flush().unwrap();
                Ok(Value::Null)
            }
            // 列表操作 / List operations
            "list-get" | "get" => {
                if args.len() != 2 {
                    return Err(InterpreterError::runtime_error(
                        "list-get requires 2 arguments: list and index".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let index = self.eval_expr(&args[1])?;
                match (list, index) {
                    (Value::List(l), Value::Int(i)) => {
                        if i < 0 || i as usize >= l.len() {
                            Err(InterpreterError::runtime_error(
                                format!("Index {} out of bounds for list of length {}", i, l.len()),
                                None,
                            ))
                        } else {
                            Ok(l[i as usize].clone())
                        }
                    }
                    _ => Err(InterpreterError::type_error(
                        "list-get requires a list and an integer index".to_string(),
                        None,
                    )),
                }
            }
            "list-set" | "set" => {
                if args.len() != 3 {
                    return Err(InterpreterError::runtime_error(
                        "list-set requires 3 arguments: list, index, value".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let index = self.eval_expr(&args[1])?;
                let value = self.eval_expr(&args[2])?;
                match (list, index) {
                    (Value::List(mut l), Value::Int(i)) => {
                        if i < 0 || i as usize >= l.len() {
                            Err(InterpreterError::runtime_error(
                                format!("Index {} out of bounds for list of length {}", i, l.len()),
                                None,
                            ))
                        } else {
                            l[i as usize] = value;
                            Ok(Value::List(l))
                        }
                    }
                    _ => Err(InterpreterError::type_error(
                        "list-set requires a list, an integer index, and a value".to_string(),
                        None,
                    )),
                }
            }
            "list-append" | "append" => {
                if args.len() != 2 {
                    return Err(InterpreterError::runtime_error(
                        "list-append requires 2 arguments: list and value".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let value = self.eval_expr(&args[1])?;
                match list {
                    Value::List(mut l) => {
                        l.push(value);
                        Ok(Value::List(l))
                    }
                    _ => Err(InterpreterError::type_error(
                        "list-append requires a list".to_string(),
                        None,
                    )),
                }
            }
            "list-length" | "length" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "list-length requires 1 argument: list".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                match list {
                    Value::List(l) => Ok(Value::Int(l.len() as i64)),
                    _ => Err(InterpreterError::type_error(
                        "list-length requires a list".to_string(),
                        None,
                    )),
                }
            }
            "list-concat" | "concat" => {
                if args.len() < 2 {
                    return Err(InterpreterError::runtime_error(
                        "list-concat requires at least 2 arguments: list1, list2, ... or str1, str2, ...".to_string(),
                        None,
                    ));
                }
                // 检查所有参数是否都是列表或都是字符串
                let mut all_lists = true;
                let mut all_strings = true;
                let mut values = Vec::new();

                for arg in args {
                    let value = self.eval_expr(arg)?;
                    values.push(value.clone());
                    match value {
                        Value::List(_) => all_strings = false,
                        Value::String(_) => all_lists = false,
                        _ => {
                            all_lists = false;
                            all_strings = false;
                        }
                    }
                }

                if all_lists {
                    // 处理列表连接
                    let mut result = Vec::new();
                    for value in values {
                        if let Value::List(l) = value {
                            result.extend(l);
                        }
                    }
                    Ok(Value::List(result))
                } else if all_strings {
                    // 处理字符串连接
                    let mut result = String::new();
                    for value in values {
                        if let Value::String(s) = value {
                            result.push_str(&s);
                        }
                    }
                    Ok(Value::String(result))
                } else {
                    return Err(InterpreterError::type_error(
                        "list-concat requires all lists or all strings".to_string(),
                        None,
                    ));
                }
            }
            // 字典操作 / Dictionary operations
            "dict-get" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(InterpreterError::runtime_error(
                        "dict-get requires 2 or 3 arguments: dict, key [default]".to_string(),
                        None,
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                let key = self.eval_expr(&args[1])?;
                let default_value = if args.len() == 3 {
                    self.eval_expr(&args[2])?
                } else {
                    Value::Null
                };
                match (dict, key) {
                    (Value::Dict(d), Value::String(k)) => {
                        Ok(d.get(&k).cloned().unwrap_or(default_value))
                    }
                    _ => Err(InterpreterError::type_error(
                        "dict-get requires a dict and a string key".to_string(),
                        None,
                    )),
                }
            }
            "dict-set" => {
                if args.len() != 3 {
                    return Err(InterpreterError::runtime_error(
                        "dict-set requires 3 arguments: dict, key, value".to_string(),
                        None,
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                let key = self.eval_expr(&args[1])?;
                let value = self.eval_expr(&args[2])?;
                match (dict, key) {
                    (Value::Dict(mut d), Value::String(k)) => {
                        d.insert(k, value);
                        Ok(Value::Dict(d))
                    }
                    _ => Err(InterpreterError::type_error(
                        "dict-set requires a dict, a string key, and a value".to_string(),
                        None,
                    )),
                }
            }
            "dict-keys" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "dict-keys requires 1 argument: dict".to_string(),
                        None,
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                match dict {
                    Value::Dict(d) => {
                        let keys: Vec<Value> = d.keys().map(|k| Value::String(k.clone())).collect();
                        Ok(Value::List(keys))
                    }
                    _ => Err(InterpreterError::type_error(
                        "dict-keys requires a dict".to_string(),
                        None,
                    )),
                }
            }
            "dict-values" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "dict-values requires 1 argument: dict".to_string(),
                        None,
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                match dict {
                    Value::Dict(d) => {
                        let values: Vec<Value> = d.values().cloned().collect();
                        Ok(Value::List(values))
                    }
                    _ => Err(InterpreterError::type_error(
                        "dict-values requires a dict".to_string(),
                        None,
                    )),
                }
            }
            "dict-has" => {
                if args.len() != 2 {
                    return Err(InterpreterError::runtime_error(
                        "dict-has requires 2 arguments: dict and key".to_string(),
                        None,
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                let key = self.eval_expr(&args[1])?;
                match (dict, key) {
                    (Value::Dict(d), Value::String(k)) => Ok(Value::Bool(d.contains_key(&k))),
                    _ => Err(InterpreterError::type_error(
                        "dict-has requires a dict and a string key".to_string(),
                        None,
                    )),
                }
            }

            // 字符串操作 / String operations
            "string-split" | "split" => {
                if args.len() != 2 {
                    return Err(InterpreterError::runtime_error(
                        "string-split requires 2 arguments: string and delimiter".to_string(),
                        None,
                    ));
                }
                let string = self.eval_expr(&args[0])?;
                let delimiter = self.eval_expr(&args[1])?;
                match (string, delimiter) {
                    (Value::String(s), Value::String(d)) => {
                        let parts: Vec<Value> = s
                            .split(&d)
                            .map(|part| Value::String(part.to_string()))
                            .collect();
                        Ok(Value::List(parts))
                    }
                    _ => Err(InterpreterError::type_error(
                        "string-split requires two strings".to_string(),
                        None,
                    )),
                }
            }
            "string-join" | "join" => {
                if args.len() != 2 {
                    return Err(InterpreterError::runtime_error(
                        "string-join requires 2 arguments: list and delimiter".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let delimiter = self.eval_expr(&args[1])?;
                match (list, delimiter) {
                    (Value::List(l), Value::String(d)) => {
                        let strings: Vec<String> = l
                            .iter()
                            .filter_map(|v| {
                                if let Value::String(s) = v {
                                    Some(s.clone())
                                } else {
                                    None
                                }
                            })
                            .collect();
                        Ok(Value::String(strings.join(&d)))
                    }
                    _ => Err(InterpreterError::type_error(
                        "string-join requires a list of strings and a string delimiter".to_string(),
                        None,
                    )),
                }
            }
            "string-trim" | "trim" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "string-trim requires 1 argument: string".to_string(),
                        None,
                    ));
                }
                let string = self.eval_expr(&args[0])?;
                match string {
                    Value::String(s) => Ok(Value::String(s.trim().to_string())),
                    _ => Err(InterpreterError::type_error(
                        "string-trim requires a string".to_string(),
                        None,
                    )),
                }
            }
            "string-replace" | "replace" => {
                if args.len() != 3 {
                    return Err(InterpreterError::runtime_error(
                        "string-replace requires 3 arguments: string, old, new".to_string(),
                        None,
                    ));
                }
                let string = self.eval_expr(&args[0])?;
                let old_str = self.eval_expr(&args[1])?;
                let new_str = self.eval_expr(&args[2])?;
                match (string, old_str, new_str) {
                    (Value::String(s), Value::String(o), Value::String(n)) => {
                        Ok(Value::String(s.replace(&o, &n)))
                    }
                    _ => Err(InterpreterError::type_error(
                        "string-replace requires three strings".to_string(),
                        None,
                    )),
                }
            }
            "string-length" | "strlen" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "string-length requires 1 argument: string".to_string(),
                        None,
                    ));
                }
                let string = self.eval_expr(&args[0])?;
                match string {
                    Value::String(s) => Ok(Value::Int(s.len() as i64)),
                    _ => Err(InterpreterError::type_error(
                        "string-length requires a string".to_string(),
                        None,
                    )),
                }
            }
            "string-substring" | "substring" => {
                if args.len() != 3 {
                    return Err(InterpreterError::runtime_error(
                        "string-substring requires 3 arguments: string, start, end".to_string(),
                        None,
                    ));
                }
                let string = self.eval_expr(&args[0])?;
                let start = self.eval_expr(&args[1])?;
                let end = self.eval_expr(&args[2])?;
                match (string, start, end) {
                    (Value::String(s), Value::Int(st), Value::Int(e)) => {
                        let start_idx = (st as usize).min(s.len());
                        let end_idx = (e as usize).min(s.len());
                        if start_idx > end_idx {
                            Ok(Value::String(String::new()))
                        } else {
                            Ok(Value::String(s[start_idx..end_idx].to_string()))
                        }
                    }
                    _ => Err(InterpreterError::type_error(
                        "string-substring requires a string and two integers".to_string(),
                        None,
                    )),
                }
            }
            "string-upper" | "upper" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "string-upper requires 1 argument: string".to_string(),
                        None,
                    ));
                }
                let string = self.eval_expr(&args[0])?;
                match string {
                    Value::String(s) => Ok(Value::String(s.to_uppercase())),
                    _ => Err(InterpreterError::type_error(
                        "string-upper requires a string".to_string(),
                        None,
                    )),
                }
            }
            "string-lower" | "lower" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "string-lower requires 1 argument: string".to_string(),
                        None,
                    ));
                }
                let string = self.eval_expr(&args[0])?;
                match string {
                    Value::String(s) => Ok(Value::String(s.to_lowercase())),
                    _ => Err(InterpreterError::type_error(
                        "string-lower requires a string".to_string(),
                        None,
                    )),
                }
            }
            // 类型转换 / Type conversion
            "to-string" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "to-string requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                Ok(Value::String(value.to_string()))
            }
            "to-int" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "to-int requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                match value {
                    Value::Int(i) => Ok(Value::Int(i)),
                    Value::Float(f) => Ok(Value::Int(f as i64)),
                    Value::String(s) => s.parse::<i64>().map(Value::Int).map_err(|_| {
                        InterpreterError::type_error(
                            format!("Cannot convert '{}' to integer", s),
                            None,
                        )
                    }),
                    _ => Err(InterpreterError::type_error(
                        "Cannot convert to integer".to_string(),
                        None,
                    )),
                }
            }
            "to-float" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "to-float requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                match value {
                    Value::Int(i) => Ok(Value::Float(i as f64)),
                    Value::Float(f) => Ok(Value::Float(f)),
                    Value::String(s) => s.parse::<f64>().map(Value::Float).map_err(|_| {
                        InterpreterError::type_error(
                            format!("Cannot convert '{}' to float", s),
                            None,
                        )
                    }),
                    _ => Err(InterpreterError::type_error(
                        "Cannot convert to float".to_string(),
                        None,
                    )),
                }
            }
            // 类型检查 / Type checking
            "is-string" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "is-string requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                Ok(Value::Bool(matches!(value, Value::String(_))))
            }
            "is-int" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "is-int requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                Ok(Value::Bool(matches!(value, Value::Int(_))))
            }
            "is-float" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "is-float requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                Ok(Value::Bool(matches!(value, Value::Float(_))))
            }
            "is-bool" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "is-bool requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                Ok(Value::Bool(matches!(value, Value::Bool(_))))
            }
            "is-list" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "is-list requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                Ok(Value::Bool(matches!(value, Value::List(_))))
            }
            "is-dict" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "is-dict requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                Ok(Value::Bool(matches!(value, Value::Dict(_))))
            }
            "is-null" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "is-null requires 1 argument".to_string(),
                        None,
                    ));
                }
                let value = self.eval_expr(&args[0])?;
                Ok(Value::Bool(matches!(value, Value::Null)))
            }
            // 增强列表操作 / Enhanced list operations
            "list-slice" | "slice" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(InterpreterError::runtime_error(
                        "list-slice requires 2 or 3 arguments: list, start, [end]".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let start = self.eval_expr(&args[1])?;
                let end = if args.len() == 3 {
                    Some(self.eval_expr(&args[2])?)
                } else {
                    None
                };
                match (list, start, end) {
                    (Value::List(l), Value::Int(s), Some(Value::Int(e))) => {
                        let start_idx = if s < 0 {
                            (l.len() as i64 + s).max(0) as usize
                        } else {
                            (s as usize).min(l.len())
                        };
                        let end_idx = if e < 0 {
                            (l.len() as i64 + e).max(0) as usize
                        } else {
                            (e as usize).min(l.len())
                        };
                        if start_idx > end_idx {
                            Ok(Value::List(vec![]))
                        } else {
                            Ok(Value::List(l[start_idx..end_idx].to_vec()))
                        }
                    }
                    (Value::List(l), Value::Int(s), None) => {
                        let start_idx = if s < 0 {
                            (l.len() as i64 + s).max(0) as usize
                        } else {
                            (s as usize).min(l.len())
                        };
                        Ok(Value::List(l[start_idx..].to_vec()))
                    }
                    _ => Err(InterpreterError::type_error(
                        "list-slice requires a list and integer indices".to_string(),
                        None,
                    )),
                }
            }
            "list-reverse" | "reverse" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "list-reverse requires 1 argument: list".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                match list {
                    Value::List(mut l) => {
                        l.reverse();
                        Ok(Value::List(l))
                    }
                    _ => Err(InterpreterError::type_error(
                        "list-reverse requires a list".to_string(),
                        None,
                    )),
                }
            }
            "list-sort" | "sort" => {
                if args.len() < 1 || args.len() > 2 {
                    return Err(InterpreterError::runtime_error(
                        "list-sort requires 1 or 2 arguments: list, [comparator]".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let comparator = if args.len() == 2 {
                    Some(self.eval_expr(&args[1])?)
                } else {
                    None
                };
                match (list, comparator) {
                    (Value::List(mut l), None) => {
                        // 默认排序：尝试按数值或字符串排序
                        l.sort_by(|a, b| match (a, b) {
                            (Value::Int(i1), Value::Int(i2)) => i1.cmp(i2),
                            (Value::Float(f1), Value::Float(f2)) => {
                                f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Equal)
                            }
                            (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                            _ => std::cmp::Ordering::Equal,
                        });
                        Ok(Value::List(l))
                    }
                    (Value::List(l), Some(Value::Lambda { id: _, params })) => {
                        if params.len() != 2 {
                            return Err(InterpreterError::runtime_error(
                                "sort comparator must accept exactly 2 arguments".to_string(),
                                None,
                            ));
                        }
                        // 使用Lambda比较函数排序 - 先收集所有比较结果，然后排序
                        let mut indexed: Vec<(usize, Value)> = l.into_iter().enumerate().collect();
                        // 简单排序：对于复杂情况，使用默认排序
                        // 注意：带比较函数的排序需要更复杂的实现，这里简化处理
                        indexed.sort_by(|(_, a), (_, b)| match (a, b) {
                            (Value::Int(i1), Value::Int(i2)) => i1.cmp(i2),
                            (Value::Float(f1), Value::Float(f2)) => {
                                f1.partial_cmp(f2).unwrap_or(std::cmp::Ordering::Equal)
                            }
                            (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
                            _ => std::cmp::Ordering::Equal,
                        });
                        let result: Vec<Value> = indexed.into_iter().map(|(_, v)| v).collect();
                        Ok(Value::List(result))
                    }
                    _ => Err(InterpreterError::type_error(
                        "list-sort requires a list".to_string(),
                        None,
                    )),
                }
            }
            "list-unique" | "unique" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "list-unique requires 1 argument: list".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                match list {
                    Value::List(l) => {
                        let mut seen = Vec::new();
                        let mut result = Vec::new();
                        for item in l {
                            if !seen.contains(&item) {
                                seen.push(item.clone());
                                result.push(item);
                            }
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err(InterpreterError::type_error(
                        "list-unique requires a list".to_string(),
                        None,
                    )),
                }
            }
            "list-flatten" | "flatten" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "list-flatten requires 1 argument: list".to_string(),
                        None,
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                match list {
                    Value::List(l) => {
                        let mut result = Vec::new();
                        for item in l {
                            match item {
                                Value::List(inner) => result.extend(inner),
                                other => result.push(other),
                            }
                        }
                        Ok(Value::List(result))
                    }
                    _ => Err(InterpreterError::type_error(
                        "list-flatten requires a list".to_string(),
                        None,
                    )),
                }
            }
            // 增强字典操作 / Enhanced dictionary operations
            "dict-merge" | "merge" => {
                if args.len() < 2 {
                    return Err(InterpreterError::runtime_error(
                        "dict-merge requires at least 2 arguments: dict1, dict2, ...".to_string(),
                        None,
                    ));
                }
                let mut result = HashMap::new();
                for arg in args {
                    let dict = self.eval_expr(arg)?;
                    match dict {
                        Value::Dict(d) => {
                            for (k, v) in d {
                                result.insert(k, v);
                            }
                        }
                        _ => {
                            return Err(InterpreterError::type_error(
                                "dict-merge requires dictionaries".to_string(),
                                None,
                            ))
                        }
                    }
                }
                Ok(Value::Dict(result))
            }
            "dict-size" | "dict-length" => {
                if args.len() != 1 {
                    return Err(InterpreterError::runtime_error(
                        "dict-size requires 1 argument: dict".to_string(),
                        None,
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                match dict {
                    Value::Dict(d) => Ok(Value::Int(d.len() as i64)),
                    _ => Err(InterpreterError::type_error(
                        "dict-size requires a dict".to_string(),
                        None,
                    )),
                }
            }
            _ => Err(InterpreterError::runtime_error(
                format!("Unknown function: {}", name),
                None,
            )),
        }
    }

    /// 从表达式解析模块名称 / Parse module name from expression
    fn module_name_from_expr(&self, expr: &Expr) -> Result<String, InterpreterError> {
        match expr {
            Expr::Literal(Literal::String(s)) => Ok(s.clone()),
            Expr::Var(name) => Ok(name.clone()),
            _ => Err(InterpreterError::runtime_error(
                "Module name must be a string literal or identifier".to_string(),
                None,
            )),
        }
    }

    /// 导入模块 / Import module
    fn import_module(&mut self, module_name: &str, alias: &str) -> Result<(), InterpreterError> {
        let module = if let Some(module) = self.modules.get(module_name).cloned() {
            module
        } else {
            let module = self.load_module(module_name)?;
            self.modules.insert(module_name.to_string(), module.clone());
            module
        };

        // 将模块内容导入到当前环境（带命名空间前缀）
        for (name, value) in &module.environment {
            let qualified_name = format!("{}.{}", alias, name);
            self.environment.insert(qualified_name, value.clone());
        }
        for (name, mut function) in module.functions {
            let qualified_name = format!("{}.{}", alias, name);
            // 保留模块名信息，用于递归调用时查找
            function.module_name = Some(module.name.clone());
            self.functions.insert(qualified_name, function);
        }

        Ok(())
    }

    /// 加载模块 / Load module
    fn load_module(&self, module_name: &str) -> Result<Module, InterpreterError> {
        let path = self.resolve_module_path(module_name)?;
        let code = fs::read_to_string(&path).map_err(|e| {
            InterpreterError::runtime_error(
                format!("Failed to read module '{}': {}", module_name, e),
                None,
            )
        })?;

        let parser = AdaptiveParser::new(true);
        let ast = parser.parse(&code).map_err(|e| {
            InterpreterError::runtime_error(
                format!("Failed to parse module '{}': {:?}", module_name, e),
                None,
            )
        })?;

        let mut module_interpreter = Interpreter::new();
        module_interpreter.execute(&ast).map_err(|e| {
            InterpreterError::runtime_error(
                format!("Failed to execute module '{}': {:?}", module_name, e),
                None,
            )
        })?;

        // 为模块中的函数设置模块名
        let mut module_functions = HashMap::new();
        for (name, mut func) in module_interpreter.functions {
            func.module_name = Some(module_name.to_string());
            module_functions.insert(name, func);
        }

        Ok(Module {
            name: module_name.to_string(),
            environment: module_interpreter.environment.clone(),
            functions: module_functions,
        })
    }

    /// 解析模块路径 / Resolve module path
    fn resolve_module_path(&self, module_name: &str) -> Result<PathBuf, InterpreterError> {
        let mut candidates = Vec::new();
        let name = if module_name.ends_with(".evo") {
            module_name.to_string()
        } else {
            format!("{}.evo", module_name)
        };

        candidates.push(PathBuf::from("modules").join(&name));
        candidates.push(PathBuf::from("examples").join(&name));
        candidates.push(PathBuf::from(&name));

        for path in candidates {
            if path.exists() {
                return Ok(path);
            }
        }

        Err(InterpreterError::runtime_error(
            format!(
                "Module '{}' not found in modules/, examples/, or current directory",
                module_name
            ),
            None,
        ))
    }

    /// 比较值 / Compare values
    fn compare_values(
        &self,
        op: BinOp,
        left: &Value,
        right: &Value,
    ) -> Result<Value, InterpreterError> {
        use BinOp::*;
        let result = match (left, right) {
            (Value::Int(a), Value::Int(b)) => match op {
                Lt => a < b,
                Le => a <= b,
                Gt => a > b,
                Ge => a >= b,
                _ => unreachable!(),
            },
            (Value::Float(a), Value::Float(b)) => match op {
                Lt => a < b,
                Le => a <= b,
                Gt => a > b,
                Ge => a >= b,
                _ => unreachable!(),
            },
            (Value::String(a), Value::String(b)) => match op {
                Lt => a < b,
                Le => a <= b,
                Gt => a > b,
                Ge => a >= b,
                _ => unreachable!(),
            },
            _ => {
                return Err(InterpreterError::type_error(
                    format!(
                        "Cannot compare {} and {}",
                        self.value_type_name(left),
                        self.value_type_name(right)
                    ),
                    None,
                ));
            }
        };
        Ok(Value::Bool(result))
    }

    /// 获取值类型名称 / Get value type name
    fn value_type_name(&self, value: &Value) -> &str {
        match value {
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Bool(_) => "Bool",
            Value::Null => "Null",
            Value::List(_) => "List",
            Value::Dict(_) => "Dict",
            Value::Lambda { .. } => "Lambda",
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
    /// 列表 / List
    List(Vec<Value>),
    /// 字典 / Dictionary
    Dict(std::collections::HashMap<String, Value>),
    /// Lambda函数 / Lambda function (closure)
    /// 注意：Lambda使用ID来标识，实际函数体在解释器的lambda_registry中存储
    /// Note: Lambda uses ID to identify, actual body is stored in interpreter's lambda_registry
    Lambda {
        /// Lambda标识符 / Lambda identifier (用于查找函数体)
        id: String,
        /// 参数列表 / Parameter names
        params: Vec<String>,
    },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::List(list) => {
                write!(f, "[")?;
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Dict(dict) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in dict {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            Value::Lambda { params, .. } => {
                write!(f, "<lambda({})>", params.join(", "))
            }
        }
    }
}

/// 源代码位置 / Source code location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// 行号（从1开始）/ Line number (1-based)
    pub line: usize,
    /// 列号（从1开始）/ Column number (1-based)
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn format(&self) -> String {
        format!("line {}, column {}", self.line, self.column)
    }
}

/// 解释器错误 / Interpreter error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpreterError {
    /// 未实现 / Not implemented
    NotImplemented,
    /// 未定义变量 / Undefined variable
    UndefinedVariable {
        name: String,
        location: Option<Location>,
    },
    /// 类型错误 / Type error
    TypeError {
        message: String,
        location: Option<Location>,
    },
    /// 除以零 / Division by zero
    DivisionByZero { location: Option<Location> },
    /// 运行时错误 / Runtime error
    RuntimeError {
        message: String,
        location: Option<Location>,
    },
}

impl InterpreterError {
    /// 创建未定义变量错误 / Create undefined variable error
    pub fn undefined_variable(name: String, location: Option<Location>) -> Self {
        Self::UndefinedVariable { name, location }
    }

    /// 创建类型错误 / Create type error
    pub fn type_error(message: String, location: Option<Location>) -> Self {
        Self::TypeError { message, location }
    }

    /// 创建运行时错误 / Create runtime error
    pub fn runtime_error(message: String, location: Option<Location>) -> Self {
        Self::RuntimeError { message, location }
    }

    /// 创建除以零错误 / Create division by zero error
    pub fn division_by_zero(location: Option<Location>) -> Self {
        Self::DivisionByZero { location }
    }
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented => write!(f, "Not implemented"),
            Self::UndefinedVariable { name, location } => {
                if let Some(loc) = location {
                    write!(f, "Undefined variable '{}' at {}", name, loc.format())
                } else {
                    write!(f, "Undefined variable '{}'", name)
                }
            }
            Self::TypeError { message, location } => {
                if let Some(loc) = location {
                    write!(f, "Type error at {}: {}", loc.format(), message)
                } else {
                    write!(f, "Type error: {}", message)
                }
            }
            Self::DivisionByZero { location } => {
                if let Some(loc) = location {
                    write!(f, "Division by zero at {}", loc.format())
                } else {
                    write!(f, "Division by zero")
                }
            }
            Self::RuntimeError { message, location } => {
                if let Some(loc) = location {
                    write!(f, "Runtime error at {}: {}", loc.format(), message)
                } else {
                    write!(f, "Runtime error: {}", message)
                }
            }
        }
    }
}

impl std::error::Error for InterpreterError {}
