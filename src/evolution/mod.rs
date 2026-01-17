// 进化引擎模块 / Evolution engine module
// 提供语法进化、语义适应、进化记录等功能
// Provides syntax evolution, semantic adaptation, evolution tracking, etc.

pub mod analyzer;
pub mod engine;
pub mod knowledge;
pub mod learning;
pub mod tracker;

pub use analyzer::*;
pub use engine::*;
pub use knowledge::*;
pub use learning::*;
pub use tracker::*;

