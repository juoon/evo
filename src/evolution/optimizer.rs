// 智能优化建议器 / Intelligent optimization advisor
// 基于质量评估和学习结果提供智能优化建议
// Provide intelligent optimization suggestions based on quality assessment and learning results

use crate::evolution::analyzer::CodeAnalysis;
use crate::evolution::learning::UsagePatternLearner;
use crate::evolution::quality_assessor::QualityAssessment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 智能优化建议器 / Intelligent optimization advisor
pub struct OptimizationAdvisor {
    /// 使用模式学习器 / Usage pattern learner
    learner: UsagePatternLearner,
    /// 优化策略库 / Optimization strategy library
    strategies: HashMap<String, OptimizationStrategy>,
    /// 优化历史 / Optimization history
    optimization_history: Vec<OptimizationRecord>,
}

/// 优化策略 / Optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStrategy {
    /// 策略名称 / Strategy name
    pub name: String,
    /// 策略描述 / Strategy description
    pub description: String,
    /// 适用场景 / Applicable scenarios
    pub scenarios: Vec<String>,
    /// 成功率 / Success rate
    pub success_rate: f64,
    /// 平均改进程度 / Average improvement
    pub avg_improvement: f64,
    /// 使用次数 / Usage count
    pub usage_count: usize,
}

/// 优化记录 / Optimization record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecord {
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 优化前质量分数 / Quality score before optimization
    pub before_score: f64,
    /// 优化后质量分数 / Quality score after optimization
    pub after_score: f64,
    /// 使用的策略 / Strategy used
    pub strategy: String,
    /// 改进程度 / Improvement
    pub improvement: f64,
}

/// 优化建议 / Optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// 建议ID / Suggestion ID
    pub id: String,
    /// 策略名称 / Strategy name
    pub strategy: String,
    /// 描述 / Description
    pub description: String,
    /// 优先级 / Priority
    pub priority: OptimizationPriority,
    /// 预期改进 / Expected improvement
    pub expected_improvement: f64,
    /// 具体建议 / Specific suggestion
    pub specific_suggestion: String,
    /// 置信度 / Confidence
    pub confidence: f64,
}

/// 优化优先级 / Optimization priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum OptimizationPriority {
    /// 低 / Low
    Low,
    /// 中 / Medium
    Medium,
    /// 高 / High
    High,
    /// 紧急 / Critical
    Critical,
}

/// 优化建议结果 / Optimization suggestion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// 优化建议列表 / Optimization suggestions
    pub suggestions: Vec<OptimizationSuggestion>,
    /// 总体预期改进 / Overall expected improvement
    pub overall_improvement: f64,
    /// 推荐策略 / Recommended strategies
    pub recommended_strategies: Vec<String>,
}

impl OptimizationAdvisor {
    /// 创建新优化建议器 / Create new optimization advisor
    pub fn new() -> Self {
        let mut advisor = Self {
            learner: UsagePatternLearner::new(),
            strategies: HashMap::new(),
            optimization_history: Vec::new(),
        };
        advisor.initialize_strategies();
        advisor
    }

    /// 初始化优化策略 / Initialize optimization strategies
    fn initialize_strategies(&mut self) {
        // 简化策略 / Simplification strategy
        self.strategies.insert(
            "simplify".to_string(),
            OptimizationStrategy {
                name: "简化代码".to_string(),
                description: "简化复杂表达式和代码结构".to_string(),
                scenarios: vec!["复杂表达式".to_string(), "嵌套过深".to_string()],
                success_rate: 0.85,
                avg_improvement: 15.0,
                usage_count: 0,
            },
        );

        // 重构策略 / Refactoring strategy
        self.strategies.insert(
            "refactor".to_string(),
            OptimizationStrategy {
                name: "重构代码".to_string(),
                description: "重构长函数和复杂结构".to_string(),
                scenarios: vec!["长函数".to_string(), "复杂结构".to_string()],
                success_rate: 0.80,
                avg_improvement: 20.0,
                usage_count: 0,
            },
        );

        // 性能优化策略 / Performance optimization strategy
        self.strategies.insert(
            "performance".to_string(),
            OptimizationStrategy {
                name: "性能优化".to_string(),
                description: "优化代码性能".to_string(),
                scenarios: vec!["性能瓶颈".to_string(), "复杂计算".to_string()],
                success_rate: 0.75,
                avg_improvement: 25.0,
                usage_count: 0,
            },
        );

        // 可读性优化策略 / Readability optimization strategy
        self.strategies.insert(
            "readability".to_string(),
            OptimizationStrategy {
                name: "可读性优化".to_string(),
                description: "提高代码可读性".to_string(),
                scenarios: vec!["可读性低".to_string(), "命名不规范".to_string()],
                success_rate: 0.90,
                avg_improvement: 18.0,
                usage_count: 0,
            },
        );
    }

    /// 生成优化建议 / Generate optimization suggestions
    pub fn suggest_optimizations(
        &mut self,
        analysis: &CodeAnalysis,
        quality: &QualityAssessment,
    ) -> OptimizationResult {
        let mut suggestions = Vec::new();
        let mut recommended_strategies = Vec::new();

        // 基于质量评估生成建议 / Generate suggestions based on quality assessment
        if quality.dimension_scores.readability < 70.0 {
            if let Some(strategy) = self.strategies.get("readability") {
                suggestions.push(self.create_suggestion(
                    "readability",
                    strategy,
                    "代码可读性需要改进",
                    quality.dimension_scores.readability,
                ));
                recommended_strategies.push("readability".to_string());
            }
        }

        if quality.dimension_scores.maintainability < 70.0 {
            if let Some(strategy) = self.strategies.get("refactor") {
                suggestions.push(self.create_suggestion(
                    "refactor",
                    strategy,
                    "代码可维护性需要改进",
                    quality.dimension_scores.maintainability,
                ));
                recommended_strategies.push("refactor".to_string());
            }
        }

        if quality.dimension_scores.performance < 70.0 {
            if let Some(strategy) = self.strategies.get("performance") {
                suggestions.push(self.create_suggestion(
                    "performance",
                    strategy,
                    "代码性能需要优化",
                    quality.dimension_scores.performance,
                ));
                recommended_strategies.push("performance".to_string());
            }
        }

        if quality.dimension_scores.simplicity < 70.0 {
            if let Some(strategy) = self.strategies.get("simplify") {
                suggestions.push(self.create_suggestion(
                    "simplify",
                    strategy,
                    "代码需要简化",
                    quality.dimension_scores.simplicity,
                ));
                recommended_strategies.push("simplify".to_string());
            }
        }

        // 基于代码分析生成建议 / Generate suggestions based on code analysis
        for pattern in &analysis.patterns {
            match pattern.pattern_type {
                crate::evolution::analyzer::PatternType::LongFunction => {
                    if let Some(strategy) = self.strategies.get("refactor") {
                        suggestions.push(self.create_suggestion(
                            "refactor",
                            strategy,
                            &format!("发现长函数: {}", pattern.description),
                            70.0,
                        ));
                    }
                }
                crate::evolution::analyzer::PatternType::ComplexExpression => {
                    if let Some(strategy) = self.strategies.get("simplify") {
                        suggestions.push(self.create_suggestion(
                            "simplify",
                            strategy,
                            &format!("发现复杂表达式: {}", pattern.description),
                            70.0,
                        ));
                    }
                }
                _ => {}
            }
        }

        // 基于学习结果优化建议 / Optimize suggestions based on learning results
        let insights = self.learner.get_insights();
        for insight in &insights {
            if insight.priority > 5 {
                if let Some(strategy) = self.strategies.get("simplify") {
                    suggestions.push(OptimizationSuggestion {
                        id: uuid::Uuid::new_v4().to_string(),
                        strategy: "simplify".to_string(),
                        description: insight.description.clone(),
                        priority: match insight.priority {
                            0..=3 => OptimizationPriority::Low,
                            4..=6 => OptimizationPriority::Medium,
                            7..=9 => OptimizationPriority::High,
                            _ => OptimizationPriority::Critical,
                        },
                        expected_improvement: strategy.avg_improvement,
                        specific_suggestion: insight.suggestion.clone().unwrap_or_default(),
                        confidence: strategy.success_rate,
                    });
                }
            }
        }

        // 排序建议 / Sort suggestions
        suggestions.sort_by(|a, b| {
            b.priority.cmp(&a.priority).then_with(|| {
                b.expected_improvement
                    .partial_cmp(&a.expected_improvement)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });

        // 计算总体预期改进 / Calculate overall expected improvement
        let overall_improvement = suggestions
            .iter()
            .take(5)
            .map(|s| s.expected_improvement)
            .sum::<f64>()
            / suggestions.len().max(1) as f64;

        OptimizationResult {
            suggestions,
            overall_improvement,
            recommended_strategies,
        }
    }

    /// 创建优化建议 / Create optimization suggestion
    fn create_suggestion(
        &self,
        strategy_id: &str,
        strategy: &OptimizationStrategy,
        description: &str,
        current_score: f64,
    ) -> OptimizationSuggestion {
        let expected_improvement = (100.0 - current_score).min(strategy.avg_improvement);
        let priority = if current_score < 40.0 {
            OptimizationPriority::Critical
        } else if current_score < 60.0 {
            OptimizationPriority::High
        } else if current_score < 75.0 {
            OptimizationPriority::Medium
        } else {
            OptimizationPriority::Low
        };

        OptimizationSuggestion {
            id: uuid::Uuid::new_v4().to_string(),
            strategy: strategy_id.to_string(),
            description: description.to_string(),
            priority,
            expected_improvement,
            specific_suggestion: strategy.description.clone(),
            confidence: strategy.success_rate,
        }
    }

    /// 记录优化结果 / Record optimization result
    pub fn record_optimization(&mut self, strategy: &str, before_score: f64, after_score: f64) {
        let improvement = after_score - before_score;
        let record = OptimizationRecord {
            timestamp: chrono::Utc::now(),
            before_score,
            after_score,
            strategy: strategy.to_string(),
            improvement,
        };
        self.optimization_history.push(record);

        // 更新策略统计 / Update strategy statistics
        if let Some(strategy_obj) = self.strategies.get_mut(strategy) {
            strategy_obj.usage_count += 1;
            if improvement > 0.0 {
                let total = strategy_obj.usage_count as f64;
                strategy_obj.avg_improvement =
                    (strategy_obj.avg_improvement * (total - 1.0) + improvement) / total;
                strategy_obj.success_rate =
                    (strategy_obj.success_rate * (total - 1.0) + 1.0) / total;
            } else {
                let total = strategy_obj.usage_count as f64;
                strategy_obj.success_rate = (strategy_obj.success_rate * (total - 1.0)) / total;
            }
        }
    }

    /// 获取优化历史 / Get optimization history
    pub fn get_optimization_history(&self) -> &[OptimizationRecord] {
        &self.optimization_history
    }

    /// 预测优化效果 / Predict optimization effect
    pub fn predict_optimization_effect(&self, strategy: &str, current_score: f64) -> f64 {
        if let Some(strategy_obj) = self.strategies.get(strategy) {
            let potential_improvement = (100.0 - current_score).min(strategy_obj.avg_improvement);
            current_score + potential_improvement * strategy_obj.success_rate
        } else {
            current_score
        }
    }
}

impl Default for OptimizationAdvisor {
    fn default() -> Self {
        Self::new()
    }
}
