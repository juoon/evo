// JIT编译器 / JIT Compiler
// 实现热点代码优化和即时编译
// Implements hot spot code optimization and just-in-time compilation

use crate::grammar::core::{Expr, GrammarElement};
use crate::runtime::interpreter::{Interpreter, Value, InterpreterError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// JIT编译器 / JIT Compiler
pub struct JITCompiler {
    /// 热点代码缓存 / Hot spot code cache
    hot_spots: HashMap<String, CompiledCode>,
    /// 执行计数器 / Execution counter
    execution_counts: HashMap<String, ExecutionStats>,
    /// 编译阈值 / Compilation threshold
    compilation_threshold: usize,
    /// 是否启用JIT / Whether JIT is enabled
    enabled: bool,
}

/// 编译后的代码 / Compiled code
#[derive(Debug, Clone)]
pub struct CompiledCode {
    /// 原始AST / Original AST
    ast: Vec<GrammarElement>,
    /// 优化后的表达式 / Optimized expression
    optimized_expr: Option<Expr>,
    /// 编译时间戳（秒） / Compilation timestamp (seconds)
    compiled_at_timestamp: u64,
    /// 执行次数 / Execution count
    execution_count: usize,
}

/// 执行统计 / Execution statistics
#[derive(Debug, Clone)]
struct ExecutionStats {
    /// 执行次数 / Execution count
    count: usize,
    /// 总执行时间（微秒） / Total execution time (microseconds)
    total_time_micros: u64,
    /// 平均执行时间（微秒） / Average execution time (microseconds)
    avg_time_micros: u64,
    /// 最后执行时间戳（秒） / Last execution timestamp (seconds)
    last_execution_timestamp: u64,
}

impl JITCompiler {
    /// 创建新JIT编译器 / Create new JIT compiler
    pub fn new() -> Self {
        Self {
            hot_spots: HashMap::new(),
            execution_counts: HashMap::new(),
            compilation_threshold: 10, // 默认阈值：执行10次后编译 / Default threshold: compile after 10 executions
            enabled: true,
        }
    }

    /// 创建带自定义阈值的JIT编译器 / Create JIT compiler with custom threshold
    pub fn with_threshold(threshold: usize) -> Self {
        Self {
            hot_spots: HashMap::new(),
            execution_counts: HashMap::new(),
            compilation_threshold: threshold,
            enabled: true,
        }
    }

    /// 启用/禁用JIT / Enable/disable JIT
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// 检查是否是热点代码 / Check if code is hot spot
    pub fn is_hot_spot(&self, code_key: &str) -> bool {
        if !self.enabled {
            return false;
        }
        
        self.execution_counts
            .get(code_key)
            .map(|stats| stats.count >= self.compilation_threshold)
            .unwrap_or(false)
    }

    /// 记录代码执行 / Record code execution
    pub fn record_execution(&mut self, code_key: &str, execution_time: Duration) {
        if !self.enabled {
            return;
        }

        let stats = self.execution_counts.entry(code_key.to_string()).or_insert_with(|| {
            ExecutionStats {
                count: 0,
                total_time_micros: 0,
                avg_time_micros: 0,
                last_execution_timestamp: 0,
            }
        });

        stats.count += 1;
        stats.total_time_micros += execution_time.as_micros() as u64;
        stats.avg_time_micros = stats.total_time_micros / stats.count as u64;
        stats.last_execution_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// 获取热点代码列表 / Get hot spot code list
    pub fn get_hot_spots(&self) -> Vec<String> {
        self.execution_counts
            .iter()
            .filter(|(_, stats)| stats.count >= self.compilation_threshold)
            .map(|(key, _)| key.clone())
            .collect()
    }

    /// 编译热点代码 / Compile hot spot code
    pub fn compile_hot_spot(&mut self, code_key: &str, ast: &[GrammarElement]) -> Result<(), InterpreterError> {
        if !self.enabled {
            return Ok(());
        }

        // 优化代码 / Optimize code
        let optimized = self.optimize_code(ast)?;

        // 缓存编译后的代码 / Cache compiled code
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.hot_spots.insert(
            code_key.to_string(),
            CompiledCode {
                ast: ast.to_vec(),
                optimized_expr: optimized,
                compiled_at_timestamp: timestamp,
                execution_count: 0,
            },
        );

        Ok(())
    }

    /// 优化代码 / Optimize code
    fn optimize_code(&self, ast: &[GrammarElement]) -> Result<Option<Expr>, InterpreterError> {
        // 简化实现：提取第一个表达式进行优化
        // Simplified implementation: extract first expression for optimization
        if let Some(GrammarElement::Expr(expr)) = ast.first() {
            let optimized = self.optimize_expr(expr)?;
            Ok(Some(optimized))
        } else {
            Ok(None)
        }
    }

    /// 优化表达式 / Optimize expression
    fn optimize_expr(&self, expr: &Expr) -> Result<Expr, InterpreterError> {
        match expr {
            // 常量折叠 / Constant folding
            Expr::Binary(op, left, right) => {
                // 如果左右都是字面量，直接计算
                // If both left and right are literals, calculate directly
                if let (Expr::Literal(left_lit), Expr::Literal(right_lit)) = (left.as_ref(), right.as_ref()) {
                    let result = self.eval_binary_literal(op, left_lit, right_lit)?;
                    Ok(Expr::Literal(result))
                } else {
                    // 递归优化子表达式 / Recursively optimize sub-expressions
                    let opt_left = Box::new(self.optimize_expr(left)?);
                    let opt_right = Box::new(self.optimize_expr(right)?);
                    Ok(Expr::Binary(*op, opt_left, opt_right))
                }
            }
            // 其他表达式保持不变或递归优化 / Other expressions remain unchanged or recursively optimized
            Expr::Call(name, args) => {
                let opt_args: Result<Vec<Expr>, InterpreterError> = args
                    .iter()
                    .map(|arg| self.optimize_expr(arg))
                    .collect();
                Ok(Expr::Call(name.clone(), opt_args?))
            }
            Expr::If(cond, then_expr, else_expr) => {
                let opt_cond = Box::new(self.optimize_expr(cond)?);
                let opt_then = Box::new(self.optimize_expr(then_expr)?);
                let opt_else = Box::new(self.optimize_expr(else_expr)?);
                Ok(Expr::If(opt_cond, opt_then, opt_else))
            }
            // 字面量和变量保持不变 / Literals and variables remain unchanged
            Expr::Literal(_) | Expr::Var(_) => Ok(expr.clone()),
        }
    }

    /// 评估二元运算字面量 / Evaluate binary operation literals
    fn eval_binary_literal(
        &self,
        op: &crate::grammar::core::BinOp,
        left: &crate::grammar::core::Literal,
        right: &crate::grammar::core::Literal,
    ) -> Result<crate::grammar::core::Literal, InterpreterError> {
        use crate::grammar::core::{BinOp, Literal};

        match (op, left, right) {
            // 算术运算 / Arithmetic operations
            (BinOp::Add, Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a + b)),
            (BinOp::Add, Literal::Float(a), Literal::Float(b)) => Ok(Literal::Float(a + b)),
            (BinOp::Sub, Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a - b)),
            (BinOp::Sub, Literal::Float(a), Literal::Float(b)) => Ok(Literal::Float(a - b)),
            (BinOp::Mul, Literal::Int(a), Literal::Int(b)) => Ok(Literal::Int(a * b)),
            (BinOp::Mul, Literal::Float(a), Literal::Float(b)) => Ok(Literal::Float(a * b)),
            (BinOp::Div, Literal::Int(a), Literal::Int(b)) => {
                if *b == 0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                Ok(Literal::Int(a / b))
            }
            (BinOp::Div, Literal::Float(a), Literal::Float(b)) => {
                if *b == 0.0 {
                    return Err(InterpreterError::DivisionByZero);
                }
                Ok(Literal::Float(a / b))
            }
            // 比较运算 / Comparison operations
            (BinOp::Eq, left, right) => {
                // 列表和字典不能进行常量折叠比较
                if matches!(left, Literal::List(_) | Literal::Dict(_)) 
                    || matches!(right, Literal::List(_) | Literal::Dict(_)) {
                    return Err(InterpreterError::TypeError(
                        "Cannot fold comparison with list or dict literals".to_string(),
                    ));
                }
                Ok(Literal::Bool(left == right))
            }
            (BinOp::Ne, left, right) => {
                // 列表和字典不能进行常量折叠比较
                if matches!(left, Literal::List(_) | Literal::Dict(_)) 
                    || matches!(right, Literal::List(_) | Literal::Dict(_)) {
                    return Err(InterpreterError::TypeError(
                        "Cannot fold comparison with list or dict literals".to_string(),
                    ));
                }
                Ok(Literal::Bool(left != right))
            }
            (BinOp::Lt, Literal::Int(a), Literal::Int(b)) => Ok(Literal::Bool(a < b)),
            (BinOp::Lt, Literal::Float(a), Literal::Float(b)) => Ok(Literal::Bool(a < b)),
            (BinOp::Gt, Literal::Int(a), Literal::Int(b)) => Ok(Literal::Bool(a > b)),
            (BinOp::Gt, Literal::Float(a), Literal::Float(b)) => Ok(Literal::Bool(a > b)),
            (BinOp::Le, Literal::Int(a), Literal::Int(b)) => Ok(Literal::Bool(a <= b)),
            (BinOp::Le, Literal::Float(a), Literal::Float(b)) => Ok(Literal::Bool(a <= b)),
            (BinOp::Ge, Literal::Int(a), Literal::Int(b)) => Ok(Literal::Bool(a >= b)),
            (BinOp::Ge, Literal::Float(a), Literal::Float(b)) => Ok(Literal::Bool(a >= b)),
            // 列表和字典不支持常量折叠的算术运算
            (_, Literal::List(_), _) | (_, _, Literal::List(_)) 
            | (_, Literal::Dict(_), _) | (_, _, Literal::Dict(_)) => {
                Err(InterpreterError::TypeError(
                    "Cannot fold binary operation with list or dict literals".to_string(),
                ))
            }
            _ => Err(InterpreterError::TypeError(
                "Invalid types for binary operation".to_string(),
            )),
        }
    }

    /// 获取编译后的代码 / Get compiled code
    pub fn get_compiled_code(&self, code_key: &str) -> Option<&CompiledCode> {
        self.hot_spots.get(code_key)
    }

    /// 执行编译后的代码 / Execute compiled code
    pub fn execute_compiled(
        &mut self,
        code_key: &str,
        interpreter: &mut Interpreter,
    ) -> Result<Value, InterpreterError> {
        if let Some(compiled) = self.hot_spots.get_mut(code_key) {
            compiled.execution_count += 1;
            
            // 如果有优化后的表达式，使用它 / If optimized expression exists, use it
            if let Some(ref opt_expr) = compiled.optimized_expr {
                interpreter.execute_expr(opt_expr)
            } else {
                // 否则使用原始AST / Otherwise use original AST
                interpreter.execute(&compiled.ast)
            }
        } else {
            Err(InterpreterError::RuntimeError(
                "Compiled code not found".to_string(),
            ))
        }
    }

    /// 生成代码键 / Generate code key
    pub fn generate_code_key(ast: &[GrammarElement]) -> String {
        // 简化实现：使用AST的字符串表示作为键
        // Simplified implementation: use AST string representation as key
        format!("{:?}", ast)
    }

    /// 清除缓存 / Clear cache
    pub fn clear_cache(&mut self) {
        self.hot_spots.clear();
        self.execution_counts.clear();
    }

    /// 获取统计信息 / Get statistics
    pub fn get_statistics(&self) -> JITStatistics {
        let total_hot_spots = self.hot_spots.len();
        let total_executions: usize = self.execution_counts.values().map(|s| s.count).sum();
        let compiled_count: usize = self.hot_spots.values().map(|c| c.execution_count).sum();

        JITStatistics {
            total_hot_spots,
            total_executions,
            compiled_count,
            compilation_threshold: self.compilation_threshold,
            enabled: self.enabled,
        }
    }
}

impl Default for JITCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// JIT统计信息 / JIT statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JITStatistics {
    /// 热点代码数量 / Number of hot spots
    pub total_hot_spots: usize,
    /// 总执行次数 / Total execution count
    pub total_executions: usize,
    /// 编译后执行次数 / Compiled execution count
    pub compiled_count: usize,
    /// 编译阈值 / Compilation threshold
    pub compilation_threshold: usize,
    /// 是否启用 / Whether enabled
    pub enabled: bool,
}
