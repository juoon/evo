// 代码审查器 / Code reviewer
// 自动审查代码，提供详细的审查报告
// Automatically review code and provide detailed review reports

use crate::evolution::analyzer::CodeAnalysis;
use crate::evolution::quality_assessor::QualityAssessment;
use crate::grammar::core::GrammarElement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 代码审查器 / Code reviewer
pub struct CodeReviewer {
    /// 审查规则库 / Review rules library
    review_rules: HashMap<String, ReviewRule>,
    /// 审查历史 / Review history
    review_history: Vec<ReviewRecord>,
}

/// 审查规则 / Review rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRule {
    /// 规则名称 / Rule name
    pub name: String,
    /// 规则描述 / Rule description
    pub description: String,
    /// 规则类型 / Rule type
    pub rule_type: ReviewRuleType,
    /// 严重程度 / Severity
    pub severity: ReviewSeverity,
    /// 检查函数 / Check function (simplified: just a description)
    pub check_description: String,
}

/// 审查规则类型 / Review rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReviewRuleType {
    /// 代码风格 / Code style
    CodeStyle,
    /// 性能问题 / Performance issue
    Performance,
    /// 安全问题 / Security issue
    Security,
    /// 最佳实践 / Best practice
    BestPractice,
    /// 可维护性 / Maintainability
    Maintainability,
    /// 错误处理 / Error handling
    ErrorHandling,
}

/// 审查严重程度 / Review severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
pub enum ReviewSeverity {
    /// 信息 / Info
    Info,
    /// 警告 / Warning
    Warning,
    /// 错误 / Error
    Error,
    /// 严重 / Critical
    Critical,
}

/// 审查问题 / Review issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewIssue {
    /// 问题ID / Issue ID
    pub id: String,
    /// 规则名称 / Rule name
    pub rule_name: String,
    /// 问题描述 / Issue description
    pub description: String,
    /// 严重程度 / Severity
    pub severity: ReviewSeverity,
    /// 位置 / Location
    pub location: String,
    /// 建议 / Suggestion
    pub suggestion: String,
    /// 置信度 / Confidence
    pub confidence: f64,
}

/// 审查记录 / Review record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRecord {
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 审查的问题数 / Number of issues found
    pub issues_count: usize,
    /// 严重问题数 / Critical issues count
    pub critical_count: usize,
    /// 错误数 / Errors count
    pub error_count: usize,
    /// 警告数 / Warnings count
    pub warning_count: usize,
    /// 信息数 / Info count
    pub info_count: usize,
}

/// 代码审查结果 / Code review result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReviewResult {
    /// 审查的问题 / Review issues
    pub issues: Vec<ReviewIssue>,
    /// 审查摘要 / Review summary
    pub summary: ReviewSummary,
    /// 审查建议 / Review recommendations
    pub recommendations: Vec<String>,
    /// 审查等级 / Review grade
    pub grade: ReviewGrade,
}

/// 审查摘要 / Review summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSummary {
    /// 总问题数 / Total issues
    pub total_issues: usize,
    /// 严重问题数 / Critical issues
    pub critical_issues: usize,
    /// 错误数 / Errors
    pub errors: usize,
    /// 警告数 / Warnings
    pub warnings: usize,
    /// 信息数 / Info
    pub info: usize,
    /// 通过率 / Pass rate
    pub pass_rate: f64,
}

/// 审查等级 / Review grade
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReviewGrade {
    /// 优秀 / Excellent
    Excellent,
    /// 良好 / Good
    Good,
    /// 需要改进 / NeedsImprovement,
    NeedsImprovement,
    /// 不合格 / Failed
    Failed,
}

impl CodeReviewer {
    /// 创建新代码审查器 / Create new code reviewer
    pub fn new() -> Self {
        let mut reviewer = Self {
            review_rules: HashMap::new(),
            review_history: Vec::new(),
        };
        reviewer.initialize_rules();
        reviewer
    }

    /// 初始化审查规则 / Initialize review rules
    fn initialize_rules(&mut self) {
        // 代码风格规则 / Code style rules
        self.review_rules.insert(
            "naming_convention".to_string(),
            ReviewRule {
                name: "命名规范".to_string(),
                description: "检查变量和函数命名是否符合规范".to_string(),
                rule_type: ReviewRuleType::CodeStyle,
                severity: ReviewSeverity::Warning,
                check_description: "检查命名是否符合规范".to_string(),
            },
        );

        // 性能规则 / Performance rules
        self.review_rules.insert(
            "performance_issue".to_string(),
            ReviewRule {
                name: "性能问题".to_string(),
                description: "检查是否存在性能问题".to_string(),
                rule_type: ReviewRuleType::Performance,
                severity: ReviewSeverity::Warning,
                check_description: "检查复杂表达式和重复计算".to_string(),
            },
        );

        // 安全规则 / Security rules
        self.review_rules.insert(
            "security_issue".to_string(),
            ReviewRule {
                name: "安全问题".to_string(),
                description: "检查是否存在安全问题".to_string(),
                rule_type: ReviewRuleType::Security,
                severity: ReviewSeverity::Error,
                check_description: "检查输入验证和错误处理".to_string(),
            },
        );

        // 最佳实践规则 / Best practice rules
        self.review_rules.insert(
            "best_practice".to_string(),
            ReviewRule {
                name: "最佳实践".to_string(),
                description: "检查是否遵循最佳实践".to_string(),
                rule_type: ReviewRuleType::BestPractice,
                severity: ReviewSeverity::Info,
                check_description: "检查代码结构和组织".to_string(),
            },
        );

        // 可维护性规则 / Maintainability rules
        self.review_rules.insert(
            "maintainability".to_string(),
            ReviewRule {
                name: "可维护性".to_string(),
                description: "检查代码可维护性".to_string(),
                rule_type: ReviewRuleType::Maintainability,
                severity: ReviewSeverity::Warning,
                check_description: "检查复杂度、嵌套深度、函数长度".to_string(),
            },
        );
    }

    /// 审查代码 / Review code
    pub fn review_code(
        &mut self,
        ast: &[GrammarElement],
        analysis: &CodeAnalysis,
        quality: &QualityAssessment,
    ) -> CodeReviewResult {
        let mut issues = Vec::new();

        // 基于代码分析审查 / Review based on code analysis
        for pattern in &analysis.patterns {
            match pattern.pattern_type {
                crate::evolution::analyzer::PatternType::LongFunction => {
                    if let Some(rule) = self.review_rules.get("maintainability") {
                        issues.push(ReviewIssue {
                            id: uuid::Uuid::new_v4().to_string(),
                            rule_name: rule.name.clone(),
                            description: format!("发现长函数: {}", pattern.description),
                            severity: ReviewSeverity::Warning,
                            location: pattern.location.clone(),
                            suggestion: "考虑将长函数拆分为多个小函数".to_string(),
                            confidence: pattern.confidence,
                        });
                    }
                }
                crate::evolution::analyzer::PatternType::DeepNesting => {
                    if let Some(rule) = self.review_rules.get("maintainability") {
                        issues.push(ReviewIssue {
                            id: uuid::Uuid::new_v4().to_string(),
                            rule_name: rule.name.clone(),
                            description: format!("发现深度嵌套: {}", pattern.description),
                            severity: ReviewSeverity::Warning,
                            location: pattern.location.clone(),
                            suggestion: "减少嵌套深度，使用早期返回或提取函数".to_string(),
                            confidence: pattern.confidence,
                        });
                    }
                }
                crate::evolution::analyzer::PatternType::ComplexExpression => {
                    if let Some(rule) = self.review_rules.get("performance_issue") {
                        issues.push(ReviewIssue {
                            id: uuid::Uuid::new_v4().to_string(),
                            rule_name: rule.name.clone(),
                            description: format!("发现复杂表达式: {}", pattern.description),
                            severity: ReviewSeverity::Warning,
                            location: pattern.location.clone(),
                            suggestion: "简化表达式，提取中间变量".to_string(),
                            confidence: pattern.confidence,
                        });
                    }
                }
                crate::evolution::analyzer::PatternType::UnusedVariable => {
                    if let Some(rule) = self.review_rules.get("best_practice") {
                        issues.push(ReviewIssue {
                            id: uuid::Uuid::new_v4().to_string(),
                            rule_name: rule.name.clone(),
                            description: format!("发现未使用的变量: {}", pattern.description),
                            severity: ReviewSeverity::Info,
                            location: pattern.location.clone(),
                            suggestion: "移除未使用的变量".to_string(),
                            confidence: pattern.confidence,
                        });
                    }
                }
                _ => {}
            }
        }

        // 基于质量评估审查 / Review based on quality assessment
        if quality.dimension_scores.readability < 60.0 {
            if let Some(rule) = self.review_rules.get("maintainability") {
                issues.push(ReviewIssue {
                    id: uuid::Uuid::new_v4().to_string(),
                    rule_name: rule.name.clone(),
                    description: "代码可读性较低".to_string(),
                    severity: ReviewSeverity::Warning,
                    location: "整体".to_string(),
                    suggestion: "提高代码可读性，添加注释，简化结构".to_string(),
                    confidence: 0.8,
                });
            }
        }

        if quality.dimension_scores.performance < 60.0 {
            if let Some(rule) = self.review_rules.get("performance_issue") {
                issues.push(ReviewIssue {
                    id: uuid::Uuid::new_v4().to_string(),
                    rule_name: rule.name.clone(),
                    description: "代码性能需要优化".to_string(),
                    severity: ReviewSeverity::Warning,
                    location: "整体".to_string(),
                    suggestion: "优化性能，减少不必要的计算".to_string(),
                    confidence: 0.8,
                });
            }
        }

        if quality.dimension_scores.security < 60.0 {
            if let Some(rule) = self.review_rules.get("security_issue") {
                issues.push(ReviewIssue {
                    id: uuid::Uuid::new_v4().to_string(),
                    rule_name: rule.name.clone(),
                    description: "代码安全性需要改进".to_string(),
                    severity: ReviewSeverity::Error,
                    location: "整体".to_string(),
                    suggestion: "加强输入验证和错误处理".to_string(),
                    confidence: 0.8,
                });
            }
        }

        // 统计问题 / Count issues
        let critical_count = issues
            .iter()
            .filter(|i| i.severity == ReviewSeverity::Critical)
            .count();
        let error_count = issues
            .iter()
            .filter(|i| i.severity == ReviewSeverity::Error)
            .count();
        let warning_count = issues
            .iter()
            .filter(|i| i.severity == ReviewSeverity::Warning)
            .count();
        let info_count = issues
            .iter()
            .filter(|i| i.severity == ReviewSeverity::Info)
            .count();
        let total_issues = issues.len();

        // 计算通过率 / Calculate pass rate
        let pass_rate = if total_issues == 0 {
            100.0
        } else {
            let penalty = (critical_count as f64 * 20.0)
                + (error_count as f64 * 10.0)
                + (warning_count as f64 * 5.0)
                + (info_count as f64 * 1.0);
            (100.0 - penalty.min(100.0)).max(0.0)
        };

        // 确定审查等级 / Determine review grade
        let grade = if pass_rate >= 90.0 {
            ReviewGrade::Excellent
        } else if pass_rate >= 75.0 {
            ReviewGrade::Good
        } else if pass_rate >= 60.0 {
            ReviewGrade::NeedsImprovement
        } else {
            ReviewGrade::Failed
        };

        // 生成建议 / Generate recommendations
        let mut recommendations = Vec::new();
        if critical_count > 0 {
            recommendations.push(format!("修复 {} 个严重问题", critical_count));
        }
        if error_count > 0 {
            recommendations.push(format!("修复 {} 个错误", error_count));
        }
        if warning_count > 0 {
            recommendations.push(format!("处理 {} 个警告", warning_count));
        }
        if recommendations.is_empty() {
            recommendations.push("代码审查通过，继续保持".to_string());
        }

        // 创建摘要 / Create summary
        let summary = ReviewSummary {
            total_issues,
            critical_issues: critical_count,
            errors: error_count,
            warnings: warning_count,
            info: info_count,
            pass_rate,
        };

        // 记录审查历史 / Record review history
        let record = ReviewRecord {
            timestamp: chrono::Utc::now(),
            issues_count: total_issues,
            critical_count,
            error_count,
            warning_count,
            info_count,
        };
        self.review_history.push(record);

        CodeReviewResult {
            issues,
            summary,
            recommendations,
            grade,
        }
    }

    /// 获取审查历史 / Get review history
    pub fn get_review_history(&self) -> &[ReviewRecord] {
        &self.review_history
    }

    /// 获取审查统计 / Get review statistics
    pub fn get_review_statistics(&self) -> serde_json::Value {
        if self.review_history.is_empty() {
            return serde_json::json!({
                "total_reviews": 0,
                "average_issues": 0.0,
            });
        }

        let total_reviews = self.review_history.len();
        let total_issues: usize = self.review_history.iter().map(|r| r.issues_count).sum();
        let avg_issues = total_issues as f64 / total_reviews as f64;

        serde_json::json!({
            "total_reviews": total_reviews,
            "total_issues": total_issues,
            "average_issues": avg_issues,
            "average_critical": self.review_history.iter().map(|r| r.critical_count).sum::<usize>() as f64 / total_reviews as f64,
            "average_errors": self.review_history.iter().map(|r| r.error_count).sum::<usize>() as f64 / total_reviews as f64,
        })
    }
}

impl Default for CodeReviewer {
    fn default() -> Self {
        Self::new()
    }
}
