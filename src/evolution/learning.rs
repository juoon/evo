// 学习模块 / Learning module
// 从使用模式和错误中学习，改进语言能力
// Learn from usage patterns and errors to improve language capabilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 使用模式学习器 / Usage pattern learner
pub struct UsagePatternLearner {
    /// 使用频率统计 / Usage frequency statistics
    usage_frequency: HashMap<String, usize>,
    /// 错误模式统计 / Error pattern statistics
    error_patterns: HashMap<String, Vec<ErrorPattern>>,
    /// 成功模式统计 / Success pattern statistics
    success_patterns: HashMap<String, Vec<SuccessPattern>>,
}

/// 错误模式 / Error pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPattern {
    /// 错误类型 / Error type
    pub error_type: String,
    /// 错误消息 / Error message
    pub message: String,
    /// 代码上下文 / Code context
    pub context: String,
    /// 发生次数 / Occurrence count
    pub count: usize,
    /// 解决建议 / Resolution suggestion
    pub suggestion: Option<String>,
}

/// 成功模式 / Success pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessPattern {
    /// 模式描述 / Pattern description
    pub description: String,
    /// 代码片段 / Code snippet
    pub code: String,
    /// 使用次数 / Usage count
    pub usage_count: usize,
    /// 平均执行时间（可选）/ Average execution time (optional)
    pub avg_execution_time: Option<f64>,
}

impl UsagePatternLearner {
    /// 创建新学习器 / Create new learner
    pub fn new() -> Self {
        Self {
            usage_frequency: HashMap::new(),
            error_patterns: HashMap::new(),
            success_patterns: HashMap::new(),
        }
    }

    /// 记录使用 / Record usage
    pub fn record_usage(&mut self, pattern: &str) {
        *self.usage_frequency.entry(pattern.to_string()).or_insert(0) += 1;
    }

    /// 记录错误 / Record error
    pub fn record_error(&mut self, error_type: &str, message: &str, context: &str) {
        let pattern_key = format!("{}:{}", error_type, context);
        let suggestion = self.generate_error_suggestion(error_type, message, context);

        // 先检查是否存在 / Check if exists first
        if let Some(pattern_list) = self.error_patterns.get_mut(&pattern_key) {
            // 查找现有模式 / Find existing pattern
            if let Some(existing) = pattern_list.iter_mut().find(|p| p.message == message) {
                existing.count += 1;
                return;
            }
        }

        // 如果不存在，添加新模式 / If not exists, add new pattern
        self.error_patterns
            .entry(pattern_key)
            .or_insert_with(|| Vec::new())
            .push(ErrorPattern {
                error_type: error_type.to_string(),
                message: message.to_string(),
                context: context.to_string(),
                count: 1,
                suggestion,
            });
    }

    /// 记录成功 / Record success
    pub fn record_success(&mut self, description: &str, code: &str) {
        // 先检查是否存在 / Check if exists first
        if let Some(pattern_list) = self.success_patterns.get_mut(description) {
            if let Some(existing) = pattern_list.iter_mut().find(|p| p.code == code) {
                existing.usage_count += 1;
                return;
            }
        }

        // 如果不存在，添加新模式 / If not exists, add new pattern
        self.success_patterns
            .entry(description.to_string())
            .or_insert_with(|| Vec::new())
            .push(SuccessPattern {
                description: description.to_string(),
                code: code.to_string(),
                usage_count: 1,
                avg_execution_time: None,
            });
    }

    /// 生成错误建议 / Generate error suggestion
    fn generate_error_suggestion(
        &self,
        error_type: &str,
        message: &str,
        context: &str,
    ) -> Option<String> {
        match error_type {
            "UndefinedVariable" => Some(format!(
                "变量未定义，建议检查变量名拼写或在使用前定义变量: {}",
                message
            )),
            "TypeError" => Some(format!("类型错误，建议检查参数类型: {}", message)),
            "DivisionByZero" => Some("除零错误，建议在使用除法前检查除数是否为0".to_string()),
            _ => Some(format!("建议查看错误上下文并修复: {}", context)),
        }
    }

    /// 获取常用模式 / Get frequent patterns
    pub fn get_frequent_patterns(&self, threshold: usize) -> Vec<(String, usize)> {
        let mut patterns: Vec<(String, usize)> = self
            .usage_frequency
            .iter()
            .filter(|(_, &count)| count >= threshold)
            .map(|(pattern, &count)| (pattern.clone(), count))
            .collect();
        patterns.sort_by(|a, b| b.1.cmp(&a.1));
        patterns
    }

    /// 获取常见错误 / Get common errors
    pub fn get_common_errors(&self, limit: usize) -> Vec<ErrorPattern> {
        let mut all_errors: Vec<ErrorPattern> = self
            .error_patterns
            .values()
            .flat_map(|patterns| patterns.iter().cloned())
            .collect();
        all_errors.sort_by(|a, b| b.count.cmp(&a.count));
        all_errors.truncate(limit);
        all_errors
    }

    /// 获取成功模式 / Get success patterns
    pub fn get_success_patterns(&self, limit: usize) -> Vec<SuccessPattern> {
        let mut all_patterns: Vec<SuccessPattern> = self
            .success_patterns
            .values()
            .flat_map(|patterns| patterns.iter().cloned())
            .collect();
        all_patterns.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        all_patterns.truncate(limit);
        all_patterns
    }

    /// 从错误中学习 / Learn from errors
    pub fn learn_from_errors(&self) -> Vec<LearningInsight> {
        let mut insights = Vec::new();
        let common_errors = self.get_common_errors(10);

        for error in &common_errors {
            if error.count > 3 {
                insights.push(LearningInsight {
                    insight_type: InsightType::ErrorPrevention,
                    description: format!("常见错误: {} (发生{}次)", error.error_type, error.count),
                    suggestion: error.suggestion.clone(),
                    priority: error.count,
                });
            }
        }

        insights.sort_by(|a, b| b.priority.cmp(&a.priority));
        insights
    }

    /// 从成功中学习 / Learn from success
    pub fn learn_from_success(&self) -> Vec<LearningInsight> {
        let mut insights = Vec::new();
        let success_patterns = self.get_success_patterns(10);

        for pattern in &success_patterns {
            if pattern.usage_count > 5 {
                insights.push(LearningInsight {
                    insight_type: InsightType::PatternRecognition,
                    description: format!(
                        "常用模式: {} (使用{}次)",
                        pattern.description, pattern.usage_count
                    ),
                    suggestion: Some(format!("考虑将 '{}' 加入标准库", pattern.code)),
                    priority: pattern.usage_count,
                });
            }
        }

        insights.sort_by(|a, b| b.priority.cmp(&a.priority));
        insights
    }

    /// 获取学习洞察 / Get learning insights
    pub fn get_insights(&self) -> Vec<LearningInsight> {
        let mut insights = self.learn_from_errors();
        insights.extend(self.learn_from_success());
        insights.sort_by(|a, b| b.priority.cmp(&a.priority));
        insights
    }

    /// 分析使用统计 / Analyze usage statistics
    pub fn analyze_usage(&self) -> UsageStatistics {
        let total_usage: usize = self.usage_frequency.values().sum();
        let unique_patterns = self.usage_frequency.len();
        let total_errors: usize = self
            .error_patterns
            .values()
            .flat_map(|patterns| patterns.iter())
            .map(|p| p.count)
            .sum();
        let total_successes: usize = self
            .success_patterns
            .values()
            .flat_map(|patterns| patterns.iter())
            .map(|p| p.usage_count)
            .sum();

        UsageStatistics {
            total_usage,
            unique_patterns,
            total_errors,
            total_successes,
            error_rate: if total_usage > 0 {
                total_errors as f64 / total_usage as f64
            } else {
                0.0
            },
            success_rate: if total_usage > 0 {
                total_successes as f64 / total_usage as f64
            } else {
                0.0
            },
        }
    }
}

impl Default for UsagePatternLearner {
    fn default() -> Self {
        Self::new()
    }
}

/// 学习洞察 / Learning insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsight {
    /// 洞察类型 / Insight type
    pub insight_type: InsightType,
    /// 描述 / Description
    pub description: String,
    /// 建议 / Suggestion
    pub suggestion: Option<String>,
    /// 优先级 / Priority
    pub priority: usize,
}

/// 洞察类型 / Insight type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    /// 错误预防 / Error prevention
    ErrorPrevention,
    /// 模式识别 / Pattern recognition
    PatternRecognition,
    /// 性能优化 / Performance optimization
    PerformanceOptimization,
    /// 代码简化 / Code simplification
    CodeSimplification,
}

/// 使用统计 / Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatistics {
    /// 总使用次数 / Total usage count
    pub total_usage: usize,
    /// 唯一模式数 / Unique pattern count
    pub unique_patterns: usize,
    /// 总错误数 / Total error count
    pub total_errors: usize,
    /// 总成功数 / Total success count
    pub total_successes: usize,
    /// 错误率 / Error rate
    pub error_rate: f64,
    /// 成功率 / Success rate
    pub success_rate: f64,
}
