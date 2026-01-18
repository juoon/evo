//! # Evolution Engine Module / 进化引擎模块
//!
//! 提供语法进化、语义适应、进化记录等功能
//! Provides syntax evolution, semantic adaptation, evolution tracking, etc.
//!
//! ## 快速导航 / Quick Navigation
//!
//! ### 核心组件 (Core Components)
//! - `engine.rs` - **引擎核心** - 主入口: `EvolutionEngine::new()`, `start_evolution()`, `self_evolve()`
//! - `knowledge.rs` - **知识图谱** - 实体提取、关系挖掘、模式发现
//! - `tracker.rs` - **进化历史追踪** - 事件记录、谱系树、回滚机制
//!
//! ### 分析工具 (Analysis Tools)
//! - `analyzer.rs` - **代码分析器** - 模式识别、复杂度分析
//! - `learning.rs` - **使用模式学习** - 使用频率跟踪、错误模式学习
//! - `similarity.rs` - **相似度检测** - 代码重复检测、重构建议
//! - `dependency.rs` - **依赖分析** - 依赖图构建、循环依赖检测
//!
//! ### 代码生成和优化 (Code Generation & Optimization)
//! - `code_generator.rs` - **智能代码生成** - 基于意图生成代码、代码补全
//! - `optimizer.rs` - **优化建议器** - 优化策略、效果预测
//! - `error_recovery.rs` - **错误恢复** - 自动修复常见错误
//!
//! ### 质量评估 (Quality Assessment)
//! - `quality_assessor.rs` - **代码质量评估** - 多维度评估、改进建议
//! - `code_reviewer.rs` - **代码审查** - 自动问题检测、审查报告
//! - `performance.rs` - **性能分析** - 性能瓶颈识别、优化建议
//! - `doc_generator.rs` - **文档生成** - 自动生成代码文档
//! - `test_generator.rs` - **测试生成** - 自动生成测试用例
//!
//! ## 依赖关系 / Dependencies
//! ```
//! engine.rs (核心协调)
//!   ├─> knowledge.rs (知识图谱)
//!   ├─> tracker.rs (历史追踪)
//!   ├─> analyzer.rs (代码分析)
//!   └─> [其他工具模块]
//! ```

pub mod analyzer;
pub mod code_generator;
pub mod code_reviewer;
pub mod dependency;
pub mod doc_generator;
pub mod engine;
pub mod error_recovery;
pub mod event_manager;
pub mod knowledge;
pub mod learning;
pub mod optimizer;
pub mod performance;
pub mod quality_assessor;
pub mod similarity;
pub mod test_generator;
pub mod tracker;

pub use analyzer::*;
pub use code_generator::*;
pub use code_reviewer::*;
pub use dependency::*;
pub use doc_generator::*;
pub use engine::*;
pub use error_recovery::*;
pub use event_manager::*;
pub use knowledge::*;
pub use learning::*;
pub use optimizer::*;
pub use performance::*;
pub use quality_assessor::*;
pub use similarity::*;
pub use test_generator::*;
pub use tracker::*;
