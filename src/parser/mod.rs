//! # Parser Module / 解析器模块
//!
//! 提供自适应解析、自然语言解析等功能
//! Provides adaptive parsing, natural language parsing, etc.
//!
//! ## 快速导航 / Quick Navigation
//!
//! - `adaptive.rs` - **自适应解析器主逻辑** - 主入口: `AdaptiveParser::new()`, `parse()`
//! - `nlu.rs` - **自然语言理解** - 意图识别、中英文转代码: `NLU::parse_intent()`
//! - `context.rs` - **上下文管理** - 多轮对话、变量引用解析: `ContextManager`
//! - `explainer.rs` - **代码解释器** - 代码转自然语言、中英文双语解释
//!
//! ## 数据流 / Data Flow
//! ```
//! 源代码/自然语言
//!   ↓
//! AdaptiveParser::parse() [adaptive.rs]
//!   ↓
//! AST (Vec<GrammarElement>)
//! ```

pub mod adaptive;
pub mod context;
pub mod explainer;
pub mod nlu;

pub use adaptive::*;
pub use context::*;
pub use explainer::*;
pub use nlu::*;
