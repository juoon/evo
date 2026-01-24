// 性能分析器 / Performance analyzer
// 分析代码性能，提供优化建议
// Analyze code performance and provide optimization suggestions

use crate::evolution::analyzer::CodeAnalysis;
use crate::grammar::core::GrammarElement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 性能分析器 / Performance analyzer
pub struct PerformanceAnalyzer {
    /// 性能基准 / Performance benchmarks
    benchmarks: HashMap<String, PerformanceBenchmark>,
    /// 性能历史 / Performance history
    performance_history: Vec<PerformanceRecord>,
}

/// 性能基准 / Performance benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBenchmark {
    /// 基准名称 / Benchmark name
    pub name: String,
    /// 基准类型 / Benchmark type
    pub benchmark_type: BenchmarkType,
    /// 预期性能 / Expected performance
    pub expected_performance: f64,
    /// 实际性能 / Actual performance
    pub actual_performance: f64,
}

/// 基准类型 / Benchmark type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenchmarkType {
    /// 时间复杂度 / Time complexity
    TimeComplexity,
    /// 空间复杂度 / Space complexity
    SpaceComplexity,
    /// 执行时间 / Execution time
    ExecutionTime,
    /// 内存使用 / Memory usage
    MemoryUsage,
}

/// 性能记录 / Performance record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecord {
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 性能指标 / Performance metrics
    pub metrics: PerformanceMetrics,
    /// 分析结果 / Analysis result
    pub analysis: PerformanceAnalysis,
}

/// 性能指标 / Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// 时间复杂度 / Time complexity
    pub time_complexity: String,
    /// 空间复杂度 / Space complexity
    pub space_complexity: String,
    /// 预估执行时间 / Estimated execution time
    pub estimated_execution_time: f64,
    /// 预估内存使用 / Estimated memory usage: f64,
    pub estimated_memory_usage: f64,
    /// 循环嵌套深度 / Loop nesting depth
    pub loop_nesting_depth: usize,
    /// 递归深度 / Recursion depth
    pub recursion_depth: usize,
}

/// 性能分析结果 / Performance analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    /// 性能评分 / Performance score
    pub performance_score: f64,
    /// 性能等级 / Performance level
    pub performance_level: PerformanceLevel,
    /// 瓶颈识别 / Bottleneck identification
    pub bottlenecks: Vec<Bottleneck>,
    /// 优化建议 / Optimization suggestions
    pub suggestions: Vec<OptimizationSuggestion>,
}

/// 性能等级 / Performance level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceLevel {
    /// 优秀 / Excellent
    Excellent,
    /// 良好 / Good
    Good,
    /// 一般 / Average
    Average,
    /// 需要改进 / NeedsImprovement,
    NeedsImprovement,
    /// 差 / Poor
    Poor,
}

/// 性能瓶颈 / Bottleneck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    /// 瓶颈类型 / Bottleneck type
    pub bottleneck_type: BottleneckType,
    /// 位置 / Location
    pub location: String,
    /// 描述 / Description
    pub description: String,
    /// 影响程度 / Impact level
    pub impact: f64,
}

/// 瓶颈类型 / Bottleneck type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckType {
    /// 深度嵌套 / Deep nesting
    DeepNesting,
    /// 递归调用 / Recursive calls
    RecursiveCalls,
    /// 重复计算 / Repeated computation
    RepeatedComputation,
    /// 内存泄漏 / Memory leak
    MemoryLeak,
    /// 低效算法 / Inefficient algorithm
    InefficientAlgorithm,
}

/// 优化建议 / Optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// 建议类型 / Suggestion type
    pub suggestion_type: String,
    /// 建议内容 / Suggestion content
    pub content: String,
    /// 预期改进 / Expected improvement
    pub expected_improvement: f64,
    /// 优先级 / Priority
    pub priority: usize,
}

impl PerformanceAnalyzer {
    /// 创建新性能分析器 / Create new performance analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            benchmarks: HashMap::new(),
            performance_history: Vec::new(),
        };
        analyzer.initialize_benchmarks();
        analyzer
    }

    /// 初始化性能基准 / Initialize performance benchmarks
    fn initialize_benchmarks(&mut self) {
        // 时间复杂度基准 / Time complexity benchmarks
        self.benchmarks.insert(
            "O(1)".to_string(),
            PerformanceBenchmark {
                name: "常数时间".to_string(),
                benchmark_type: BenchmarkType::TimeComplexity,
                expected_performance: 100.0,
                actual_performance: 0.0,
            },
        );

        self.benchmarks.insert(
            "O(n)".to_string(),
            PerformanceBenchmark {
                name: "线性时间".to_string(),
                benchmark_type: BenchmarkType::TimeComplexity,
                expected_performance: 80.0,
                actual_performance: 0.0,
            },
        );

        self.benchmarks.insert(
            "O(n^2)".to_string(),
            PerformanceBenchmark {
                name: "平方时间".to_string(),
                benchmark_type: BenchmarkType::TimeComplexity,
                expected_performance: 50.0,
                actual_performance: 0.0,
            },
        );
    }

    /// 分析代码性能 / Analyze code performance
    pub fn analyze_performance(
        &mut self,
        ast: &[GrammarElement],
        analysis: &CodeAnalysis,
    ) -> PerformanceAnalysis {
        // 计算性能指标 / Calculate performance metrics
        let metrics = self.calculate_metrics(ast, analysis);

        // 识别性能瓶颈 / Identify performance bottlenecks
        let bottlenecks = self.identify_bottlenecks(ast, analysis, &metrics);

        // 生成优化建议 / Generate optimization suggestions
        let suggestions = self.generate_suggestions(&metrics, &bottlenecks, analysis);

        // 计算性能评分 / Calculate performance score
        let performance_score = self.calculate_performance_score(&metrics, &bottlenecks);

        // 确定性能等级 / Determine performance level
        let performance_level = self.determine_performance_level(performance_score);

        let result = PerformanceAnalysis {
            performance_score,
            performance_level,
            bottlenecks,
            suggestions,
        };

        // 记录性能分析历史 / Record performance analysis history
        let record = PerformanceRecord {
            timestamp: chrono::Utc::now(),
            metrics: metrics.clone(),
            analysis: result.clone(),
        };
        self.performance_history.push(record);

        result
    }

    /// 计算性能指标 / Calculate performance metrics
    fn calculate_metrics(
        &self,
        ast: &[GrammarElement],
        analysis: &CodeAnalysis,
    ) -> PerformanceMetrics {
        // 分析时间复杂度 / Analyze time complexity
        let time_complexity = self.analyze_time_complexity(ast, analysis);

        // 分析空间复杂度 / Analyze space complexity
        let space_complexity = self.analyze_space_complexity(ast, analysis);

        // 估算执行时间 / Estimate execution time
        let estimated_execution_time = self.estimate_execution_time(ast, analysis);

        // 估算内存使用 / Estimate memory usage
        let estimated_memory_usage = self.estimate_memory_usage(ast, analysis);

        // 计算循环嵌套深度 / Calculate loop nesting depth
        let loop_nesting_depth = self.calculate_loop_nesting_depth(ast);

        // 计算递归深度 / Calculate recursion depth
        let recursion_depth = self.calculate_recursion_depth(ast);

        PerformanceMetrics {
            time_complexity,
            space_complexity,
            estimated_execution_time,
            estimated_memory_usage,
            loop_nesting_depth,
            recursion_depth,
        }
    }

    /// 分析时间复杂度 / Analyze time complexity
    fn analyze_time_complexity(&self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> String {
        // 基于代码复杂度估算时间复杂度 / Estimate time complexity based on code complexity
        if analysis.complexity < 10.0 {
            "O(1)".to_string()
        } else if analysis.complexity < 50.0 {
            "O(n)".to_string()
        } else if analysis.complexity < 200.0 {
            "O(n log n)".to_string()
        } else if analysis.complexity < 500.0 {
            "O(n^2)".to_string()
        } else {
            "O(n^3) or worse".to_string()
        }
    }

    /// 分析空间复杂度 / Analyze space complexity
    fn analyze_space_complexity(&self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> String {
        // 基于变量和函数数量估算空间复杂度 / Estimate space complexity based on variables and functions
        let total_items = analysis.statistics.variable_count + analysis.statistics.function_count;
        if total_items < 10 {
            "O(1)".to_string()
        } else if total_items < 50 {
            "O(n)".to_string()
        } else {
            "O(n^2) or worse".to_string()
        }
    }

    /// 估算执行时间 / Estimate execution time
    fn estimate_execution_time(&self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> f64 {
        // 基于复杂度估算 / Estimate based on complexity
        let base_time = 1.0; // 基础时间（毫秒）/ Base time (ms)
        let complexity_factor = analysis.complexity / 100.0;
        let function_factor = analysis.statistics.function_count as f64 * 0.1;
        base_time + complexity_factor + function_factor
    }

    /// 估算内存使用 / Estimate memory usage
    fn estimate_memory_usage(&self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> f64 {
        // 基于变量和函数数量估算 / Estimate based on variables and functions
        let base_memory = 1.0; // 基础内存（KB）/ Base memory (KB)
        let variable_factor = analysis.statistics.variable_count as f64 * 0.1;
        let function_factor = analysis.statistics.function_count as f64 * 0.5;
        base_memory + variable_factor + function_factor
    }

    /// 计算循环嵌套深度 / Calculate loop nesting depth
    fn calculate_loop_nesting_depth(&self, _ast: &[GrammarElement]) -> usize {
        // 简化版本：基于代码复杂度估算 / Simplified version: estimate based on complexity
        // 实际实现需要遍历AST识别循环 / Actual implementation needs to traverse AST to identify loops
        0
    }

    /// 计算递归深度 / Calculate recursion depth
    fn calculate_recursion_depth(&self, _ast: &[GrammarElement]) -> usize {
        // 简化版本：基于函数调用数量估算 / Simplified version: estimate based on function calls
        // 实际实现需要识别递归调用 / Actual implementation needs to identify recursive calls
        0
    }

    /// 识别性能瓶颈 / Identify performance bottlenecks
    fn identify_bottlenecks(
        &self,
        ast: &[GrammarElement],
        analysis: &CodeAnalysis,
        metrics: &PerformanceMetrics,
    ) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();

        // 检查深度嵌套 / Check deep nesting
        if analysis.statistics.max_nesting_depth > 5 {
            bottlenecks.push(Bottleneck {
                bottleneck_type: BottleneckType::DeepNesting,
                location: "代码整体".to_string(),
                description: format!(
                    "嵌套深度达到 {} 层，可能影响性能",
                    analysis.statistics.max_nesting_depth
                ),
                impact: (analysis.statistics.max_nesting_depth - 5) as f64 * 10.0,
            });
        }

        // 检查高复杂度 / Check high complexity
        if analysis.complexity > 200.0 {
            bottlenecks.push(Bottleneck {
                bottleneck_type: BottleneckType::InefficientAlgorithm,
                location: "代码整体".to_string(),
                description: format!(
                    "代码复杂度较高 ({:.2})，可能存在低效算法",
                    analysis.complexity
                ),
                impact: (analysis.complexity - 200.0) / 10.0,
            });
        }

        // 检查内存使用 / Check memory usage
        if metrics.estimated_memory_usage > 100.0 {
            bottlenecks.push(Bottleneck {
                bottleneck_type: BottleneckType::MemoryLeak,
                location: "内存使用".to_string(),
                description: format!(
                    "预估内存使用较高 ({:.2} KB)",
                    metrics.estimated_memory_usage
                ),
                impact: (metrics.estimated_memory_usage - 100.0) / 10.0,
            });
        }

        bottlenecks
    }

    /// 生成优化建议 / Generate optimization suggestions
    fn generate_suggestions(
        &self,
        _metrics: &PerformanceMetrics,
        bottlenecks: &[Bottleneck],
        analysis: &CodeAnalysis,
    ) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        // 基于瓶颈生成建议 / Generate suggestions based on bottlenecks
        for bottleneck in bottlenecks {
            match bottleneck.bottleneck_type {
                BottleneckType::DeepNesting => {
                    suggestions.push(OptimizationSuggestion {
                        suggestion_type: "重构建议".to_string(),
                        content: "考虑将深度嵌套的代码重构为多个函数，提高可读性和性能".to_string(),
                        expected_improvement: 15.0,
                        priority: 1,
                    });
                }
                BottleneckType::InefficientAlgorithm => {
                    suggestions.push(OptimizationSuggestion {
                        suggestion_type: "算法优化".to_string(),
                        content: "考虑使用更高效的算法或数据结构来降低时间复杂度".to_string(),
                        expected_improvement: 25.0,
                        priority: 1,
                    });
                }
                BottleneckType::MemoryLeak => {
                    suggestions.push(OptimizationSuggestion {
                        suggestion_type: "内存优化".to_string(),
                        content: "检查是否有未释放的资源，考虑使用更高效的数据结构".to_string(),
                        expected_improvement: 20.0,
                        priority: 2,
                    });
                }
                _ => {}
            }
        }

        // 基于复杂度生成建议 / Generate suggestions based on complexity
        if analysis.complexity > 100.0 {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: "代码简化".to_string(),
                content: "考虑简化代码逻辑，减少不必要的计算".to_string(),
                expected_improvement: 10.0,
                priority: 2,
            });
        }

        // 按优先级排序 / Sort by priority
        suggestions.sort_by_key(|s| s.priority);

        suggestions
    }

    /// 计算性能评分 / Calculate performance score
    fn calculate_performance_score(
        &self,
        metrics: &PerformanceMetrics,
        bottlenecks: &[Bottleneck],
    ) -> f64 {
        let mut score = 100.0;

        // 基于时间复杂度扣分 / Deduct based on time complexity
        match metrics.time_complexity.as_str() {
            "O(1)" => score -= 0.0,
            "O(n)" => score -= 10.0,
            "O(n log n)" => score -= 20.0,
            "O(n^2)" => score -= 30.0,
            _ => score -= 40.0,
        }

        // 基于瓶颈扣分 / Deduct based on bottlenecks
        for bottleneck in bottlenecks {
            score -= bottleneck.impact.min(30.0);
        }

        // 基于嵌套深度扣分 / Deduct based on nesting depth
        if metrics.loop_nesting_depth > 3 {
            score -= (metrics.loop_nesting_depth - 3) as f64 * 5.0;
        }

        score.max(0.0).min(100.0)
    }

    /// 确定性能等级 / Determine performance level
    fn determine_performance_level(&self, score: f64) -> PerformanceLevel {
        if score >= 90.0 {
            PerformanceLevel::Excellent
        } else if score >= 75.0 {
            PerformanceLevel::Good
        } else if score >= 60.0 {
            PerformanceLevel::Average
        } else if score >= 40.0 {
            PerformanceLevel::NeedsImprovement
        } else {
            PerformanceLevel::Poor
        }
    }

    /// 获取性能历史 / Get performance history
    pub fn get_performance_history(&self) -> &[PerformanceRecord] {
        &self.performance_history
    }

    /// 获取性能统计 / Get performance statistics
    pub fn get_performance_statistics(&self) -> serde_json::Value {
        if self.performance_history.is_empty() {
            return serde_json::json!({
                "total_analyses": 0,
                "average_score": 0.0,
            });
        }

        let total = self.performance_history.len();
        let avg_score = self
            .performance_history
            .iter()
            .map(|r| r.analysis.performance_score)
            .sum::<f64>()
            / total as f64;

        serde_json::json!({
            "total_analyses": total,
            "average_score": avg_score,
            "excellent_count": self.performance_history.iter().filter(|r| matches!(r.analysis.performance_level, PerformanceLevel::Excellent)).count(),
            "good_count": self.performance_history.iter().filter(|r| matches!(r.analysis.performance_level, PerformanceLevel::Good)).count(),
            "average_count": self.performance_history.iter().filter(|r| matches!(r.analysis.performance_level, PerformanceLevel::Average)).count(),
        })
    }
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
