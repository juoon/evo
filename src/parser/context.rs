// 上下文管理器 / Context manager
// 维护对话历史和状态，支持多轮对话
// Maintains conversation history and state, supports multi-turn conversations

use crate::grammar::core::GrammarElement;
use crate::parser::nlu::{ParsedIntent, ProgrammingIntent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 上下文管理器 / Context manager
pub struct ContextManager {
    /// 对话历史 / Conversation history
    history: Vec<ConversationTurn>,
    /// 变量上下文 / Variable context
    variables: HashMap<String, GrammarElement>,
    /// 函数上下文 / Function context
    functions: HashMap<String, ParsedIntent>,
    /// 当前会话ID / Current session ID
    session_id: String,
}

/// 对话轮次 / Conversation turn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    /// 轮次ID / Turn ID
    pub turn_id: usize,
    /// 用户输入 / User input
    pub user_input: String,
    /// 解析的意图 / Parsed intent
    pub intent: Option<ParsedIntent>,
    /// 生成的代码 / Generated code
    pub generated_code: Option<Vec<GrammarElement>>,
    /// 执行结果 / Execution result
    pub execution_result: Option<String>,
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ContextManager {
    /// 创建新上下文管理器 / Create new context manager
    pub fn new(session_id: String) -> Self {
        Self {
            history: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
            session_id,
        }
    }

    /// 添加上下文 / Add context
    pub fn add_turn(&mut self, user_input: String, intent: Option<ParsedIntent>) -> usize {
        let turn_id = self.history.len();
        let turn = ConversationTurn {
            turn_id,
            user_input: user_input.clone(),
            intent: intent.clone(),
            generated_code: None,
            execution_result: None,
            timestamp: chrono::Utc::now(),
        };
        self.history.push(turn);

        // 更新变量和函数上下文 / Update variable and function context
        if let Some(intent) = intent {
            match intent.intent_type {
                crate::parser::nlu::IntentType::DefineVariable => {
                    // 提取变量名和值 / Extract variable name and value
                    if let Some(first) = intent.code_structure.first() {
                        // 简化：假设变量名在第一个元素中 / Simplified: assume variable name in first element
                        if let GrammarElement::Atom(name) = first {
                            // 这里应该提取值，简化处理 / Should extract value here, simplified
                            self.variables.insert(name.clone(), first.clone());
                        }
                    }
                }
                crate::parser::nlu::IntentType::DefineFunction => {
                    // 提取函数名 / Extract function name
                    if let Some(first) = intent.code_structure.first() {
                        if let GrammarElement::Atom(name) = first {
                            // 存储函数定义 / Store function definition
                            self.functions.insert(name.clone(), intent.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        turn_id
    }

    /// 获取上下文 / Get context
    pub fn get_context(&self, lookback: usize) -> Vec<ConversationTurn> {
        let start = if self.history.len() > lookback {
            self.history.len() - lookback
        } else {
            0
        };
        self.history[start..].to_vec()
    }

    /// 解析带上下文的输入 / Parse input with context
    pub fn parse_with_context(&self, input: &str) -> Result<EnhancedIntent, ContextError> {
        // 检查是否有引用之前的对话 / Check if there are references to previous conversations
        let references = self.extract_references(input);

        // 解析当前输入 / Parse current input
        // 这里应该调用NLUParser，但为了简化，我们创建一个基本的意图 / Should call NLUParser here, but simplified
        let base_intent = EnhancedIntent {
            original_input: input.to_string(),
            parsed_intent: None,
            context_references: references,
            resolved_variables: self.resolve_variables(input),
            resolved_functions: self.resolve_functions(input),
        };

        Ok(base_intent)
    }

    /// 提取引用 / Extract references
    fn extract_references(&self, input: &str) -> Vec<ContextReference> {
        let mut references = Vec::new();

        // 检查"上面的"、"之前的"、"刚才的"等引用 / Check for "above", "previous", "just now" references
        let reference_keywords = vec!["上面", "之前", "刚才", "之前定义的", "上面的变量"];
        for keyword in reference_keywords {
            if input.contains(keyword) {
                // 查找最近的匹配 / Find nearest match
                if let Some(last_turn) = self.history.last() {
                    references.push(ContextReference {
                        reference_type: ReferenceType::PreviousTurn,
                        turn_id: last_turn.turn_id,
                        description: format!("引用之前的对话: {}", last_turn.user_input),
                    });
                }
            }
        }

        references
    }

    /// 解析变量引用 / Resolve variable references
    fn resolve_variables(&self, input: &str) -> HashMap<String, GrammarElement> {
        let mut resolved = HashMap::new();

        // 检查输入中提到的变量名 / Check for variable names mentioned in input
        for (var_name, var_value) in &self.variables {
            if input.contains(var_name) {
                resolved.insert(var_name.clone(), var_value.clone());
            }
        }

        resolved
    }

    /// 解析函数引用 / Resolve function references
    fn resolve_functions(&self, input: &str) -> HashMap<String, ParsedIntent> {
        let mut resolved = HashMap::new();

        // 检查输入中提到的函数名 / Check for function names mentioned in input
        for (func_name, func_intent) in &self.functions {
            if input.contains(func_name) {
                resolved.insert(func_name.clone(), func_intent.clone());
            }
        }

        resolved
    }

    /// 获取历史 / Get history
    pub fn get_history(&self) -> &[ConversationTurn] {
        &self.history
    }

    /// 清除上下文 / Clear context
    pub fn clear(&mut self) {
        self.history.clear();
        self.variables.clear();
        self.functions.clear();
    }

    /// 更新执行结果 / Update execution result
    pub fn update_execution_result(&mut self, turn_id: usize, result: String) {
        if let Some(turn) = self.history.get_mut(turn_id) {
            turn.execution_result = Some(result);
        }
    }
}

/// 增强的意图 / Enhanced intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedIntent {
    /// 原始输入 / Original input
    pub original_input: String,
    /// 解析的意图 / Parsed intent
    pub parsed_intent: Option<ParsedIntent>,
    /// 上下文引用 / Context references
    pub context_references: Vec<ContextReference>,
    /// 解析的变量 / Resolved variables
    pub resolved_variables: HashMap<String, GrammarElement>,
    /// 解析的函数 / Resolved functions
    pub resolved_functions: HashMap<String, ParsedIntent>,
}

/// 上下文引用 / Context reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextReference {
    /// 引用类型 / Reference type
    pub reference_type: ReferenceType,
    /// 轮次ID / Turn ID
    pub turn_id: usize,
    /// 描述 / Description
    pub description: String,
}

/// 引用类型 / Reference type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    /// 之前的轮次 / Previous turn
    PreviousTurn,
    /// 变量引用 / Variable reference
    VariableReference,
    /// 函数引用 / Function reference
    FunctionReference,
}

/// 上下文错误 / Context error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextError {
    /// 未找到引用 / Reference not found
    ReferenceNotFound(String),
    /// 解析错误 / Parse error
    ParseError(String),
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::ReferenceNotFound(msg) => write!(f, "Reference not found: {}", msg),
            ContextError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ContextError {}
