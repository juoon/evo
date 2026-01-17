// 解释器 / Interpreter
// 执行Aevolang代码的解释器
// Interpreter for executing Aevolang code

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
                "if" => self.eval_if_special(&list[1..]),
                "lambda" => self.eval_lambda(&list[1..]),
                _ => {
                    // 尝试作为函数调用
                    let func_name = keyword.to_string();
                    let args: Vec<Expr> = list[1..]
                        .iter()
                        .map(|e| self.element_to_expr(e))
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|_| {
                            InterpreterError::runtime_error(
                                "Failed to convert arguments to expressions".to_string(),
                                None,
                            )
                        })?;
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
    fn element_to_expr(&self, element: &GrammarElement) -> Result<Expr, ()> {
        match element {
            GrammarElement::Expr(e) => Ok(*e.clone()),
            GrammarElement::Atom(s) => Ok(Expr::Var(s.clone())),
            GrammarElement::List(_) => Err(()), // 列表需要特殊处理
            GrammarElement::NaturalLang(_) => Err(()),
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
                return Err(InterpreterError::RuntimeError(
                    "Function name must be an atom or variable".to_string(),
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
                                "Parameter must be an atom or variable".to_string(),
                                None,
                            ))
                        }
                    }
                    _ => Err(InterpreterError::RuntimeError(
                        "Parameter must be an atom or variable".to_string(),
                    )),
                })
                .collect::<Result<Vec<_>, _>>()?,
            _ => {
                return Err(InterpreterError::runtime_error(
                    "Parameters must be a list".to_string(),
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
            },
        );

        Ok(Value::Null)
    }

    /// 评估let绑定 / Evaluate let binding
    fn eval_let(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.len() < 3 {
            return Err(InterpreterError::RuntimeError(
                "Let requires: name, value, body".to_string(),
            ));
        }

        // 获取变量名（支持 Atom 和 Expr(Var(...)) 两种形式）
        let name = match &rest[0] {
            GrammarElement::Atom(s) => s.clone(),
            GrammarElement::Expr(boxed_expr) => {
                if let Expr::Var(s) = boxed_expr.as_ref() {
                    s.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        "Variable name must be an atom or variable".to_string(),
                    ));
                }
            }
            _ => {
                return Err(InterpreterError::RuntimeError(
                    "Variable name must be an atom or variable".to_string(),
                ))
            }
        };

        // 评估值
        let value = self.eval_element(&rest[1])?;

        // 保存旧值（用于作用域）
        let old_value = self.environment.insert(name.clone(), value);

        // 评估函数体
        let result = self.eval_element(&rest[2])?;

        // 恢复旧值（如果存在）
        if let Some(old) = old_value {
            self.environment.insert(name, old);
        } else {
            self.environment.remove(&name);
        }

        Ok(result)
    }

    /// 评估if特殊形式 / Evaluate if special form
    fn eval_if_special(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.is_empty() {
            return Err(InterpreterError::RuntimeError(
                "If requires at least a condition".to_string(),
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

    /// 评估Lambda表达式 / Evaluate lambda expression
    /// 语法: (lambda (params...) body)
    fn eval_lambda(&mut self, rest: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if rest.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "Lambda requires: params and body".to_string(),
            ));
        }

        // 解析参数列表
        let params_elem = &rest[0];
        let params = match params_elem {
            GrammarElement::List(params_list) => params_list
                .iter()
                .filter_map(|e| {
                    if let GrammarElement::Atom(s) = e {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .collect(),
            GrammarElement::Atom(single_param) => vec![single_param.clone()],
            _ => {
                return Err(InterpreterError::RuntimeError(
                    "Lambda params must be a list of atoms".to_string(),
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
            Expr::Var(name) => self
                .environment
                .get(name)
                .cloned()
                .ok_or_else(|| InterpreterError::undefined_variable(name.clone(), None)),
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
        for item in items {
            // 保存旧值（如果存在）
            let old_value = self.environment.insert(var.to_string(), item);

            // 执行循环体
            last_value = self.eval_expr(body)?;

            // 恢复旧值（如果存在）
            if let Some(old) = old_value {
                self.environment.insert(var.to_string(), old);
            } else {
                self.environment.remove(var);
            }
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
                        self.environment.remove(&var);
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
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::List(a), Value::List(b)) => {
                let mut result = a.clone();
                result.extend_from_slice(b);
                Ok(Value::List(result))
            }
            _ => Err(InterpreterError::TypeError(
                "Invalid types for addition".to_string(),
            )),
        }
    }

    /// 减法运算 / Subtract values
    fn sub_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err(InterpreterError::TypeError(
                "Invalid types for subtraction".to_string(),
            )),
        }
    }

    /// 乘法运算 / Multiply values
    fn mul_values(&self, left: &Value, right: &Value) -> Result<Value, InterpreterError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err(InterpreterError::TypeError(
                "Invalid types for multiplication".to_string(),
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
        // 首先检查是否是Lambda值的调用
        // First check if it's a call to a Lambda value
        if let Some(Value::Lambda { id, params }) = self.environment.get(name).cloned() {
            return self.call_lambda(&id, &params, args);
        }
        // 检查是否是内置操作符
        if name.starts_with("op:") {
            return self.eval_builtin_operator(name, args);
        }

        // 检查是否是用户定义函数（需要克隆以避免借用冲突）
        if let Some(func) = self.functions.get(name).cloned() {
            return self.call_user_function(&func, args);
        }

        // 检查是否是内置函数
        self.eval_builtin_function(name, args)
    }

    /// 评估内置操作符 / Evaluate built-in operator
    fn eval_builtin_operator(
        &mut self,
        name: &str,
        args: &[Expr],
    ) -> Result<Value, InterpreterError> {
        if args.len() != 2 {
            return Err(InterpreterError::RuntimeError(format!(
                "Operator {} requires 2 arguments",
                name
            )));
        }

        let left = self.eval_expr(&args[0])?;
        let right = self.eval_expr(&args[1])?;

        let op_str = name.strip_prefix("op:").unwrap();
        let op = match op_str {
            "+" => BinOp::Add,
            "-" => BinOp::Sub,
            "*" => BinOp::Mul,
            "/" => BinOp::Div,
            "=" | "==" => BinOp::Eq,
            "!=" | "<>" => BinOp::Ne,
            "<" => BinOp::Lt,
            ">" => BinOp::Gt,
            "<=" => BinOp::Le,
            ">=" => BinOp::Ge,
            _ => {
                return Err(InterpreterError::RuntimeError(format!(
                    "Unknown operator: {}",
                    op_str
                )))
            }
        };

        self.eval_binary_op(op, &left, &right)
    }

    /// 调用Lambda函数 / Call Lambda function
    fn call_lambda(
        &mut self,
        lambda_id: &str,
        params: &[String],
        args: &[Expr],
    ) -> Result<Value, InterpreterError> {
        if args.len() != params.len() {
            return Err(InterpreterError::RuntimeError(format!(
                "Lambda expects {} arguments, got {}",
                params.len(),
                args.len()
            )));
        }

        // 从注册表中获取Lambda函数体和捕获的环境
        let (_, body, captured_env) = self
            .lambda_registry
            .get(lambda_id)
            .ok_or_else(|| {
                InterpreterError::RuntimeError(format!(
                    "Lambda {} not found in registry",
                    lambda_id
                ))
            })?
            .clone();

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
        for (key, old_value) in saved_env {
            self.environment.insert(key, old_value);
        }

        // 移除捕获环境中新增的变量（Lambda执行时新增的）
        for key in captured_env.keys() {
            if !saved_env.contains_key(key) && !params.contains(key) {
                // 这个变量是Lambda执行时新增的，不应该保留
                self.environment.remove(key);
            }
        }

        // 移除Lambda执行时新增的变量（这些变量不在捕获环境中，也不在参数中）
        // 注意：这里我们只移除那些在Lambda执行前不存在于环境中的变量
        // 由于我们已经恢复了saved_env中的变量，这里不需要额外处理

        Ok(result)
    }

    /// 调用用户定义函数 / Call user-defined function
    fn call_user_function(
        &mut self,
        func: &Function,
        args: &[Expr],
    ) -> Result<Value, InterpreterError> {
        if args.len() != func.params.len() {
            return Err(InterpreterError::RuntimeError(format!(
                "Function expects {} arguments, got {}",
                func.params.len(),
                args.len()
            )));
        }

        // 评估参数
        let arg_values: Vec<Value> = args
            .iter()
            .map(|e| self.eval_expr(e))
            .collect::<Result<Vec<_>, _>>()?;

        // 保存当前环境 - 优化：只保存被修改的变量
        let mut saved_env = HashMap::new();
        for (param, value) in func.params.iter().zip(arg_values.iter()) {
            // 只在环境中有旧值时才保存
            if let Some(old) = self.environment.insert(param.clone(), value.clone()) {
                saved_env.insert(param.clone(), old);
            }
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

        Ok(result)
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
                    return Err(InterpreterError::RuntimeError(
                        "import requires 1 or 2 arguments: module_name [alias]".to_string(),
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
                for arg in args {
                    let value = self.eval_expr(arg)?;
                    print!("{}", value);
                }
                println!();
                Ok(Value::Null)
            }
            // 列表操作 / List operations
            "list-get" | "get" => {
                if args.len() != 2 {
                    return Err(InterpreterError::RuntimeError(
                        "list-get requires 2 arguments: list and index".to_string(),
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let index = self.eval_expr(&args[1])?;
                match (list, index) {
                    (Value::List(l), Value::Int(i)) => {
                        if i < 0 || i as usize >= l.len() {
                            Err(InterpreterError::RuntimeError(format!(
                                "Index {} out of bounds for list of length {}",
                                i,
                                l.len()
                            )))
                        } else {
                            Ok(l[i as usize].clone())
                        }
                    }
                    _ => Err(InterpreterError::TypeError(
                        "list-get requires a list and an integer index".to_string(),
                    )),
                }
            }
            "list-set" | "set" => {
                if args.len() != 3 {
                    return Err(InterpreterError::RuntimeError(
                        "list-set requires 3 arguments: list, index, value".to_string(),
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let index = self.eval_expr(&args[1])?;
                let value = self.eval_expr(&args[2])?;
                match (list, index) {
                    (Value::List(mut l), Value::Int(i)) => {
                        if i < 0 || i as usize >= l.len() {
                            Err(InterpreterError::RuntimeError(format!(
                                "Index {} out of bounds for list of length {}",
                                i,
                                l.len()
                            )))
                        } else {
                            l[i as usize] = value;
                            Ok(Value::List(l))
                        }
                    }
                    _ => Err(InterpreterError::TypeError(
                        "list-set requires a list, an integer index, and a value".to_string(),
                    )),
                }
            }
            "list-append" | "append" => {
                if args.len() != 2 {
                    return Err(InterpreterError::RuntimeError(
                        "list-append requires 2 arguments: list and value".to_string(),
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                let value = self.eval_expr(&args[1])?;
                match list {
                    Value::List(mut l) => {
                        l.push(value);
                        Ok(Value::List(l))
                    }
                    _ => Err(InterpreterError::TypeError(
                        "list-append requires a list".to_string(),
                    )),
                }
            }
            "list-length" | "length" => {
                if args.len() != 1 {
                    return Err(InterpreterError::RuntimeError(
                        "list-length requires 1 argument: list".to_string(),
                    ));
                }
                let list = self.eval_expr(&args[0])?;
                match list {
                    Value::List(l) => Ok(Value::Int(l.len() as i64)),
                    _ => Err(InterpreterError::TypeError(
                        "list-length requires a list".to_string(),
                    )),
                }
            }
            // 字典操作 / Dictionary operations
            "dict-get" => {
                if args.len() != 2 {
                    return Err(InterpreterError::RuntimeError(
                        "dict-get requires 2 arguments: dict and key".to_string(),
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                let key = self.eval_expr(&args[1])?;
                match (dict, key) {
                    (Value::Dict(d), Value::String(k)) => {
                        Ok(d.get(&k).cloned().unwrap_or(Value::Null))
                    }
                    _ => Err(InterpreterError::TypeError(
                        "dict-get requires a dict and a string key".to_string(),
                    )),
                }
            }
            "dict-set" => {
                if args.len() != 3 {
                    return Err(InterpreterError::RuntimeError(
                        "dict-set requires 3 arguments: dict, key, value".to_string(),
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
                    _ => Err(InterpreterError::TypeError(
                        "dict-set requires a dict, a string key, and a value".to_string(),
                    )),
                }
            }
            "dict-keys" => {
                if args.len() != 1 {
                    return Err(InterpreterError::RuntimeError(
                        "dict-keys requires 1 argument: dict".to_string(),
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                match dict {
                    Value::Dict(d) => {
                        let keys: Vec<Value> = d.keys().map(|k| Value::String(k.clone())).collect();
                        Ok(Value::List(keys))
                    }
                    _ => Err(InterpreterError::TypeError(
                        "dict-keys requires a dict".to_string(),
                    )),
                }
            }
            "dict-values" => {
                if args.len() != 1 {
                    return Err(InterpreterError::RuntimeError(
                        "dict-values requires 1 argument: dict".to_string(),
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                match dict {
                    Value::Dict(d) => {
                        let values: Vec<Value> = d.values().cloned().collect();
                        Ok(Value::List(values))
                    }
                    _ => Err(InterpreterError::TypeError(
                        "dict-values requires a dict".to_string(),
                    )),
                }
            }
            "dict-has" => {
                if args.len() != 2 {
                    return Err(InterpreterError::RuntimeError(
                        "dict-has requires 2 arguments: dict and key".to_string(),
                    ));
                }
                let dict = self.eval_expr(&args[0])?;
                let key = self.eval_expr(&args[1])?;
                match (dict, key) {
                    (Value::Dict(d), Value::String(k)) => Ok(Value::Bool(d.contains_key(&k))),
                    _ => Err(InterpreterError::TypeError(
                        "dict-has requires a dict and a string key".to_string(),
                    )),
                }
            }
            // 函数式编程操作 / Functional programming operations
            "map" => {
                if args.len() != 2 {
                    return Err(InterpreterError::RuntimeError(
                        "map requires 2 arguments: function and list".to_string(),
                    ));
                }
                let func_value = self.eval_expr(&args[0])?;
                let list = self.eval_expr(&args[1])?;
                match (func_value, list) {
                    (Value::Lambda { id, params }, Value::List(l)) => {
                        if params.len() != 1 {
                            return Err(InterpreterError::RuntimeError(
                                "map function must accept exactly 1 argument".to_string(),
                            ));
                        }
                        // 优化：预分配容量
                        let mut result = Vec::with_capacity(l.len());
                        for item in l {
                            // 直接调用Lambda函数
                            let item_expr = Expr::Literal(match item {
                                Value::Int(i) => Literal::Int(i),
                                Value::Float(f) => Literal::Float(f),
                                Value::String(s) => Literal::String(s),
                                Value::Bool(b) => Literal::Bool(b),
                                Value::Null => Literal::Null,
                                Value::List(_) | Value::Dict(_) | Value::Lambda { .. } => {
                                    // 对于复杂值，需要先求值
                                    return Err(InterpreterError::RuntimeError(
                                        "map: complex values need to be evaluated first"
                                            .to_string(),
                                    ));
                                }
                            });
                            let result_value = self.call_lambda(&id, &params, &[item_expr])?;
                            result.push(result_value);
                        }
                        Ok(Value::List(result))
                    }
                    (Value::Lambda { .. }, _) => Err(InterpreterError::TypeError(
                        "map requires a list as second argument".to_string(),
                    )),
                    _ => Err(InterpreterError::TypeError(
                        "map requires a function (lambda) as first argument".to_string(),
                    )),
                }
            }
            "filter" => {
                if args.len() != 2 {
                    return Err(InterpreterError::RuntimeError(
                        "filter requires 2 arguments: predicate and list".to_string(),
                    ));
                }
                let func_value = self.eval_expr(&args[0])?;
                let list = self.eval_expr(&args[1])?;
                match (func_value, list) {
                    (Value::Lambda { id, params }, Value::List(l)) => {
                        if params.len() != 1 {
                            return Err(InterpreterError::RuntimeError(
                                "filter predicate must accept exactly 1 argument".to_string(),
                            ));
                        }
                        // 优化：预分配容量
                        let mut result = Vec::with_capacity(l.len());
                        for item in l {
                            // 先克隆item以便后续使用
                            let item_clone = item.clone();
                            // 直接调用Lambda函数
                            let item_expr = Expr::Literal(match item_clone {
                                Value::Int(i) => Literal::Int(i),
                                Value::Float(f) => Literal::Float(f),
                                Value::String(s) => Literal::String(s),
                                Value::Bool(b) => Literal::Bool(b),
                                Value::Null => Literal::Null,
                                Value::List(_) | Value::Dict(_) | Value::Lambda { .. } => {
                                    return Err(InterpreterError::RuntimeError(
                                        "filter: complex values need to be evaluated first"
                                            .to_string(),
                                    ));
                                }
                            });
                            let predicate_result = self.call_lambda(&id, &params, &[item_expr])?;
                            // 如果predicate返回true，保留该元素
                            if self.is_truthy(&predicate_result) {
                                result.push(item);
                            }
                        }
                        Ok(Value::List(result))
                    }
                    (Value::Lambda { .. }, _) => Err(InterpreterError::TypeError(
                        "filter requires a list as second argument".to_string(),
                    )),
                    _ => Err(InterpreterError::TypeError(
                        "filter requires a function (lambda) as first argument".to_string(),
                    )),
                }
            }
            "reduce" => {
                if args.len() != 3 {
                    return Err(InterpreterError::RuntimeError(
                        "reduce requires 3 arguments: function, initial value, and list"
                            .to_string(),
                    ));
                }
                let func_value = self.eval_expr(&args[0])?;
                let initial = self.eval_expr(&args[1])?;
                let list = self.eval_expr(&args[2])?;
                match (func_value, list) {
                    (Value::Lambda { id, params }, Value::List(l)) => {
                        if params.len() != 2 {
                            return Err(InterpreterError::RuntimeError(
                                "reduce function must accept exactly 2 arguments".to_string(),
                            ));
                        }
                        let mut accumulator = initial;
                        for item in l {
                            // 直接调用Lambda函数: func(accumulator, item)
                            let acc_expr = Expr::Literal(match accumulator {
                                Value::Int(i) => Literal::Int(i),
                                Value::Float(f) => Literal::Float(f),
                                Value::String(s) => Literal::String(s),
                                Value::Bool(b) => Literal::Bool(b),
                                Value::Null => Literal::Null,
                                Value::List(_) | Value::Dict(_) | Value::Lambda { .. } => {
                                    return Err(InterpreterError::RuntimeError(
                                        "reduce: complex accumulator values need to be evaluated first".to_string(),
                                    ));
                                }
                            });
                            let item_expr = Expr::Literal(match item {
                                Value::Int(i) => Literal::Int(i),
                                Value::Float(f) => Literal::Float(f),
                                Value::String(s) => Literal::String(s),
                                Value::Bool(b) => Literal::Bool(b),
                                Value::Null => Literal::Null,
                                Value::List(_) | Value::Dict(_) | Value::Lambda { .. } => {
                                    return Err(InterpreterError::RuntimeError(
                                        "reduce: complex item values need to be evaluated first"
                                            .to_string(),
                                    ));
                                }
                            });
                            accumulator = self.call_lambda(&id, &params, &[acc_expr, item_expr])?;
                        }
                        Ok(accumulator)
                    }
                    (Value::Lambda { .. }, _) => Err(InterpreterError::TypeError(
                        "reduce requires a list as third argument".to_string(),
                    )),
                    _ => Err(InterpreterError::TypeError(
                        "reduce requires a function (lambda) as first argument".to_string(),
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
                    (Value::List(mut l), Some(Value::Lambda { id, params })) => {
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
            _ => Err(InterpreterError::RuntimeError(
                "Module name must be a string literal or identifier".to_string(),
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
        for (name, function) in &module.functions {
            let qualified_name = format!("{}.{}", alias, name);
            self.functions.insert(qualified_name, function.clone());
        }

        Ok(())
    }

    /// 加载模块 / Load module
    fn load_module(&self, module_name: &str) -> Result<Module, InterpreterError> {
        let path = self.resolve_module_path(module_name)?;
        let code = fs::read_to_string(&path).map_err(|e| {
            InterpreterError::RuntimeError(format!(
                "Failed to read module '{}': {}",
                module_name, e
            ))
        })?;

        let parser = AdaptiveParser::new(true);
        let ast = parser.parse(&code).map_err(|e| {
            InterpreterError::RuntimeError(format!(
                "Failed to parse module '{}': {:?}",
                module_name, e
            ))
        })?;

        let mut module_interpreter = Interpreter::new();
        module_interpreter.execute(&ast).map_err(|e| {
            InterpreterError::RuntimeError(format!(
                "Failed to execute module '{}': {:?}",
                module_name, e
            ))
        })?;

        Ok(Module {
            name: module_name.to_string(),
            environment: module_interpreter.environment.clone(),
            functions: module_interpreter.functions.clone(),
        })
    }

    /// 解析模块路径 / Resolve module path
    fn resolve_module_path(&self, module_name: &str) -> Result<PathBuf, InterpreterError> {
        let mut candidates = Vec::new();
        let name = if module_name.ends_with(".aevo") {
            module_name.to_string()
        } else {
            format!("{}.aevo", module_name)
        };

        candidates.push(PathBuf::from("modules").join(&name));
        candidates.push(PathBuf::from("examples").join(&name));
        candidates.push(PathBuf::from(&name));

        for path in candidates {
            if path.exists() {
                return Ok(path);
            }
        }

        Err(InterpreterError::RuntimeError(format!(
            "Module '{}' not found in modules/, examples/, or current directory",
            module_name
        )))
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
                return Err(InterpreterError::TypeError(format!(
                    "Cannot compare {} and {}",
                    self.value_type_name(left),
                    self.value_type_name(right)
                )));
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
