// 错误恢复模块 / Error recovery module
// 自动修复常见错误，提供智能建议
// Automatically fix common errors and provide intelligent suggestions

use crate::runtime::interpreter::InterpreterError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 错误恢复器 / Error recoverer
pub struct ErrorRecoverer {
    /// 错误修复规则 / Error fix rules
    fix_rules: HashMap<String, Vec<FixRule>>,
}

/// 修复规则 / Fix rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixRule {
    /// 错误模式 / Error pattern
    pub error_pattern: String,
    /// 修复方法 / Fix method
    pub fix_method: FixMethod,
    /// 置信度 / Confidence
    pub confidence: f64,
    /// 描述 / Description
    pub description: String,
}

/// 修复方法 / Fix method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixMethod {
    /// 添加缺失的定义 / Add missing definition
    AddDefinition(String),
    /// 修复类型错误 / Fix type error
    FixType(String, String),
    /// 添加空值检查 / Add null check
    AddNullCheck,
    /// 修复语法错误 / Fix syntax error
    FixSyntax(String),
    /// 提供修复建议 / Provide fix suggestion
    SuggestFix(String),
}

/// 错误恢复结果 / Error recovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryResult {
    /// 是否成功恢复 / Whether recovery succeeded
    pub recovered: bool,
    /// 修复后的代码 / Fixed code
    pub fixed_code: Option<String>,
    /// 修复建议 / Fix suggestions
    pub suggestions: Vec<String>,
    /// 恢复方法 / Recovery method
    pub method: Option<String>,
}

impl ErrorRecoverer {
    /// 创建新错误恢复器 / Create new error recoverer
    pub fn new() -> Self {
        let mut recoverer = Self {
            fix_rules: HashMap::new(),
        };
        recoverer.initialize_fix_rules();
        recoverer
    }

    /// 初始化修复规则 / Initialize fix rules
    fn initialize_fix_rules(&mut self) {
        // 未定义变量修复 / Undefined variable fix
        self.fix_rules.insert(
            "UndefinedVariable".to_string(),
            vec![
                FixRule {
                    error_pattern: "变量未定义".to_string(),
                    fix_method: FixMethod::SuggestFix("在使用前定义变量".to_string()),
                    confidence: 0.8,
                    description: "建议在使用前定义变量".to_string(),
                },
                FixRule {
                    error_pattern: "变量未定义".to_string(),
                    fix_method: FixMethod::AddDefinition("变量定义".to_string()),
                    confidence: 0.6,
                    description: "自动添加变量定义".to_string(),
                },
            ],
        );

        // 类型错误修复 / Type error fix
        self.fix_rules.insert(
            "TypeError".to_string(),
            vec![
                FixRule {
                    error_pattern: "类型不匹配".to_string(),
                    fix_method: FixMethod::FixType("String".to_string(), "Int".to_string()),
                    confidence: 0.7,
                    description: "修复类型不匹配".to_string(),
                },
                FixRule {
                    error_pattern: "类型错误".to_string(),
                    fix_method: FixMethod::SuggestFix("检查参数类型是否匹配".to_string()),
                    confidence: 0.8,
                    description: "检查参数类型".to_string(),
                },
            ],
        );

        // 除零错误修复 / Division by zero fix
        self.fix_rules.insert(
            "DivisionByZero".to_string(),
            vec![FixRule {
                error_pattern: "除零错误".to_string(),
                fix_method: FixMethod::AddNullCheck,
                confidence: 0.9,
                description: "添加除数检查".to_string(),
            }],
        );

        // 语法错误修复 / Syntax error fix
        self.fix_rules.insert(
            "RuntimeError".to_string(),
            vec![FixRule {
                error_pattern: "语法错误".to_string(),
                fix_method: FixMethod::FixSyntax("修复语法结构".to_string()),
                confidence: 0.6,
                description: "修复语法错误".to_string(),
            }],
        );
    }

    /// 尝试恢复错误 / Try to recover from error
    pub fn recover_from_error(
        &self,
        error: &InterpreterError,
        code_context: &str,
    ) -> RecoveryResult {
        let error_type = self.extract_error_type(error);
        let mut suggestions = Vec::new();
        let mut fixed_code: Option<String> = None;
        let mut recovery_method: Option<String> = None;

        // 查找匹配的修复规则 / Find matching fix rules
        if let Some(rules) = self.fix_rules.get(&error_type) {
            for rule in rules {
                if code_context.contains(&rule.error_pattern) || self.matches_error(error, rule) {
                    match &rule.fix_method {
                        FixMethod::SuggestFix(suggestion) => {
                            suggestions.push(suggestion.clone());
                            recovery_method = Some("建议修复".to_string());
                        }
                        FixMethod::AddDefinition(def) => {
                            let fixed = self.add_definition(code_context, def);
                            fixed_code = Some(fixed);
                            recovery_method = Some("自动添加定义".to_string());
                            suggestions.push(rule.description.clone());
                        }
                        FixMethod::FixType(from, to) => {
                            suggestions.push(format!("将类型从 {} 改为 {}", from, to));
                            recovery_method = Some("类型修复".to_string());
                        }
                        FixMethod::AddNullCheck => {
                            suggestions.push("添加除数检查，确保除数不为0".to_string());
                            recovery_method = Some("添加空值检查".to_string());
                        }
                        FixMethod::FixSyntax(fix) => {
                            suggestions.push(fix.clone());
                            recovery_method = Some("语法修复".to_string());
                        }
                    }
                }
            }
        }

        // 如果没有找到匹配的规则，提供通用建议 / If no matching rule, provide general suggestion
        if suggestions.is_empty() {
            suggestions.push(format!("检查错误：{:?}", error));
            suggestions.push("查看代码上下文并手动修复".to_string());
        }

        RecoveryResult {
            recovered: fixed_code.is_some(),
            fixed_code,
            suggestions,
            method: recovery_method,
        }
    }

    /// 提取错误类型 / Extract error type
    fn extract_error_type(&self, error: &InterpreterError) -> String {
        match error {
            InterpreterError::UndefinedVariable { .. } => "UndefinedVariable".to_string(),
            InterpreterError::TypeError { .. } => "TypeError".to_string(),
            InterpreterError::DivisionByZero { .. } => "DivisionByZero".to_string(),
            InterpreterError::RuntimeError { .. } => "RuntimeError".to_string(),
            _ => "UnknownError".to_string(),
        }
    }

    /// 检查错误是否匹配规则 / Check if error matches rule
    fn matches_error(&self, error: &InterpreterError, rule: &FixRule) -> bool {
        match error {
            InterpreterError::UndefinedVariable { .. } => {
                rule.error_pattern.contains("未定义") || rule.error_pattern.contains("变量")
            }
            InterpreterError::TypeError { .. } => {
                rule.error_pattern.contains("类型") || rule.error_pattern.contains("Type")
            }
            InterpreterError::DivisionByZero { .. } => {
                rule.error_pattern.contains("除零") || rule.error_pattern.contains("Division")
            }
            _ => false,
        }
    }

    /// 添加定义 / Add definition
    fn add_definition(&self, code: &str, def_type: &str) -> String {
        // 简化版本：在实际使用前添加定义 / Simplified version: add definition before actual use
        // 实际实现需要更复杂的AST分析 / Actual implementation needs more complex AST analysis
        format!(
            "; 自动添加的{}定义 / Auto-added {} definition\n{}",
            def_type, def_type, code
        )
    }

    /// 获取常见错误的修复建议 / Get fix suggestions for common errors
    pub fn get_common_fixes(&self) -> Vec<FixRule> {
        self.fix_rules
            .values()
            .flat_map(|rules| rules.iter())
            .filter(|rule| rule.confidence > 0.7)
            .cloned()
            .collect()
    }
}

impl Default for ErrorRecoverer {
    fn default() -> Self {
        Self::new()
    }
}
