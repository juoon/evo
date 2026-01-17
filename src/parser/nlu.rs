// 自然语言解析器 / Natural Language Understanding parser
// 将自然语言输入转换为代码结构
// Converts natural language input to code structures

use crate::grammar::core::{BinOp, Expr, GrammarElement, Literal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 自然语言解析器 / Natural language parser
pub struct NLUParser {
    /// 使用的模型类型 / Model type used
    model_type: ModelType,
    /// 是否使用本地模型 / Whether to use local model
    use_local: bool,
    /// 规则库 / Rule database
    rules: RuleDatabase,
}

/// 模型类型 / Model type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelType {
    /// 基于规则 / Rule-based
    RuleBased,
    /// 本地轻量模型 / Local lightweight model
    LocalLightweight,
    /// 本地中等模型 / Local medium model
    LocalMedium,
    /// 云模型 / Cloud model
    Cloud,
    /// 专用微调模型 / Specialized fine-tuned model
    FineTuned,
}

/// 规则数据库 / Rule database
struct RuleDatabase {
    /// 函数定义关键词 / Function definition keywords
    function_keywords: Vec<&'static str>,
    /// 变量定义关键词 / Variable definition keywords
    variable_keywords: Vec<&'static str>,
    /// 操作关键词映射 / Operation keyword mapping
    operation_keywords: HashMap<&'static str, BinOp>,
    /// 数字提取模式 / Number extraction patterns
    number_patterns: Vec<&'static str>,
}

impl RuleDatabase {
    fn new() -> Self {
        let mut operation_keywords = HashMap::new();
        // 中文操作符
        operation_keywords.insert("加", BinOp::Add);
        operation_keywords.insert("加上", BinOp::Add);
        operation_keywords.insert("减", BinOp::Sub);
        operation_keywords.insert("减去", BinOp::Sub);
        operation_keywords.insert("乘", BinOp::Mul);
        operation_keywords.insert("乘以", BinOp::Mul);
        operation_keywords.insert("除", BinOp::Div);
        operation_keywords.insert("除以", BinOp::Div);
        operation_keywords.insert("等于", BinOp::Eq);
        operation_keywords.insert("大于", BinOp::Gt);
        operation_keywords.insert("小于", BinOp::Lt);
        operation_keywords.insert("小于等于", BinOp::Le);
        operation_keywords.insert("大于等于", BinOp::Ge);
        operation_keywords.insert("不等于", BinOp::Ne);
        // 英文操作符
        operation_keywords.insert("plus", BinOp::Add);
        operation_keywords.insert("add", BinOp::Add);
        operation_keywords.insert("minus", BinOp::Sub);
        operation_keywords.insert("subtract", BinOp::Sub);
        operation_keywords.insert("multiply", BinOp::Mul);
        operation_keywords.insert("times", BinOp::Mul);
        operation_keywords.insert("divide", BinOp::Div);
        operation_keywords.insert("equals", BinOp::Eq);
        operation_keywords.insert("equal", BinOp::Eq);
        operation_keywords.insert("greater than", BinOp::Gt);
        operation_keywords.insert("less than", BinOp::Lt);

        Self {
            function_keywords: vec![
                "定义函数", "创建一个函数", "写一个函数", "函数", "def", "define", "function",
                "create function", "make function", "定义一个函数", "写函数",
            ],
            variable_keywords: vec![
                "定义变量", "创建一个变量", "变量", "let", "variable", "var", "set",
                "create variable", "定义一个变量", "设置变量", "赋值",
            ],
            operation_keywords,
            number_patterns: vec!["零", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十"],
        }
    }
}

impl NLUParser {
    /// 创建新NLU解析器（基于规则） / Create new NLU parser (rule-based)
    pub fn new_rule_based() -> Self {
        Self {
            model_type: ModelType::RuleBased,
            use_local: true,
            rules: RuleDatabase::new(),
        }
    }

    /// 创建新NLU解析器 / Create new NLU parser
    #[allow(dead_code)]
    pub fn new(model_type: ModelType, use_local: bool) -> Self {
        Self {
            model_type,
            use_local,
            rules: RuleDatabase::new(),
        }
    }

    /// 解析自然语言 / Parse natural language
    pub fn parse(&self, input: &str) -> Result<ParsedIntent, NLUError> {
        let input_trimmed = input.trim();
        let statements = self.split_into_statements(input_trimmed);

        if statements.len() > 1 {
            let mut all_elements = Vec::new();
            let mut total_confidence = 0.0;
            for statement in &statements {
                let intent_type = self.detect_intent_type(&statement.to_lowercase())?;
                let elements = self.generate_code_structure(statement, &intent_type)?;
                total_confidence += self.calculate_confidence(statement, &intent_type);
                all_elements.extend(elements);
            }
            let avg_confidence = total_confidence / statements.len() as f64;
            return Ok(ParsedIntent {
                intent_type: IntentType::Other("Sequence".to_string()),
                code_structure: all_elements,
                confidence: avg_confidence,
                suggested_rules: vec![],
            });
        }

        let input_lower = input_trimmed.to_lowercase();

        // 尝试识别意图类型
        let intent_type = self.detect_intent_type(&input_lower)?;

        // 根据意图类型生成代码结构
        let code_structure = self.generate_code_structure(input_trimmed, &intent_type)?;

        // 计算置信度
        let confidence = self.calculate_confidence(input_trimmed, &intent_type);

        Ok(ParsedIntent {
            intent_type,
            code_structure,
            confidence,
            suggested_rules: vec![],
        })
    }

    /// 提取编程意图 / Extract programming intent
    pub fn extract_intent(&self, input: &str) -> Result<ProgrammingIntent, NLUError> {
        let input_lower = input.trim().to_lowercase();
        let intent_type = self.detect_intent_type(&input_lower)?;
        
        let (action, entities, parameters) = match intent_type {
            IntentType::DefineFunction => {
                let func_name = self.extract_function_name(input)?;
                let params = self.extract_function_params(input)?;
                (
                    "define_function".to_string(),
                    vec![func_name.clone()],
                    params.iter().map(|p| ("param".to_string(), p.clone())).collect(),
                )
            }
            IntentType::DefineVariable => {
                let var_name = self.extract_variable_name(input)?;
                (
                    "define_variable".to_string(),
                    vec![var_name.clone()],
                    vec![],
                )
            }
            IntentType::ExecuteOperation => {
                (
                    "execute_operation".to_string(),
                    vec![],
                    vec![],
                )
            }
            IntentType::Conditional => {
                (
                    "conditional_expression".to_string(),
                    vec![],
                    vec![],
                )
            }
            _ => (
                "unknown".to_string(),
                vec![],
                vec![],
            ),
        };
        
        Ok(ProgrammingIntent {
            action,
            entities,
            parameters,
            context: Some(input.to_string()),
        })
    }

    /// 检测意图类型 / Detect intent type
    fn detect_intent_type(&self, input: &str) -> Result<IntentType, NLUError> {
        // 条件表达式检测
        if (input.contains("如果") && (input.contains("否则") || input.contains("不然")))
            || (input.contains("if") && input.contains("else"))
        {
            return Ok(IntentType::Conditional);
        }

        // 检查是否是函数定义
        for keyword in &self.rules.function_keywords {
            if input.contains(keyword) {
                return Ok(IntentType::DefineFunction);
            }
        }
        
        // 检查是否是变量定义
        for keyword in &self.rules.variable_keywords {
            if input.contains(keyword) {
                return Ok(IntentType::DefineVariable);
            }
        }
        
        // 检查是否是操作
        for keyword in self.rules.operation_keywords.keys() {
            if input.contains(keyword) {
                return Ok(IntentType::ExecuteOperation);
            }
        }
        
        // 默认返回执行操作
        Ok(IntentType::ExecuteOperation)
    }

    /// 生成代码结构 / Generate code structure
    fn generate_code_structure(
        &self,
        input: &str,
        intent_type: &IntentType,
    ) -> Result<Vec<GrammarElement>, NLUError> {
        match intent_type {
            IntentType::DefineFunction => self.generate_function_definition(input),
            IntentType::DefineVariable => self.generate_variable_definition(input),
            IntentType::ExecuteOperation => self.generate_operation(input),
            IntentType::Conditional => self.generate_conditional(input),
            _ => Err(NLUError::UnsupportedOperation(format!("{:?}", intent_type))),
        }
    }

    /// 生成函数定义 / Generate function definition
    fn generate_function_definition(&self, input: &str) -> Result<Vec<GrammarElement>, NLUError> {
        let func_name = self.extract_function_name(input)?;
        let params = self.extract_function_params(input)?;
        let body = self.extract_function_body(input)?;
        
        // 构建函数定义: (def func_name (param1 param2 ...) body)
        let mut elements = vec![
            GrammarElement::Atom("def".to_string()),
            GrammarElement::Atom(func_name),
        ];
        
        // 参数列表
        let param_list: Vec<GrammarElement> = params
            .iter()
            .map(|p| GrammarElement::Atom(p.clone()))
            .collect();
        elements.push(GrammarElement::List(param_list));
        
        // 函数体
        elements.push(body);
        
        Ok(vec![GrammarElement::List(elements)])
    }

    /// 生成变量定义 / Generate variable definition
    fn generate_variable_definition(&self, input: &str) -> Result<Vec<GrammarElement>, NLUError> {
        let var_name = self.extract_variable_name(input)?;
        let value = self.extract_variable_value(input)?;
        
        // 构建变量定义: (let var_name value)
        let elements = vec![
            GrammarElement::Atom("let".to_string()),
            GrammarElement::Atom(var_name),
            value,
        ];
        
        Ok(vec![GrammarElement::List(elements)])
    }

    /// 生成操作表达式 / Generate operation expression
    fn generate_operation(&self, input: &str) -> Result<Vec<GrammarElement>, NLUError> {
        // 尝试解析为复杂表达式
        let expr = self.parse_expression_from_text(input)?;
        Ok(vec![GrammarElement::Expr(Box::new(expr))])
    }

    /// 生成条件表达式 / Generate conditional expression
    fn generate_conditional(&self, input: &str) -> Result<Vec<GrammarElement>, NLUError> {
        let (cond_text, then_text, else_text) = self.extract_conditional_parts(input)?;
        let cond_expr = self.parse_expression_from_text(&cond_text)?;
        let then_expr = self.parse_expression_from_text(&then_text)?;
        let else_expr = self.parse_expression_from_text(&else_text)?;

        let expr = Expr::If(
            Box::new(cond_expr),
            Box::new(then_expr),
            Box::new(else_expr),
        );
        Ok(vec![GrammarElement::Expr(Box::new(expr))])
    }

    /// 提取函数名 / Extract function name
    fn extract_function_name(&self, input: &str) -> Result<String, NLUError> {
        // 增强的模式匹配：查找"函数名"、"叫"、"名为"等关键词后的名称
        let patterns = vec![
            ("函数叫", "function called"),
            ("函数名为", "function named"),
            ("函数", "function"),
            ("叫", "called"),
            ("名为", "named"),
            ("def", "def"),
            ("define", "define"),
        ];
        
        for (cn_pattern, en_pattern) in patterns {
            // 尝试中文模式
            if let Some(pos) = input.find(cn_pattern) {
                let after = &input[pos + cn_pattern.len()..];
                // 提取函数名，支持中英文和数字
                let name = self.extract_identifier(after);
                if !name.is_empty() && name != "func" {
                    return Ok(name);
                }
            }
            // 尝试英文模式
            if let Some(pos) = input.find(en_pattern) {
                let after = &input[pos + en_pattern.len()..];
                let name = self.extract_identifier(after);
                if !name.is_empty() && name != "func" {
                    return Ok(name);
                }
            }
        }
        
        // 尝试从"定义一个函数add"这样的模式中提取
        for keyword in &self.rules.function_keywords {
            if let Some(pos) = input.find(keyword) {
                let after = &input[pos + keyword.len()..];
                // 跳过"一个"、"个"等词
                let cleaned = after.trim_start_matches(|c: char| c == '一' || c == '个' || c == ' ' || c == 'a' || c == 'A');
                let name = self.extract_identifier(cleaned);
                if !name.is_empty() && name != "func" {
                    return Ok(name);
                }
            }
        }
        
        // 默认函数名
        Ok("func".to_string())
    }
    
    /// 提取标识符（函数名、变量名等）/ Extract identifier
    fn extract_identifier(&self, text: &str) -> String {
        let text = text.trim();
        if text.is_empty() {
            return String::new();
        }
        
        // 尝试提取第一个有效的标识符
        let mut result = String::new();
        let mut started = false;
        
        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                started = true;
            } else if started {
                // 遇到非标识符字符，停止提取
                break;
            } else if ch == ' ' || ch == '，' || ch == ',' {
                // 跳过空格和逗号
                continue;
            } else {
                // 遇到其他字符，停止
                break;
            }
        }
        
        result
    }

    /// 提取函数参数 / Extract function parameters
    fn extract_function_params(&self, input: &str) -> Result<Vec<String>, NLUError> {
        // 查找参数列表，通常在"参数"、"parameters"、"接受"等词后
        let patterns = vec![
            ("参数是", "parameters are"),
            ("参数", "parameters"),
            ("接受", "takes"),
            ("输入", "input"),
            ("参数为", "params"),
        ];
        
        for (cn_pattern, en_pattern) in patterns {
            if let Some(pos) = input.find(cn_pattern) {
                let after = &input[pos + cn_pattern.len()..];
                let params = self.parse_parameter_list(after);
                if !params.is_empty() {
                    return Ok(params);
                }
            }
            if let Some(pos) = input.find(en_pattern) {
                let after = &input[pos + en_pattern.len()..];
                let params = self.parse_parameter_list(after);
                if !params.is_empty() {
                    return Ok(params);
                }
            }
        }
        
        // 尝试从"函数名(x, y)"这样的模式中提取
        if let Some(start) = input.find('(') {
            if let Some(end) = input.find(')') {
                let param_str = &input[start + 1..end];
                let params = self.parse_parameter_list(param_str);
                if !params.is_empty() {
                    return Ok(params);
                }
            }
        }
        
        // 尝试从"函数名 x y"这样的模式中提取（函数名后的单词）
        if let Some(func_name) = self.extract_function_name(input).ok() {
            if let Some(pos) = input.find(&func_name) {
                let after = &input[pos + func_name.len()..];
                // 跳过"叫"、"名为"等词
                let cleaned = after.trim_start_matches(|c: char| 
                    c == '叫' || c == '名' || c == '为' || c == ' ' || 
                    c == 'c' || c == 'a' || c == 'l' || c == 'e' || c == 'd'
                );
                let params = self.parse_parameter_list(cleaned);
                if !params.is_empty() {
                    return Ok(params);
                }
            }
        }
        
        Ok(vec![])
    }
    
    /// 解析参数列表 / Parse parameter list
    fn parse_parameter_list(&self, text: &str) -> Vec<String> {
        let mut params = Vec::new();
        
        // 分割参数：支持中文逗号、英文逗号、空格、"和"、"and"等
        let parts: Vec<&str> = text
            .split(|c: char| c == '，' || c == ',' || c == ' ' || c == '(' || c == ')')
            .collect();
        
        for part in parts {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            
            // 跳过"和"、"and"等连接词
            if trimmed == "和" || trimmed == "and" || trimmed == "与" {
                continue;
            }
            
            // 提取标识符
            let param = self.extract_identifier(trimmed);
            if !param.is_empty() && !params.contains(&param) {
                params.push(param);
            }
            
            // 最多5个参数
            if params.len() >= 5 {
                break;
            }
        }
        
        params
    }

    /// 提取函数体 / Extract function body
    fn extract_function_body(&self, input: &str) -> Result<GrammarElement, NLUError> {
        // 查找函数体的关键词
        let body_patterns = vec![
            ("返回", "return"),
            ("结果是", "result is"),
            ("计算", "calculate"),
            ("执行", "execute"),
            ("做", "do"),
            ("等于", "equals"),
            ("是", "is"),
        ];
        
        // 尝试提取函数体表达式
        for (cn_pattern, en_pattern) in body_patterns {
            // 中文模式
            if let Some(pos) = input.find(cn_pattern) {
                let after = &input[pos + cn_pattern.len()..];
                if let Ok(expr) = self.parse_expression_from_text(after) {
                    return Ok(GrammarElement::Expr(Box::new(expr)));
                }
            }
            // 英文模式
            if let Some(pos) = input.find(en_pattern) {
                let after = &input[pos + en_pattern.len()..];
                if let Ok(expr) = self.parse_expression_from_text(after) {
                    return Ok(GrammarElement::Expr(Box::new(expr)));
                }
            }
        }
        
        // 如果没有找到明确的函数体，尝试从整个输入中提取操作
        // 例如："定义一个函数add，参数是x和y，x加y"
        if let Ok((op, left, right)) = self.extract_operation(input) {
            let left_expr = self.parse_value_to_expr(&left)?;
            let right_expr = self.parse_value_to_expr(&right)?;
            let expr = Expr::Binary(op, Box::new(left_expr), Box::new(right_expr));
            return Ok(GrammarElement::Expr(Box::new(expr)));
        }
        
        // 默认返回一个简单的表达式（使用第一个参数）
        Ok(GrammarElement::Expr(Box::new(Expr::Literal(Literal::Null))))
    }
    
    /// 从文本中解析表达式 / Parse expression from text
    fn parse_expression_from_text(&self, text: &str) -> Result<Expr, NLUError> {
        let text = text.trim();
        
        // 尝试提取操作（支持嵌套表达式）
        if let Ok((op, left, right)) = self.extract_operation(text) {
            let left_expr = self.parse_expression_from_text(&left)?;
            let right_expr = self.parse_expression_from_text(&right)?;
            return Ok(Expr::Binary(op, Box::new(left_expr), Box::new(right_expr)));
        }
        
        // 尝试解析为单个值
        self.parse_value_to_expr(text)
    }

    /// 拆分多条语句 / Split multiple statements
    fn split_into_statements(&self, input: &str) -> Vec<String> {
        let connectors = [
            "然后",
            "并且",
            "同时",
            "接着",
            "再",
            "then",
            "and then",
            "and",
        ];

        let mut parts = vec![input.to_string()];
        for connector in connectors.iter() {
            let mut new_parts = Vec::new();
            for part in parts {
                if part.contains(connector) {
                    let split: Vec<&str> = part.split(connector).collect();
                    for item in split {
                        let trimmed = item.trim();
                        if !trimmed.is_empty() {
                            new_parts.push(trimmed.to_string());
                        }
                    }
                } else {
                    new_parts.push(part);
                }
            }
            parts = new_parts;
        }

        parts.into_iter().filter(|p| !p.is_empty()).collect()
    }

    /// 提取条件表达式 / Extract conditional parts
    fn extract_conditional_parts(&self, input: &str) -> Result<(String, String, String), NLUError> {
        let input_lower = input.to_lowercase();

        // 中文模式：如果 ... [那么/则] ... 否则/不然 ...
        if let Some(if_pos) = input.find("如果") {
            let after_if = &input[if_pos + "如果".len()..];
            if let Some(else_pos) = after_if.find("否则").or_else(|| after_if.find("不然")) {
                let cond_then = &after_if[..else_pos];
                let else_part = &after_if[else_pos + 2..];
                let then_part = cond_then
                    .trim_start_matches("那么")
                    .trim_start_matches("则")
                    .trim();
                return Ok((cond_then.trim().to_string(), then_part.to_string(), else_part.trim().to_string()));
            }
        }

        // 英文模式：if ... then ... else ...
        if let Some(if_pos) = input_lower.find("if") {
            let after_if = &input[if_pos + 2..];
            if let Some(else_pos) = after_if.to_lowercase().find("else") {
                let cond_then = &after_if[..else_pos];
                let else_part = &after_if[else_pos + 4..];
                let then_part = cond_then
                    .trim_start_matches("then")
                    .trim();
                return Ok((cond_then.trim().to_string(), then_part.to_string(), else_part.trim().to_string()));
            }
        }

        Err(NLUError::UnsupportedOperation(
            "Conditional expression must include if/else or 如果/否则".to_string(),
        ))
    }

    /// 提取变量名 / Extract variable name
    fn extract_variable_name(&self, input: &str) -> Result<String, NLUError> {
        let patterns = vec![
            ("变量叫", "variable called"),
            ("变量名为", "variable named"),
            ("变量", "variable"),
            ("叫", "called"),
            ("名为", "named"),
            ("let", "let"),
            ("var", "var"),
            ("等于", "equals"),
            ("=", "="),
        ];
        
        for (cn_pattern, en_pattern) in patterns {
            if let Some(pos) = input.find(cn_pattern) {
                let after = &input[pos + cn_pattern.len()..];
                let name = self.extract_identifier(after);
                if !name.is_empty() && name != "x" {
                    return Ok(name);
                }
            }
            if let Some(pos) = input.find(en_pattern) {
                let after = &input[pos + en_pattern.len()..];
                let name = self.extract_identifier(after);
                if !name.is_empty() && name != "x" {
                    return Ok(name);
                }
            }
        }
        
        // 尝试从"定义一个变量x等于10"这样的模式中提取
        for keyword in &self.rules.variable_keywords {
            if let Some(pos) = input.find(keyword) {
                let after = &input[pos + keyword.len()..];
                let cleaned = after.trim_start_matches(|c: char| c == '一' || c == '个' || c == ' ' || c == 'a' || c == 'A');
                let name = self.extract_identifier(cleaned);
                if !name.is_empty() && name != "x" {
                    return Ok(name);
                }
            }
        }
        
        Ok("x".to_string())
    }

    /// 提取变量值 / Extract variable value
    fn extract_variable_value(&self, input: &str) -> Result<GrammarElement, NLUError> {
        // 查找"等于"、"="等关键词后的值
        let value_patterns = vec![
            ("等于", "equals"),
            ("=", "="),
            ("是", "is"),
        ];
        
        for (cn_pattern, en_pattern) in value_patterns {
            if let Some(pos) = input.find(cn_pattern) {
                let after = &input[pos + cn_pattern.len()..].trim();
                if let Ok(expr) = self.parse_expression_from_text(after) {
                    return Ok(GrammarElement::Expr(Box::new(expr)));
                }
            }
            if let Some(pos) = input.find(en_pattern) {
                let after = &input[pos + en_pattern.len()..].trim();
                if let Ok(expr) = self.parse_expression_from_text(after) {
                    return Ok(GrammarElement::Expr(Box::new(expr)));
                }
            }
        }
        
        // 尝试提取数字
        if let Ok(num) = self.extract_number(input) {
            return Ok(GrammarElement::Expr(Box::new(Expr::Literal(Literal::Int(num)))));
        }
        
        // 尝试提取字符串（如果输入包含引号）
        if let Some(start) = input.find('"') {
            if let Some(end) = input[start + 1..].find('"') {
                let str_value = &input[start + 1..start + 1 + end];
                return Ok(GrammarElement::Expr(Box::new(Expr::Literal(Literal::String(str_value.to_string())))));
            }
        }
        if let Some(start) = input.find('\'') {
            if let Some(end) = input[start + 1..].find('\'') {
                let str_value = &input[start + 1..start + 1 + end];
                return Ok(GrammarElement::Expr(Box::new(Expr::Literal(Literal::String(str_value.to_string())))));
            }
        }
        
        // 默认返回0
        Ok(GrammarElement::Expr(Box::new(Expr::Literal(Literal::Int(0)))))
    }

    /// 提取操作 / Extract operation
    fn extract_operation(&self, input: &str) -> Result<(BinOp, String, String), NLUError> {
        // 按优先级查找操作关键词（先查找更长的关键词）
        let mut found_ops: Vec<(usize, BinOp, &str)> = Vec::new();
        
        for (keyword, op) in &self.rules.operation_keywords {
            if let Some(pos) = input.find(keyword) {
                found_ops.push((pos, *op, keyword));
            }
        }
        
        // 按位置排序，找到第一个操作符
        found_ops.sort_by_key(|(pos, _, _)| *pos);
        
        if let Some((_pos, op, keyword)) = found_ops.first() {
            // 提取左右操作数
            let parts: Vec<&str> = input.split(keyword).collect();
            if parts.len() >= 2 {
                let left = parts[0].trim().to_string();
                // 处理右侧，可能包含多个部分
                let right_joined = parts[1..].join(keyword);
                let right_parts: Vec<&str> = right_joined.split_whitespace().collect();
                let right = right_parts.join(" ").trim().to_string();
                
                if !left.is_empty() && !right.is_empty() {
                    return Ok((*op, left, right));
                }
            }
        }
        
        // 尝试从"x + y"这样的模式中提取（如果输入包含操作符）
        if input.contains('+') {
            let parts: Vec<&str> = input.split('+').collect();
            if parts.len() >= 2 {
                return Ok((BinOp::Add, parts[0].trim().to_string(), parts[1].trim().to_string()));
            }
        }
        if input.contains('-') && !input.starts_with('-') {
            let parts: Vec<&str> = input.split('-').collect();
            if parts.len() >= 2 {
                return Ok((BinOp::Sub, parts[0].trim().to_string(), parts[1].trim().to_string()));
            }
        }
        if input.contains('*') {
            let parts: Vec<&str> = input.split('*').collect();
            if parts.len() >= 2 {
                return Ok((BinOp::Mul, parts[0].trim().to_string(), parts[1].trim().to_string()));
            }
        }
        if input.contains('/') {
            let parts: Vec<&str> = input.split('/').collect();
            if parts.len() >= 2 {
                return Ok((BinOp::Div, parts[0].trim().to_string(), parts[1].trim().to_string()));
            }
        }
        
        Err(NLUError::UnsupportedOperation(input.to_string()))
    }

    /// 提取数字 / Extract number
    fn extract_number(&self, input: &str) -> Result<i64, NLUError> {
        // 尝试直接解析数字
        for word in input.split_whitespace() {
            // 移除可能的标点符号
            let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric());
            if let Ok(num) = cleaned.parse::<i64>() {
                return Ok(num);
            }
        }
        
        // 尝试解析中文数字（支持简单组合）
        if let Ok(num) = self.parse_chinese_number(input) {
            return Ok(num);
        }
        
        Ok(0)
    }
    
    /// 解析中文数字 / Parse Chinese number
    fn parse_chinese_number(&self, input: &str) -> Result<i64, NLUError> {
        let cn_digits: HashMap<&str, i64> = [
            ("零", 0), ("一", 1), ("二", 2), ("三", 3), ("四", 4),
            ("五", 5), ("六", 6), ("七", 7), ("八", 8), ("九", 9),
        ]
        .iter()
        .cloned()
        .collect();
        
        let cn_units: HashMap<&str, i64> = [
            ("十", 10), ("百", 100), ("千", 1000), ("万", 10000),
        ]
        .iter()
        .cloned()
        .collect();
        
        let mut result = 0i64;
        let mut temp = 0i64;
        let mut found_digit = false;
        
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            let ch_str = &ch.to_string();
            
            // 检查是否是数字
            if let Some(&digit) = cn_digits.get(ch_str.as_str()) {
                temp = digit;
                found_digit = true;
                i += 1;
                continue;
            }
            
            // 检查是否是单位
            if let Some(&unit) = cn_units.get(ch_str.as_str()) {
                if found_digit {
                    result += temp * unit;
                    temp = 0;
                } else if unit == 10 {
                    // "十"可以单独使用，表示10
                    result += 10;
                }
                found_digit = false;
                i += 1;
                continue;
            }
            
            // 如果遇到非数字字符，且已经找到数字，返回结果
            if found_digit && temp > 0 {
                result += temp;
                break;
            }
            
            i += 1;
        }
        
        // 处理末尾的数字
        if found_digit && temp > 0 {
            result += temp;
        }
        
        if result > 0 {
            Ok(result)
        } else {
            Err(NLUError::UnsupportedOperation("无法解析中文数字".to_string()))
        }
    }

    /// 将值解析为表达式 / Parse value to expression
    fn parse_value_to_expr(&self, value: &str) -> Result<Expr, NLUError> {
        // 尝试解析为数字
        if let Ok(num) = value.parse::<i64>() {
            return Ok(Expr::Literal(Literal::Int(num)));
        }
        
        // 尝试解析为浮点数
        if let Ok(num) = value.parse::<f64>() {
            return Ok(Expr::Literal(Literal::Float(num)));
        }
        
        // 尝试解析为布尔值
        if value == "true" || value == "真" {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if value == "false" || value == "假" {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        // 尝试解析为字符串字面量
        if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
            return Ok(Expr::Literal(Literal::String(
                value[1..value.len() - 1].to_string(),
            )));
        }
        if value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2 {
            return Ok(Expr::Literal(Literal::String(
                value[1..value.len() - 1].to_string(),
            )));
        }
        
        // 尝试作为变量
        if value.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Ok(Expr::Var(value.to_string()));
        }
        
        // 默认返回0
        Ok(Expr::Literal(Literal::Int(0)))
    }

    /// 计算置信度 / Calculate confidence
    fn calculate_confidence(&self, input: &str, intent_type: &IntentType) -> f64 {
        let mut score: f64 = 0.5; // 基础分数
        
        // 根据关键词匹配提高置信度
        match intent_type {
            IntentType::DefineFunction => {
                for keyword in &self.rules.function_keywords {
                    if input.contains(keyword) {
                        score += 0.2;
                    }
                }
            }
            IntentType::DefineVariable => {
                for keyword in &self.rules.variable_keywords {
                    if input.contains(keyword) {
                        score += 0.2;
                    }
                }
            }
            IntentType::ExecuteOperation => {
                for keyword in self.rules.operation_keywords.keys() {
                    if input.contains(keyword) {
                        score += 0.2;
                    }
                }
            }
            _ => {}
        }
        
        score.min(1.0)
    }
}

/// 解析后的意图 / Parsed intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIntent {
    /// 意图类型 / Intent type
    pub intent_type: IntentType,
    /// 提取的代码结构 / Extracted code structure
    pub code_structure: Vec<GrammarElement>,
    /// 置信度 / Confidence
    pub confidence: f64,
    /// 建议的语法规则 / Suggested grammar rules
    pub suggested_rules: Vec<String>,
}

/// 意图类型 / Intent type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentType {
    /// 定义函数 / Define function
    DefineFunction,
    /// 定义变量 / Define variable
    DefineVariable,
    /// 执行操作 / Execute operation
    ExecuteOperation,
    /// 条件表达式 / Conditional expression
    Conditional,
    /// 扩展语法 / Extend syntax
    ExtendSyntax,
    /// 其他 / Other
    Other(String),
}

/// 编程意图 / Programming intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgrammingIntent {
    /// 动作 / Action
    pub action: String,
    /// 实体 / Entities
    pub entities: Vec<String>,
    /// 参数 / Parameters
    pub parameters: Vec<(String, String)>,
    /// 上下文 / Context
    pub context: Option<String>,
}

/// NLU错误 / NLU error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NLUError {
    /// 未实现 / Not implemented
    NotImplemented,
    /// 模型错误 / Model error
    ModelError(String),
    /// 意图不明确 / Ambiguous intent
    AmbiguousIntent(String),
    /// 不支持的操作 / Unsupported operation
    UnsupportedOperation(String),
}

