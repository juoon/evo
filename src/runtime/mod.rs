//! # Runtime Module / 运行时模块
//!
//! 提供解释器、JIT编译器、执行模式选择等功能
//! Provides interpreter, JIT compiler, execution mode selection, etc.
//!
//! ## 快速导航 / Quick Navigation
//!
//! - `interpreter.rs` - **解释器核心** - 主入口: `Interpreter::new()`, `execute()`
//! - `jit.rs` - **JIT编译器** - 热点检测、常量折叠: `JITCompiler::compile()`
//! - `jit_interpreter.rs` - **JIT解释器** - 整合解释器和JIT编译器
//! - `mode.rs` - **执行模式选择** - 解释模式 vs JIT模式切换
//!
//! ## 数据流 / Data Flow
//! ```
//! AST (Vec<GrammarElement>)
//!   ↓
//! Interpreter::execute() [interpreter.rs]
//!   ↓
//! 热点检测 [jit.rs]
//!   ↓ (可选)
//! JIT编译优化
//!   ↓
//! Value (运行时值)
//! ```

pub mod interpreter;
pub mod jit;
pub mod jit_interpreter;
pub mod mode;

pub use interpreter::*;
pub use jit::*;
pub use jit_interpreter::*;
pub use mode::*;
