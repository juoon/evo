// 语法定义模块 / Grammar definition module
// 该模块提供语法规则的定义、存储和扩展功能
// This module provides grammar rule definition, storage and extension capabilities

pub mod core;
pub mod rule;
pub mod self_desc;

pub use core::*;
pub use rule::*;
pub use self_desc::*;

