// 代码质量评估器 / Code quality assessor
// 评估代码质量，提供改进建议
// Assess code quality and provide improvement suggestions

use crate::evolution::analyzer::CodeAnalysis;
use crate::grammar::core::GrammarElement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 代码质量评估器 / Code quality assessor
pub struct QualityAssessor {
    /// 质量阈值 / Quality thresholds
    thresholds: QualityThresholds,
    /// 质量历史 / Quality history
    quality_history: Vec<QualitySnapshot>,
}

/// 质量阈值 / Quality thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityThresholds {
    /// 复杂度阈值 / Complexity threshold
    pub complexity_threshold: f64,
    /// 函数长度阈值 / Function length threshold
    pub function_length_threshold: usize,
    /// 嵌套深度阈值 / Nesting depth threshold
    pub nesting_depth_threshold: usize,
    /// 表达式复杂度阈值 / Expression complexity threshold
    pub expression_complexity_threshold: f64,
}

/// 质量快照 / Quality snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySnapshot {
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 总体质量分数 / Overall quality score
    pub overall_score: f64,
    /// 各维度分数 / Dimension scores
    pub dimension_scores: HashMap<String, f64>,
    /// 代码分析结果 / Code analysis result
    pub analysis: CodeAnalysis,
}

/// 质量评估结果 / Quality assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssessment {
    /// 总体质量分数 / Overall quality score (0-100)
    pub overall_score: f64,
    /// 各维度分数 / Dimension scores
    pub dimension_scores: QualityDimensions,
    /// 质量等级 / Quality grade
    pub grade: QualityGrade,
    /// 改进建议 / Improvement suggestions
    pub suggestions: Vec<QualitySuggestion>,
    /// 质量趋势 / Quality trend
    pub trend: QualityTrend,
}

/// 质量维度 / Quality dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDimensions {
    /// 可读性 / Readability
    pub readability: f64,
    /// 可维护性 / Maintainability
    pub maintainability: f64,
    /// 性能 / Performance
    pub performance: f64,
    /// 安全性 / Security
    pub security: f64,
    /// 简洁性 / Simplicity
    pub simplicity: f64,
}

/// 质量等级 / Quality grade
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QualityGrade {
    /// 优秀 / Excellent
    Excellent,
    /// 良好 / Good
    Good,
    /// 一般 / Average,
    Average,
    /// 需要改进 / NeedsImprovement,
    NeedsImprovement,
    /// 差 / Poor
    Poor,
}

/// 质量建议 / Quality suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitySuggestion {
    /// 建议类型 / Suggestion type
    pub suggestion_type: SuggestionType,
    /// 描述 / Description
    pub description: String,
    /// 优先级 / Priority
    pub priority: Priority,
    /// 改进方法 / Improvement method
    pub improvement: String,
}

/// 建议类型 / Suggestion type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    /// 简化代码 / Simplify code
    Simplify,
    /// 减少复杂度 / Reduce complexity
    ReduceComplexity,
    /// 提高可读性 / Improve readability
    ImproveReadability,
    /// 优化性能 / Optimize performance
    OptimizePerformance,
    /// 提高安全性 / Improve security
    ImproveSecurity,
}

/// 优先级 / Priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum Priority {
    /// 低 / Low
    Low,
    /// 中 / Medium
    Medium,
    /// 高 / High
    High,
    /// 紧急 / Critical
    Critical,
}

/// 质量趋势 / Quality trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityTrend {
    /// 改善 / Improving
    Improving,
    /// 稳定 / Stable
    Stable,
    /// 下降 / Declining
    Declining,
    /// 无历史数据 / NoHistory
    NoHistory,
}

impl QualityAssessor {
    /// 创建新质量评估器 / Create new quality assessor
    pub fn new() -> Self {
        Self {
            thresholds: QualityThresholds {
                complexity_threshold: 20.0,
                function_length_threshold: 50,
                nesting_depth_threshold: 5,
                expression_complexity_threshold: 10.0,
            },
            quality_history: Vec::new(),
        }
    }

    /// 评估代码质量 / Assess code quality
    pub fn assess(&mut self, analysis: &CodeAnalysis) -> QualityAssessment {
        // 计算各维度分数 / Calculate dimension scores
        let dimensions = self.calculate_dimensions(analysis);

        // 计算总体分数 / Calculate overall score
        let overall_score = self.calculate_overall_score(&dimensions);

        // 确定质量等级 / Determine quality grade
        let grade = self.determine_grade(overall_score);

        // 生成改进建议 / Generate improvement suggestions
        let suggestions = self.generate_suggestions(analysis, &dimensions);

        // 分析质量趋势 / Analyze quality trend
        let trend = self.analyze_trend(overall_score);

        // 保存快照 / Save snapshot
        let snapshot = QualitySnapshot {
            timestamp: chrono::Utc::now(),
            overall_score,
            dimension_scores: HashMap::from([
                ("readability".to_string(), dimensions.readability),
                ("maintainability".to_string(), dimensions.maintainability),
                ("performance".to_string(), dimensions.performance),
                ("security".to_string(), dimensions.security),
                ("simplicity".to_string(), dimensions.simplicity),
            ]),
            analysis: analysis.clone(),
        };
        self.quality_history.push(snapshot);

        QualityAssessment {
            overall_score,
            dimension_scores: dimensions,
            grade,
            suggestions,
            trend,
        }
    }

    /// 计算各维度分数 / Calculate dimension scores
    fn calculate_dimensions(&self, analysis: &CodeAnalysis) -> QualityDimensions {
        // 可读性：基于复杂度、嵌套深度、函数长度 / Readability: based on complexity, nesting depth, function length
        let readability = self.calculate_readability(analysis);

        // 可维护性：基于复杂度、模式数量 / Maintainability: based on complexity, pattern count
        let maintainability = self.calculate_maintainability(analysis);

        // 性能：基于表达式复杂度、优化建议 / Performance: based on expression complexity, optimization suggestions
        let performance = self.calculate_performance(analysis);

        // 安全性：基于错误模式、潜在问题 / Security: based on error patterns, potential issues
        let security = self.calculate_security(analysis);

        // 简洁性：基于代码统计、模式 / Simplicity: based on code statistics, patterns
        let simplicity = self.calculate_simplicity(analysis);

        QualityDimensions {
            readability,
            maintainability,
            performance,
            security,
            simplicity,
        }
    }

    /// 计算可读性 / Calculate readability
    fn calculate_readability(&self, analysis: &CodeAnalysis) -> f64 {
        let mut score = 100.0;

        // 复杂度惩罚 / Complexity penalty
        if analysis.complexity > self.thresholds.complexity_threshold {
            score -= (analysis.complexity - self.thresholds.complexity_threshold) * 2.0;
        }

        // 嵌套深度惩罚 / Nesting depth penalty
        if analysis.statistics.max_nesting_depth > self.thresholds.nesting_depth_threshold {
            score -= (analysis.statistics.max_nesting_depth - self.thresholds.nesting_depth_threshold) as f64 * 5.0;
        }

        // 函数长度惩罚 / Function length penalty
        if analysis.statistics.avg_function_length > self.thresholds.function_length_threshold as f64 {
            score -= (analysis.statistics.avg_function_length - self.thresholds.function_length_threshold as f64) * 0.5;
        }

        (score as f64).max(0.0_f64).min(100.0_f64)
    }

    /// 计算可维护性 / Calculate maintainability
    fn calculate_maintainability(&self, analysis: &CodeAnalysis) -> f64 {
        let mut score = 100.0;

        // 复杂度惩罚 / Complexity penalty
        score -= analysis.complexity * 1.5;

        // 模式惩罚（复杂模式越多，可维护性越低）/ Pattern penalty (more complex patterns = lower maintainability)
        for pattern in &analysis.patterns {
            match pattern.pattern_type {
                crate::evolution::analyzer::PatternType::LongFunction => score -= 5.0,
                crate::evolution::analyzer::PatternType::DeepNesting => score -= 8.0,
                crate::evolution::analyzer::PatternType::ComplexExpression => score -= 3.0,
                _ => {}
            }
        }

        (score as f64).max(0.0_f64).min(100.0_f64)
    }

    /// 计算性能 / Calculate performance
    fn calculate_performance(&self, analysis: &CodeAnalysis) -> f64 {
        let mut score = 100.0;

        // 表达式复杂度惩罚 / Expression complexity penalty
        if analysis.statistics.expression_complexity > self.thresholds.expression_complexity_threshold {
            score -= (analysis.statistics.expression_complexity - self.thresholds.expression_complexity_threshold) * 3.0;
        }

        // 优化建议数量（建议越多，性能问题越多）/ Number of optimization suggestions (more suggestions = more performance issues)
        score -= analysis.suggestions.len() as f64 * 2.0;

        (score as f64).max(0.0_f64).min(100.0_f64)
    }

    /// 计算安全性 / Calculate security
    fn calculate_security(&self, analysis: &CodeAnalysis) -> f64 {
        // 简化版本：基于错误模式 / Simplified version: based on error patterns
        let mut score = 100.0;

        for pattern in &analysis.patterns {
            if matches!(pattern.pattern_type, crate::evolution::analyzer::PatternType::UnusedVariable) {
                score -= 2.0;
            }
        }

        (score as f64).max(0.0_f64).min(100.0_f64)
    }

    /// 计算简洁性 / Calculate simplicity
    fn calculate_simplicity(&self, analysis: &CodeAnalysis) -> f64 {
        let mut score = 100.0;

        // 复杂度惩罚 / Complexity penalty
        score -= analysis.complexity * 2.0;

        // 函数数量（适中的函数数量更好）/ Function count (moderate function count is better)
        if analysis.statistics.function_count > 20 {
            score -= (analysis.statistics.function_count - 20) as f64 * 0.5;
        }

        // 可简化模式惩罚 / Simplifiable pattern penalty
        for pattern in &analysis.patterns {
            if matches!(pattern.pattern_type, crate::evolution::analyzer::PatternType::Simplifiable) {
                score -= 3.0;
            }
        }

        (score as f64).max(0.0_f64).min(100.0_f64)
    }

    /// 计算总体分数 / Calculate overall score
    fn calculate_overall_score(&self, dimensions: &QualityDimensions) -> f64 {
        // 加权平均 / Weighted average
        (dimensions.readability * 0.25
            + dimensions.maintainability * 0.25
            + dimensions.performance * 0.20
            + dimensions.security * 0.15
            + dimensions.simplicity * 0.15)
    }

    /// 确定质量等级 / Determine quality grade
    fn determine_grade(&self, score: f64) -> QualityGrade {
        if score >= 90.0 {
            QualityGrade::Excellent
        } else if score >= 75.0 {
            QualityGrade::Good
        } else if score >= 60.0 {
            QualityGrade::Average
        } else if score >= 40.0 {
            QualityGrade::NeedsImprovement
        } else {
            QualityGrade::Poor
        }
    }

    /// 生成改进建议 / Generate improvement suggestions
    fn generate_suggestions(
        &self,
        analysis: &CodeAnalysis,
        dimensions: &QualityDimensions,
    ) -> Vec<QualitySuggestion> {
        let mut suggestions = Vec::new();

        // 基于各维度分数生成建议 / Generate suggestions based on dimension scores
        if dimensions.readability < 70.0 {
            suggestions.push(QualitySuggestion {
                suggestion_type: SuggestionType::ImproveReadability,
                description: "代码可读性需要改进".to_string(),
                priority: if dimensions.readability < 50.0 {
                    Priority::High
                } else {
                    Priority::Medium
                },
                improvement: "减少嵌套深度，简化函数，添加注释".to_string(),
            });
        }

        if dimensions.maintainability < 70.0 {
            suggestions.push(QualitySuggestion {
                suggestion_type: SuggestionType::ReduceComplexity,
                description: "代码可维护性需要改进".to_string(),
                priority: if dimensions.maintainability < 50.0 {
                    Priority::High
                } else {
                    Priority::Medium
                },
                improvement: "降低复杂度，拆分长函数，减少复杂模式".to_string(),
            });
        }

        if dimensions.performance < 70.0 {
            suggestions.push(QualitySuggestion {
                suggestion_type: SuggestionType::OptimizePerformance,
                description: "代码性能需要优化".to_string(),
                priority: Priority::Medium,
                improvement: "简化表达式，减少不必要的计算".to_string(),
            });
        }

        if dimensions.simplicity < 70.0 {
            suggestions.push(QualitySuggestion {
                suggestion_type: SuggestionType::Simplify,
                description: "代码需要简化".to_string(),
                priority: Priority::Medium,
                improvement: "简化代码结构，减少复杂度".to_string(),
            });
        }

        // 基于分析结果添加建议 / Add suggestions based on analysis results
        for suggestion in &analysis.suggestions {
            suggestions.push(QualitySuggestion {
                suggestion_type: SuggestionType::Simplify,
                description: suggestion.description.clone(),
                priority: Priority::Medium,
                improvement: format!("{}: {} -> {}", 
                    format!("{:?}", suggestion.suggestion_type),
                    suggestion.original,
                    suggestion.suggested),
            });
        }

        suggestions.sort_by(|a, b| b.priority.cmp(&a.priority));
        suggestions
    }

    /// 分析质量趋势 / Analyze quality trend
    fn analyze_trend(&self, current_score: f64) -> QualityTrend {
        if self.quality_history.len() < 2 {
            return QualityTrend::NoHistory;
        }

        let recent_scores: Vec<f64> = self
            .quality_history
            .iter()
            .rev()
            .take(5)
            .map(|s| s.overall_score)
            .collect();

        if recent_scores.len() < 2 {
            return QualityTrend::NoHistory;
        }

        let avg_recent = recent_scores.iter().sum::<f64>() / recent_scores.len() as f64;
        let avg_older = if self.quality_history.len() > 5 {
            self.quality_history
                .iter()
                .rev()
                .skip(5)
                .take(5)
                .map(|s| s.overall_score)
                .sum::<f64>()
                / 5.0
        } else {
            recent_scores[recent_scores.len() - 1]
        };

        if avg_recent > avg_older + 2.0 {
            QualityTrend::Improving
        } else if avg_recent < avg_older - 2.0 {
            QualityTrend::Declining
        } else {
            QualityTrend::Stable
        }
    }

    /// 获取质量历史 / Get quality history
    pub fn get_quality_history(&self) -> &[QualitySnapshot] {
        &self.quality_history
    }
}

impl Default for QualityAssessor {
    fn default() -> Self {
        Self::new()
    }
}
