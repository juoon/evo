// 进化引擎核心 / Evolution engine core
// 驱动语言的自进化过程
// Drives the self-evolution process of the language

use crate::grammar::core::GrammarElement;
use crate::grammar::rule::{
    DefinitionMethod, GrammarRule, Pattern, PatternElement, Production, RuleMetadata, Stability,
};
use crate::evolution::tracker::{EvolutionTracker, EvolutionEvent, EvolutionType, TriggerSource};
use crate::parser::nlu::NLUParser;
use crate::parser::AdaptiveParser;
use crate::runtime::interpreter::{Interpreter, Value};
use std::collections::HashMap;

/// 进化引擎 / Evolution engine
pub struct EvolutionEngine {
    /// 语法变异记录 / Syntax mutation records
    syntax_mutations: Vec<GrammarRule>,
    /// 语义适应记录 / Semantic adaptation records
    semantic_adaptations: Vec<GrammarRule>,
    /// 进化追踪器 / Evolution tracker
    tracker: EvolutionTracker,
    /// NLU解析器 / NLU parser
    nlu_parser: NLUParser,
}

impl EvolutionEngine {
    /// 创建新进化引擎 / Create new evolution engine
    pub fn new() -> Self {
        let bootstrap_rules = Self::load_bootstrap_rules();
        Self {
            syntax_mutations: bootstrap_rules,
            semantic_adaptations: Vec::new(),
            tracker: EvolutionTracker::new(),
            nlu_parser: NLUParser::new(crate::parser::nlu::ModelType::LocalLightweight, true),
        }
    }

    /// 从自然语言进化 / Evolve from natural language
    pub fn evolve_from_natural_language(&mut self, nl_input: &str) -> Result<Vec<GrammarRule>, EvolutionError> {
        // 解析自然语言意图 / Parse natural language intent
        let intent = self.nlu_parser.extract_intent(nl_input)
            .map_err(|e| EvolutionError::NLUError(format!("{:?}", e)))?;

        // 生成语法变体 / Generate syntax variants
        let syntax_variants = self.generate_syntax_variants(&intent)?;

        // 测试并选择最优变体 / Test and select optimal variant
        let optimal = self.test_variants(syntax_variants)?;

        // 集成新特性 / Integrate new feature
        self.integrate_new_feature(optimal.clone())?;

        Ok(vec![optimal])
    }

    /// 生成语法变体 / Generate syntax variants
    fn generate_syntax_variants(
        &self,
        intent: &crate::parser::nlu::ProgrammingIntent,
    ) -> Result<Vec<GrammarRule>, EvolutionError> {
        let aevo_intent = self.intent_to_aevo_dict(intent);
        let code = format!(
            "(import \"evolution\")\n(evolution.generate_variants {})",
            aevo_intent
        );
        let value = self.execute_aevo_code(&code)?;
        self.rules_from_value(&value)
    }

    /// 测试变体 / Test variants
    fn test_variants(&self, variants: Vec<GrammarRule>) -> Result<GrammarRule, EvolutionError> {
        // TODO: 实现变体测试逻辑 / Implement variant testing logic
        // 暂时返回第一个变体 / Temporarily return first variant
        variants.into_iter().next()
            .ok_or(EvolutionError::NoVariants)
    }

    /// 集成新特性 / Integrate new feature
    fn integrate_new_feature(&mut self, rule: GrammarRule) -> Result<(), EvolutionError> {
        // 记录进化事件 / Record evolution event
        let event = EvolutionEvent {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_type: EvolutionType::SyntaxEvolution,
            before_state: crate::evolution::tracker::StateSnapshot {
                grammar_rules: self.syntax_mutations.clone(),
                version: "0.1.0".to_string(),
                metadata: serde_json::json!({}),
            },
            after_state: crate::evolution::tracker::StateSnapshot {
                grammar_rules: {
                    let mut rules = self.syntax_mutations.clone();
                    rules.push(rule.clone());
                    rules
                },
                version: "0.1.1".to_string(),
                metadata: serde_json::json!({}),
            },
            delta: crate::evolution::tracker::EvolutionDelta {
                added_rules: vec![rule.clone()],
                modified_rules: Vec::new(),
                removed_rules: Vec::new(),
                description: "Added new grammar rule from natural language".to_string(),
            },
            trigger: crate::evolution::tracker::TriggerContext {
                source: TriggerSource::NaturalLanguageInstruction,
                conditions: Vec::new(),
                environment: serde_json::json!({}),
            },
            author: None,
            success_metrics: None,
        };

        self.tracker.record(event);
        self.syntax_mutations.push(rule);
        Ok(())
    }

    /// 获取进化历史 / Get evolution history
    pub fn get_history(&self) -> &[EvolutionEvent] {
        self.tracker.get_history()
    }

    /// 获取语法规则 / Get syntax rules
    pub fn get_syntax_rules(&self) -> &[GrammarRule] {
        &self.syntax_mutations
    }

    /// 加载自举规则 / Load bootstrap rules
    fn load_bootstrap_rules() -> Vec<GrammarRule> {
        let code = "(import \"evolution\")\n(evolution.bootstrap_rules)";
        let mut interpreter = Interpreter::new();
        let parser = AdaptiveParser::new(true);
        let ast = match parser.parse(code) {
            Ok(ast) => ast,
            Err(_) => return Vec::new(),
        };
        let value = match interpreter.execute(&ast) {
            Ok(value) => value,
            Err(_) => return Vec::new(),
        };
        Self::rules_from_value_static(&value).unwrap_or_default()
    }

    /// 执行Aevolang代码 / Execute Aevolang code
    fn execute_aevo_code(&self, code: &str) -> Result<Value, EvolutionError> {
        let mut interpreter = Interpreter::new();
        let parser = AdaptiveParser::new(true);
        let ast = parser.parse(code).map_err(|e| {
            EvolutionError::IntegrationFailed(format!("Failed to parse aevo code: {:?}", e))
        })?;
        interpreter.execute(&ast).map_err(|e| {
            EvolutionError::IntegrationFailed(format!("Failed to execute aevo code: {:?}", e))
        })
    }

    /// Intent 转换为 Aevolang 字典文本 / Convert intent to Aevolang dict literal
    fn intent_to_aevo_dict(&self, intent: &crate::parser::nlu::ProgrammingIntent) -> String {
        let entities = self.list_to_aevo(&intent.entities);
        let params = self.pairs_to_aevo(&intent.parameters);
        let context = match &intent.context {
            Some(value) => self.string_to_aevo(value),
            None => "null".to_string(),
        };
        format!(
            "(dict \"action\" {} \"entities\" {} \"parameters\" {} \"context\" {})",
            self.string_to_aevo(&intent.action),
            entities,
            params,
            context
        )
    }

    fn string_to_aevo(&self, input: &str) -> String {
        let escaped = input.replace('\\', "\\\\").replace('"', "\\\"");
        format!("\"{}\"", escaped)
    }

    fn list_to_aevo(&self, items: &[String]) -> String {
        let values: Vec<String> = items.iter().map(|item| self.string_to_aevo(item)).collect();
        format!("(list {})", values.join(" "))
    }

    fn pairs_to_aevo(&self, pairs: &[(String, String)]) -> String {
        let values: Vec<String> = pairs
            .iter()
            .flat_map(|(k, v)| vec![self.string_to_aevo(k), self.string_to_aevo(v)])
            .collect();
        format!("(list {})", values.join(" "))
    }

    fn rules_from_value(&self, value: &Value) -> Result<Vec<GrammarRule>, EvolutionError> {
        Self::rules_from_value_static(value)
    }

    fn rules_from_value_static(value: &Value) -> Result<Vec<GrammarRule>, EvolutionError> {
        match value {
            Value::List(list) => list
                .iter()
                .map(Self::rule_from_value)
                .collect::<Result<Vec<_>, _>>(),
            _ => Err(EvolutionError::IntegrationFailed(
                "Expected a list of rules".to_string(),
            )),
        }
    }

    fn rule_from_value(value: &Value) -> Result<GrammarRule, EvolutionError> {
        let dict = match value {
            Value::Dict(map) => map,
            _ => {
                return Err(EvolutionError::IntegrationFailed(
                    "Rule must be a dict".to_string(),
                ))
            }
        };
        Self::rule_from_dict(dict)
    }

    fn rule_from_dict(dict: &HashMap<String, Value>) -> Result<GrammarRule, EvolutionError> {
        let name = Self::dict_string(dict, "name").unwrap_or_else(|| "unnamed".to_string());
        let production = Self::dict_string(dict, "production").unwrap_or_else(|| "Unknown".to_string());
        let description = Self::dict_string(dict, "description").unwrap_or_default();
        let variadic = Self::dict_bool(dict, "variadic").unwrap_or(false);
        let keywords = Self::dict_string_list(dict, "keywords");
        let examples = Self::dict_string_list(dict, "examples");
        let synonyms = Self::dict_string_list(dict, "synonyms");

        let elements = if keywords.is_empty() {
            vec![PatternElement::NaturalLang(name.clone())]
        } else {
            keywords
                .iter()
                .map(|keyword| PatternElement::NaturalLang(keyword.clone()))
                .collect()
        };

        let pattern = Pattern { elements, variadic };
        let production = Production {
            target: GrammarElement::Atom(production),
            transform: Vec::new(),
            conditions: Vec::new(),
        };
        let meta = RuleMetadata {
            version: "0.1.0".to_string(),
            defined_by: DefinitionMethod::Evolutionary,
            stability: Stability::Experimental,
            description,
            examples,
            natural_lang_synonyms: synonyms,
        };

        Ok(GrammarRule::new(name, pattern, production, meta))
    }

    fn dict_string(dict: &HashMap<String, Value>, key: &str) -> Option<String> {
        match dict.get(key) {
            Some(Value::String(value)) => Some(value.clone()),
            _ => None,
        }
    }

    fn dict_bool(dict: &HashMap<String, Value>, key: &str) -> Option<bool> {
        match dict.get(key) {
            Some(Value::Bool(value)) => Some(*value),
            _ => None,
        }
    }

    fn dict_string_list(dict: &HashMap<String, Value>, key: &str) -> Vec<String> {
        match dict.get(key) {
            Some(Value::List(items)) => items
                .iter()
                .filter_map(|item| match item {
                    Value::String(value) => Some(value.clone()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}

impl Default for EvolutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 进化错误 / Evolution error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvolutionError {
    /// NLU错误 / NLU error
    NLUError(String),
    /// 无变体 / No variants
    NoVariants,
    /// 测试失败 / Test failed
    TestFailed(String),
    /// 集成失败 / Integration failed
    IntegrationFailed(String),
}

