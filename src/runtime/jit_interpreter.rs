// JIT解释器 / JIT Interpreter
// 集成JIT编译器的解释器
// Interpreter integrated with JIT compiler

use crate::grammar::core::GrammarElement;
use crate::runtime::interpreter::{Interpreter, Value, InterpreterError};
use crate::runtime::jit::{JITCompiler, JITStatistics};
use std::time::{Duration, Instant};

/// JIT解释器 / JIT Interpreter
/// 集成JIT编译器的解释器，自动检测和优化热点代码
/// Interpreter integrated with JIT compiler, automatically detects and optimizes hot spot code
pub struct JITInterpreter {
    /// 基础解释器 / Base interpreter
    interpreter: Interpreter,
    /// JIT编译器 / JIT compiler
    jit_compiler: JITCompiler,
    /// 是否启用JIT / Whether JIT is enabled
    jit_enabled: bool,
}

impl JITInterpreter {
    /// 创建新JIT解释器 / Create new JIT interpreter
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            jit_compiler: JITCompiler::new(),
            jit_enabled: true,
        }
    }

    /// 创建带自定义JIT阈值的解释器 / Create interpreter with custom JIT threshold
    pub fn with_threshold(threshold: usize) -> Self {
        Self {
            interpreter: Interpreter::new(),
            jit_compiler: JITCompiler::with_threshold(threshold),
            jit_enabled: true,
        }
    }

    /// 启用/禁用JIT / Enable/disable JIT
    pub fn set_jit_enabled(&mut self, enabled: bool) {
        self.jit_enabled = enabled;
        self.jit_compiler.set_enabled(enabled);
    }

    /// 执行代码（带JIT优化） / Execute code (with JIT optimization)
    pub fn execute(&mut self, ast: &[GrammarElement]) -> Result<Value, InterpreterError> {
        if !self.jit_enabled {
            return self.interpreter.execute(ast);
        }

        // 生成代码键 / Generate code key
        let code_key = JITCompiler::generate_code_key(ast);

        // 检查是否是热点代码 / Check if hot spot
        if self.jit_compiler.is_hot_spot(&code_key) {
            // 检查是否已编译 / Check if already compiled
            if self.jit_compiler.get_compiled_code(&code_key).is_some() {
                // 执行编译后的代码 / Execute compiled code
                return self.jit_compiler.execute_compiled(&code_key, &mut self.interpreter);
            } else {
                // 编译热点代码 / Compile hot spot code
                if let Err(e) = self.jit_compiler.compile_hot_spot(&code_key, ast) {
                    // 编译失败，回退到解释执行 / Compilation failed, fall back to interpretation
                    eprintln!("JIT compilation failed: {:?}, falling back to interpretation", e);
                }
            }
        }

        // 记录执行时间 / Record execution time
        let start = Instant::now();
        let result = self.interpreter.execute(ast)?;
        let execution_time = start.elapsed();

        // 记录执行统计 / Record execution statistics
        self.jit_compiler.record_execution(&code_key, execution_time);

        // 如果达到阈值，编译为热点代码 / If threshold reached, compile as hot spot
        if self.jit_compiler.is_hot_spot(&code_key) {
            if self.jit_compiler.get_compiled_code(&code_key).is_none() {
                if let Err(e) = self.jit_compiler.compile_hot_spot(&code_key, ast) {
                    eprintln!("JIT compilation failed: {:?}", e);
                }
            }
        }

        Ok(result)
    }

    /// 执行代码（不记录统计） / Execute code (without recording statistics)
    pub fn execute_without_profiling(&mut self, ast: &[GrammarElement]) -> Result<Value, InterpreterError> {
        self.interpreter.execute(ast)
    }

    /// 获取JIT统计信息 / Get JIT statistics
    pub fn get_jit_statistics(&self) -> JITStatistics {
        self.jit_compiler.get_statistics()
    }

    /// 获取热点代码列表 / Get hot spot code list
    pub fn get_hot_spots(&self) -> Vec<String> {
        self.jit_compiler.get_hot_spots()
    }

    /// 清除JIT缓存 / Clear JIT cache
    pub fn clear_jit_cache(&mut self) {
        self.jit_compiler.clear_cache();
    }

    /// 获取基础解释器引用（用于高级用法） / Get base interpreter reference (for advanced usage)
    pub fn interpreter_mut(&mut self) -> &mut Interpreter {
        &mut self.interpreter
    }

    /// 获取基础解释器引用（只读） / Get base interpreter reference (read-only)
    pub fn interpreter(&self) -> &Interpreter {
        &self.interpreter
    }
}

impl Default for JITInterpreter {
    fn default() -> Self {
        Self::new()
    }
}
