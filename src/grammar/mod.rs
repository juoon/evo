//! # Grammar Module / 语法定义模块
//!
//! 该模块提供语法规则的定义、存储和扩展功能
//! This module provides grammar rule definition, storage and extension capabilities
//!
//! ## 快速导航 / Quick Navigation
//!
//! - `core.rs` - **核心语法定义** - AST节点类型 (`GrammarElement`)、数据类型 (`Value`)
//! - `rule.rs` - **语法规则系统** - 规则定义 (`GrammarRule`)、规则匹配和应用
//! - `self_desc.rs` - **自描述语法机制** - 用语言自身描述语法规则
//!
//! ## 关键类型 / Key Types
//!
//! - `GrammarElement` - AST节点（在 `core.rs` 定义）
//! - `Value` - 运行时值类型（在 `core.rs` 定义）
//! - `GrammarRule` - 语法规则（在 `rule.rs` 定义）

pub mod core;
pub mod rule;
pub mod self_desc;

pub use core::*;
pub use rule::*;
pub use self_desc::*;
