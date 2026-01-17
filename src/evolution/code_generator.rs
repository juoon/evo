// 智能代码生成器 / Intelligent code generator
// 基于上下文、使用模式和学习结果生成代码
// Generate code based on context, usage patterns, and learning results

use crate::evolution::learning::UsagePatternLearner;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 智能代码生成器 / Intelligent code generator
pub struct IntelligentCodeGenerator {
    /// 使用模式学习器 / Usage pattern learner
    learner: UsagePatternLearner,
    /// 代码模板库 / Code template library
    templates: HashMap<String, CodeTemplate>,
    /// 上下文信息 / Context information
    context: GenerationContext,
}

/// 代码模板 / Code template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTemplate {
    /// 模板名称 / Template name
    pub name: String,
    /// 模板代码 / Template code
    pub code: String,
    /// 使用次数 / Usage count
    pub usage_count: usize,
    /// 成功率 / Success rate
    pub success_rate: f64,
    /// 适用场景 / Applicable scenarios
    pub scenarios: Vec<String>,
}

/// 生成上下文 / Generation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationContext {
    /// 当前变量 / Current variables
    pub variables: Vec<String>,
    /// 当前函数 / Current functions
    pub functions: Vec<String>,
    /// 最近使用的模式 / Recently used patterns
    pub recent_patterns: Vec<String>,
    /// 用户意图 / User intent
    pub intent: Option<String>,
}

/// 代码生成结果 / Code generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    /// 生成的代码 / Generated code
    pub code: String,
    /// 置信度 / Confidence
    pub confidence: f64,
    /// 使用的模板 / Template used
    pub template: Option<String>,
    /// 建议 / Suggestions
    pub suggestions: Vec<String>,
}

impl IntelligentCodeGenerator {
    /// 创建新代码生成器 / Create new code generator
    pub fn new() -> Self {
        let mut generator = Self {
            learner: UsagePatternLearner::new(),
            templates: HashMap::new(),
            context: GenerationContext {
                variables: Vec::new(),
                functions: Vec::new(),
                recent_patterns: Vec::new(),
                intent: None,
            },
        };
        generator.initialize_templates();
        generator
    }

    /// 初始化模板库 / Initialize template library
    fn initialize_templates(&mut self) {
        // 变量定义模板 / Variable definition template
        self.templates.insert(
            "variable_definition".to_string(),
            CodeTemplate {
                name: "变量定义".to_string(),
                code: "(let {name} {value})".to_string(),
                usage_count: 0,
                success_rate: 0.95,
                scenarios: vec!["定义变量".to_string(), "初始化变量".to_string()],
            },
        );

        // 函数定义模板 / Function definition template
        self.templates.insert(
            "function_definition".to_string(),
            CodeTemplate {
                name: "函数定义".to_string(),
                code: "(def {name} ({params}) {body})".to_string(),
                usage_count: 0,
                success_rate: 0.90,
                scenarios: vec!["定义函数".to_string(), "创建函数".to_string()],
            },
        );

        // 条件表达式模板 / Conditional expression template
        self.templates.insert(
            "conditional".to_string(),
            CodeTemplate {
                name: "条件表达式".to_string(),
                code: "(if {condition} {then} {else})".to_string(),
                usage_count: 0,
                success_rate: 0.85,
                scenarios: vec!["条件判断".to_string(), "分支逻辑".to_string()],
            },
        );

        // 列表操作模板 / List operation template
        self.templates.insert(
            "list_operation".to_string(),
            CodeTemplate {
                name: "列表操作".to_string(),
                code: "(list-{op} {list} {args})".to_string(),
                usage_count: 0,
                success_rate: 0.88,
                scenarios: vec!["列表处理".to_string(), "数据操作".to_string()],
            },
        );
    }

    /// 基于意图生成代码 / Generate code based on intent
    pub fn generate_from_intent(
        &mut self,
        intent: &str,
        context: &GenerationContext,
    ) -> GenerationResult {
        self.context = context.clone();

        // 分析意图 / Analyze intent
        let intent_lower = intent.to_lowercase();
        let mut best_template: Option<&CodeTemplate> = None;
        let mut best_score = 0.0;

        // 查找匹配的模板 / Find matching template
        for template in self.templates.values() {
            let score = self.score_template(template, &intent_lower);
            if score > best_score {
                best_score = score;
                best_template = Some(template);
            }
        }

        // 基于学习结果优化 / Optimize based on learning results
        let insights = self.learner.get_insights();
        let mut suggestions = Vec::new();

        for insight in &insights {
            if insight.priority > 3 {
                if let Some(suggestion) = &insight.suggestion {
                    suggestions.push(suggestion.clone());
                }
            }
        }

        // 生成代码 / Generate code
        if let Some(template) = best_template {
            let code = self.fill_template(template, intent);
            let confidence = best_score * template.success_rate;

            // 记录使用 / Record usage
            self.learner.record_usage(&template.name);
            self.learner.record_success(&template.name, &code);

            GenerationResult {
                code,
                confidence,
                template: Some(template.name.clone()),
                suggestions,
            }
        } else {
            // 如果没有匹配的模板，生成基础代码 / If no matching template, generate basic code
            let code = self.generate_basic_code(intent);
            GenerationResult {
                code,
                confidence: 0.5,
                template: None,
                suggestions,
            }
        }
    }

    /// 评分模板 / Score template
    fn score_template(&self, template: &CodeTemplate, intent: &str) -> f64 {
        let mut score = 0.0;

        // 场景匹配 / Scenario matching
        for scenario in &template.scenarios {
            if intent.contains(scenario) {
                score += 0.4;
            }
        }

        // 使用频率 / Usage frequency
        if template.usage_count > 0 {
            score += (template.usage_count as f64 / 100.0).min(0.3);
        }

        // 成功率 / Success rate
        score += template.success_rate * 0.3;

        score.min(1.0)
    }

    /// 填充模板 / Fill template
    fn fill_template(&self, template: &CodeTemplate, intent: &str) -> String {
        let mut code = template.code.clone();

        // 简单的模板填充 / Simple template filling
        // 实际实现需要更复杂的解析 / Actual implementation needs more complex parsing
        if intent.contains("变量") || intent.contains("variable") {
            code = code.replace("{name}", "x");
            code = code.replace("{value}", "0");
        } else if intent.contains("函数") || intent.contains("function") {
            code = code.replace("{name}", "func");
            code = code.replace("{params}", "x y");
            code = code.replace("{body}", "(+ x y)");
        } else if intent.contains("条件") || intent.contains("if") {
            code = code.replace("{condition}", "(> x 0)");
            code = code.replace("{then}", "x");
            code = code.replace("{else}", "0");
        } else if intent.contains("列表") || intent.contains("list") {
            code = code.replace("{op}", "sum");
            code = code.replace("{list}", "(list 1 2 3)");
            code = code.replace("{args}", "");
        }

        code
    }

    /// 生成基础代码 / Generate basic code
    fn generate_basic_code(&self, intent: &str) -> String {
        // 基于意图生成基础代码结构 / Generate basic code structure based on intent
        if intent.contains("计算") || intent.contains("calculate") {
            "(+ 1 2)".to_string()
        } else if intent.contains("定义") || intent.contains("define") {
            "(let x 0)".to_string()
        } else {
            "(def func (x) x)".to_string()
        }
    }

    /// 提供代码补全建议 / Provide code completion suggestions
    pub fn suggest_completion(
        &self,
        partial_code: &str,
        context: &GenerationContext,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        // 基于使用模式提供建议 / Provide suggestions based on usage patterns
        let frequent_patterns = self.learner.get_frequent_patterns(2);
        for (pattern, _) in frequent_patterns.iter().take(5) {
            if pattern.contains(partial_code) || partial_code.is_empty() {
                suggestions.push(pattern.clone());
            }
        }

        // 基于上下文提供建议 / Provide suggestions based on context
        for var in &context.variables {
            if var.starts_with(partial_code) {
                suggestions.push(format!("(let {} ...)", var));
            }
        }

        for func in &context.functions {
            if func.starts_with(partial_code) {
                suggestions.push(format!("({} ...)", func));
            }
        }

        suggestions
    }

    /// 优化生成的代码 / Optimize generated code
    pub fn optimize_code(&self, code: &str) -> String {
        // 基于学习洞察优化代码 / Optimize code based on learning insights
        let insights = self.learner.get_insights();
        let mut optimized = code.to_string();

        for insight in &insights {
            match insight.insight_type {
                crate::evolution::learning::InsightType::CodeSimplification => {
                    // 简化代码 / Simplify code
                    if let Some(suggestion) = &insight.suggestion {
                        if suggestion.contains("简化") {
                            // 应用简化规则 / Apply simplification rules
                            optimized = optimized.replace("(+ 0 ", "(+ ");
                            optimized = optimized.replace("(* 1 ", "(* ");
                        }
                    }
                }
                _ => {}
            }
        }

        optimized
    }

    /// 更新模板使用统计 / Update template usage statistics
    pub fn update_template_stats(&mut self, template_name: &str, success: bool) {
        if let Some(template) = self.templates.get_mut(template_name) {
            template.usage_count += 1;
            let total = template.usage_count as f64;
            if success {
                // 更新成功率 / Update success rate
                template.success_rate = (template.success_rate * (total - 1.0) + 1.0) / total;
            } else {
                template.success_rate = (template.success_rate * (total - 1.0)) / total;
            }
        }
    }
}

impl Default for IntelligentCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}
