// 自描述语法 / Self-describing syntax
// 允许语言用自身描述自身的语法规则
// Allows the language to describe its own grammar rules using itself

use serde::{Deserialize, Serialize};
use crate::grammar::rule::{GrammarRule, Pattern, Production, RuleMetadata, DefinitionMethod, Stability};

/// 自描述语法规则 / Self-describing syntax rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfDescribingRule {
    /// 基础规则 / Base rule
    pub rule: GrammarRule,
    /// 自描述代码 / Self-describing code
    pub self_describing_code: String,
    /// 递归定义引用 / Recursive definition reference
    pub recursive_reference: Option<Box<SelfDescribingRule>>,
}

impl SelfDescribingRule {
    /// 创建自描述规则 / Create self-describing rule
    pub fn new(rule: GrammarRule, self_describing_code: String) -> Self {
        Self {
            rule,
            self_describing_code,
            recursive_reference: None,
        }
    }

    /// 创建递归自描述规则 / Create recursive self-describing rule
    pub fn recursive(rule: GrammarRule, self_describing_code: String, reference: SelfDescribingRule) -> Self {
        Self {
            rule,
            self_describing_code,
            recursive_reference: Some(Box::new(reference)),
        }
    }
}

/// 语法定义语法规则（元规则）/ Grammar definition syntax rule (meta-rule)
/// 这是最基础的元规则，允许用Aevolang定义语法规则
/// This is the most fundamental meta-rule that allows defining grammar rules in Aevolang
pub fn syntax_definition_rule() -> SelfDescribingRule {
    let pattern = Pattern {
        elements: vec![
            // 语法定义的模式: "语法" 标识符 ":" 模式列表 "=>" 产生式
            // Pattern for syntax definition: "syntax" identifier ":" pattern_list "=>" production
            crate::grammar::rule::PatternElement::Keyword("语法".to_string()),
            crate::grammar::rule::PatternElement::Identifier("name".to_string()),
            crate::grammar::rule::PatternElement::Keyword(":".to_string()),
            crate::grammar::rule::PatternElement::Wildcard("pattern_list".to_string()),
            crate::grammar::rule::PatternElement::Keyword("=>".to_string()),
            crate::grammar::rule::PatternElement::Wildcard("production".to_string()),
        ],
        variadic: false,
    };

    let production = Production {
        target: crate::grammar::core::GrammarElement::Atom("GrammarRule".to_string()),
        transform: vec![],
        conditions: vec![],
    };

    let meta = RuleMetadata {
        version: "1.0".to_string(),
        defined_by: DefinitionMethod::SelfDescribing,
        stability: Stability::Stable,
        description: "允许定义语法规则的元规则 / Meta-rule that allows defining grammar rules".to_string(),
        examples: vec![
            "语法 变量声明: \"让\" 标识符 \"=\" 表达式 => VariableDeclaration(标识符, 表达式)".to_string(),
        ],
        natural_lang_synonyms: vec![
            "定义语法".to_string(),
            "创建语法规则".to_string(),
        ],
    };

    let rule = GrammarRule::new(
        "语法定义".to_string(),
        pattern,
        production,
        meta,
    );

    let self_describing_code = r#"
语法 语法定义:
    模式: "语法" 标识符 ":" 模式列表 "=>" 产生式
    产生: 创建语法规则(标识符, 模式列表, 产生式)
    版本: "1.0"
    定义方式: "自描述"
    稳定性: "稳定"
"#.to_string();

    SelfDescribingRule::new(rule, self_describing_code)
}

