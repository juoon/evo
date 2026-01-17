// 进化引擎模块 / Evolution engine module
// 提供语法进化、语义适应、进化记录等功能
// Provides syntax evolution, semantic adaptation, evolution tracking, etc.

pub mod analyzer;
pub mod code_generator;
pub mod code_reviewer;
pub mod dependency;
pub mod doc_generator;
pub mod engine;
pub mod error_recovery;
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
pub use knowledge::*;
pub use learning::*;
pub use optimizer::*;
pub use performance::*;
pub use quality_assessor::*;
pub use similarity::*;
pub use test_generator::*;
pub use tracker::*;
