// 语法规则定义 / Grammar rule definition
// 定义可扩展的语法规则系统
// Defines extensible grammar rule system

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::grammar::core::GrammarElement;

/// 语法规则 / Grammar rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarRule {
    /// 规则唯一标识 / Rule unique identifier
    pub id: Uuid,
    /// 规则名称 / Rule name
    pub name: String,
    /// 模式定义 / Pattern definition
    pub pattern: Pattern,
    /// 产生式 / Production rule
    pub production: Production,
    /// 元数据 / Metadata
    pub meta: RuleMetadata,
    /// 创建时间 / Creation time
    pub created_at: DateTime<Utc>,
    /// 更新时间 / Update time
    pub updated_at: DateTime<Utc>,
}

/// 模式定义 / Pattern definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// 模式元素列表 / List of pattern elements
    pub elements: Vec<PatternElement>,
    /// 是否允许可变参数 / Whether variable arguments are allowed
    pub variadic: bool,
}

/// 模式元素 / Pattern element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternElement {
    /// 关键字匹配 / Keyword match
    Keyword(String),
    /// 标识符匹配 / Identifier match
    Identifier(String),
    /// 字面量匹配 / Literal match
    Literal(GrammarElement),
    /// 通配符匹配 / Wildcard match
    Wildcard(String),
    /// 可选元素 / Optional element
    Optional(Box<PatternElement>),
    /// 重复元素 / Repeated element
    Repeat(Box<PatternElement>),
    /// 分组 / Group
    Group(Vec<PatternElement>),
    /// 自然语言模式 / Natural language pattern
    NaturalLang(String),
}

/// 产生式定义 / Production definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Production {
    /// 目标语法结构 / Target grammar structure
    pub target: GrammarElement,
    /// 转换规则 / Transformation rules
    pub transform: Vec<TransformRule>,
    /// 条件检查 / Condition checks
    pub conditions: Vec<Condition>,
}

/// 转换规则 / Transform rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformRule {
    /// 源模式 / Source pattern
    pub from: PatternElement,
    /// 目标结构 / Target structure
    pub to: GrammarElement,
}

/// 条件检查 / Condition check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// 类型检查 / Type check
    TypeCheck(String, String),
    /// 值检查 / Value check
    ValueCheck(String, String),
    /// 上下文检查 / Context check
    ContextCheck(String),
}

/// 规则元数据 / Rule metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMetadata {
    /// 版本号 / Version number
    pub version: String,
    /// 定义方式 / Definition method
    pub defined_by: DefinitionMethod,
    /// 稳定性 / Stability
    pub stability: Stability,
    /// 描述 / Description
    pub description: String,
    /// 示例 / Examples
    pub examples: Vec<String>,
    /// 自然语言同义词 / Natural language synonyms
    pub natural_lang_synonyms: Vec<String>,
}

/// 定义方式 / Definition method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DefinitionMethod {
    /// 外部语言定义 / External language definition
    External(String),
    /// 自描述定义 / Self-describing definition
    SelfDescribing,
    /// 进化定义 / Evolutionary definition
    Evolutionary,
    /// 混合定义 / Hybrid definition
    Hybrid(Vec<DefinitionMethod>),
}

/// 稳定性级别 / Stability level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stability {
    /// 实验性 / Experimental
    Experimental,
    /// 稳定中 / Stabilizing,
    Stabilizing,
    /// 稳定 / Stable
    Stable,
    /// 已弃用 / Deprecated
    Deprecated,
}

impl GrammarRule {
    /// 创建新规则 / Create new rule
    pub fn new(
        name: String,
        pattern: Pattern,
        production: Production,
        meta: RuleMetadata,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            pattern,
            production,
            meta,
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新规则 / Update rule
    pub fn update(&mut self, pattern: Pattern, production: Production, meta: RuleMetadata) {
        self.pattern = pattern;
        self.production = production;
        self.meta = meta;
        self.updated_at = Utc::now();
    }

    /// 检查模式匹配 / Check pattern matching
    pub fn matches(&self, input: &[GrammarElement]) -> bool {
        // TODO: 实现模式匹配逻辑 / Implement pattern matching logic
        false
    }
}

