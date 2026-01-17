// 代码分析器 / Code analyzer
// 分析代码模式，提供优化建议
// Analyzes code patterns and provides optimization suggestions

use crate::grammar::core::{BinOp, Expr, GrammarElement, Literal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 代码分析结果 / Code analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysis {
    /// 复杂度 / Complexity
    pub complexity: f64,
    /// 发现的模式 / Patterns found
    pub patterns: Vec<CodePattern>,
    /// 优化建议 / Optimization suggestions
    pub suggestions: Vec<OptimizationSuggestion>,
    /// 代码统计 / Code statistics
    pub statistics: CodeStatistics,
}

/// 代码模式 / Code pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePattern {
    /// 模式类型 / Pattern type
    pub pattern_type: PatternType,
    /// 模式描述 / Pattern description
    pub description: String,
    /// 位置 / Location
    pub location: String,
    /// 置信度 / Confidence
    pub confidence: f64,
}

/// 模式类型 / Pattern type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// 重复代码 / Duplicate code
    Duplicate,
    /// 长函数 / Long function
    LongFunction,
    /// 复杂表达式 / Complex expression
    ComplexExpression,
    /// 嵌套过深 / Deep nesting
    DeepNesting,
    /// 未使用的变量 / Unused variable
    UnusedVariable,
    /// 可以简化的代码 / Simplifiable code
    Simplifiable,
    /// 可以合并的代码 / Mergeable code
    Mergeable,
}

/// 优化建议 / Optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// 建议类型 / Suggestion type
    pub suggestion_type: SuggestionType,
    /// 描述 / Description
    pub description: String,
    /// 原代码 / Original code
    pub original: String,
    /// 建议代码 / Suggested code
    pub suggested: String,
    /// 改进程度 / Improvement level
    pub improvement: f64,
}

/// 建议类型 / Suggestion type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    /// 简化 / Simplify
    Simplify,
    /// 重构 / Refactor
    Refactor,
    /// 提取函数 / Extract function
    ExtractFunction,
    /// 合并表达式 / Merge expressions
    MergeExpressions,
    /// 消除重复 / Remove duplication
    RemoveDuplication,
    /// 优化性能 / Optimize performance
    OptimizePerformance,
}

/// 代码统计 / Code statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStatistics {
    /// 函数数量 / Function count
    pub function_count: usize,
    /// 变量数量 / Variable count
    pub variable_count: usize,
    /// 平均函数长度 / Average function length
    pub avg_function_length: f64,
    /// 最大嵌套深度 / Max nesting depth
    pub max_nesting_depth: usize,
    /// 表达式复杂度 / Expression complexity
    pub expression_complexity: f64,
}

/// 代码分析器 / Code analyzer
pub struct CodeAnalyzer;

impl CodeAnalyzer {
    /// 创建新代码分析器 / Create new code analyzer
    pub fn new() -> Self {
        Self
    }

    /// 分析代码 / Analyze code
    pub fn analyze(&self, ast: &[GrammarElement]) -> CodeAnalysis {
        let mut statistics = self.collect_statistics(ast);
        let patterns = self.detect_patterns(ast);
        let suggestions = self.generate_suggestions(ast, &patterns);
        
        // 计算复杂度 / Calculate complexity
        let complexity = self.calculate_complexity(ast, &statistics);

        CodeAnalysis {
            complexity,
            patterns,
            suggestions,
            statistics,
        }
    }

    /// 收集统计信息 / Collect statistics
    fn collect_statistics(&self, ast: &[GrammarElement]) -> CodeStatistics {
        let mut function_count = 0;
        let mut variable_count = 0;
        let mut total_function_length = 0;
        let mut max_nesting = 0;
        let mut total_complexity = 0.0;

        self.collect_statistics_recursive(ast, 0, &mut function_count, &mut variable_count, 
                                         &mut total_function_length, &mut max_nesting, &mut total_complexity);

        CodeStatistics {
            function_count,
            variable_count,
            avg_function_length: if function_count > 0 {
                total_function_length as f64 / function_count as f64
            } else {
                0.0
            },
            max_nesting_depth: max_nesting,
            expression_complexity: total_complexity,
        }
    }

    /// 递归收集统计信息 / Recursively collect statistics
    fn collect_statistics_recursive(
        &self,
        elements: &[GrammarElement],
        depth: usize,
        function_count: &mut usize,
        variable_count: &mut usize,
        total_function_length: &mut usize,
        max_nesting: &mut usize,
        total_complexity: &mut f64,
    ) {
        *max_nesting = (*max_nesting).max(depth);

        for element in elements {
            match element {
                GrammarElement::List(list) => {
                    if let Some(GrammarElement::Atom(first)) = list.first() {
                        match first.as_str() {
                            "def" | "function" => {
                                *function_count += 1;
                                *total_function_length += list.len();
                            }
                            "let" => {
                                *variable_count += 1;
                            }
                            _ => {}
                        }
                    }
                    self.collect_statistics_recursive(list, depth + 1, function_count, variable_count,
                                                     total_function_length, max_nesting, total_complexity);
                }
                GrammarElement::Expr(expr) => {
                    *total_complexity += self.expr_complexity(expr);
                }
                GrammarElement::Atom(_) => {}
                GrammarElement::NaturalLang(_) => {}
            }
        }
    }

    /// 计算表达式复杂度 / Calculate expression complexity
    fn expr_complexity(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Binary(_, left, right) => {
                1.0 + self.expr_complexity(left) + self.expr_complexity(right)
            }
            Expr::If(cond, then_expr, else_expr) => {
                2.0 + self.expr_complexity(cond) + self.expr_complexity(then_expr) + self.expr_complexity(else_expr)
            }
            Expr::Call(_, args) => {
                1.0 + args.iter().map(|a| self.expr_complexity(a)).sum::<f64>()
            }
            _ => 0.5,
        }
    }

    /// 检测模式 / Detect patterns
    fn detect_patterns(&self, ast: &[GrammarElement]) -> Vec<CodePattern> {
        let mut patterns = Vec::new();

        // 检测长函数 / Detect long functions
        self.detect_long_functions(ast, &mut patterns);
        
        // 检测复杂表达式 / Detect complex expressions
        self.detect_complex_expressions(ast, &mut patterns);
        
        // 检测深度嵌套 / Detect deep nesting
        self.detect_deep_nesting(ast, &mut patterns);

        patterns
    }

    /// 检测长函数 / Detect long functions
    fn detect_long_functions(&self, ast: &[GrammarElement], patterns: &mut Vec<CodePattern>) {
        for element in ast {
            if let GrammarElement::List(list) = element {
                if let Some(GrammarElement::Atom(first)) = list.first() {
                    if first == "def" || first == "function" {
                        if list.len() > 10 {
                            patterns.push(CodePattern {
                                pattern_type: PatternType::LongFunction,
                                description: format!("函数长度: {} 个元素", list.len()),
                                location: "function".to_string(),
                                confidence: 0.8,
                            });
                        }
                    }
                }
            }
        }
    }

    /// 检测复杂表达式 / Detect complex expressions
    fn detect_complex_expressions(&self, ast: &[GrammarElement], patterns: &mut Vec<CodePattern>) {
        for element in ast {
            if let GrammarElement::Expr(expr) = element {
                let complexity = self.expr_complexity(expr);
                if complexity > 5.0 {
                    patterns.push(CodePattern {
                        pattern_type: PatternType::ComplexExpression,
                        description: format!("表达式复杂度: {:.2}", complexity),
                        location: "expression".to_string(),
                        confidence: 0.7,
                    });
                }
            }
        }
    }

    /// 检测深度嵌套 / Detect deep nesting
    fn detect_deep_nesting(&self, ast: &[GrammarElement], patterns: &mut Vec<CodePattern>) {
        let max_depth = self.max_nesting_depth(ast, 0);
        if max_depth > 4 {
            patterns.push(CodePattern {
                pattern_type: PatternType::DeepNesting,
                description: format!("最大嵌套深度: {}", max_depth),
                location: "code".to_string(),
                confidence: 0.8,
            });
        }
    }

    /// 计算最大嵌套深度 / Calculate max nesting depth
    fn max_nesting_depth(&self, ast: &[GrammarElement], current: usize) -> usize {
        let mut max = current;
        for element in ast {
            if let GrammarElement::List(list) = element {
                let depth = self.max_nesting_depth(list, current + 1);
                max = max.max(depth);
            }
        }
        max
    }

    /// 生成优化建议 / Generate optimization suggestions
    fn generate_suggestions(&self, ast: &[GrammarElement], patterns: &[CodePattern]) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        for pattern in patterns {
            match &pattern.pattern_type {
                PatternType::LongFunction => {
                    suggestions.push(OptimizationSuggestion {
                        suggestion_type: SuggestionType::ExtractFunction,
                        description: "考虑将长函数拆分为多个小函数".to_string(),
                        original: "long function".to_string(),
                        suggested: "extracted functions".to_string(),
                        improvement: 0.7,
                    });
                }
                PatternType::ComplexExpression => {
                    suggestions.push(OptimizationSuggestion {
                        suggestion_type: SuggestionType::Simplify,
                        description: "考虑简化复杂表达式，提取中间变量".to_string(),
                        original: "complex expression".to_string(),
                        suggested: "simplified expression".to_string(),
                        improvement: 0.6,
                    });
                }
                PatternType::DeepNesting => {
                    suggestions.push(OptimizationSuggestion {
                        suggestion_type: SuggestionType::Refactor,
                        description: "考虑减少嵌套深度，使用早期返回或提取函数".to_string(),
                        original: "deep nesting".to_string(),
                        suggested: "flattened code".to_string(),
                        improvement: 0.8,
                    });
                }
                _ => {}
            }
        }

        suggestions
    }

    /// 计算代码复杂度 / Calculate code complexity
    fn calculate_complexity(&self, ast: &[GrammarElement], stats: &CodeStatistics) -> f64 {
        let base_complexity = stats.function_count as f64 * 2.0;
        let nesting_complexity = stats.max_nesting_depth as f64 * 3.0;
        let expression_complexity = stats.expression_complexity;
        
        base_complexity + nesting_complexity + expression_complexity
    }
}

impl Default for CodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// 代码重构器 / Code refactorer
pub struct CodeRefactorer;

impl CodeRefactorer {
    /// 创建新代码重构器 / Create new code refactorer
    pub fn new() -> Self {
        Self
    }

    /// 根据分析结果重构代码 / Refactor code based on analysis results
    pub fn refactor(&self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> Vec<GrammarElement> {
        let mut refactored = ast.to_vec();
        
        // 根据建议重构 / Refactor based on suggestions
        for suggestion in &analysis.suggestions {
            match suggestion.suggestion_type {
                SuggestionType::Simplify => {
                    refactored = self.simplify_expressions(&refactored);
                }
                SuggestionType::Refactor => {
                    refactored = self.reduce_nesting(&refactored);
                }
                SuggestionType::ExtractFunction => {
                    refactored = self.extract_functions(&refactored);
                }
                _ => {}
            }
        }

        refactored
    }

    /// 简化表达式 / Simplify expressions
    fn simplify_expressions(&self, ast: &[GrammarElement]) -> Vec<GrammarElement> {
        ast.iter().map(|elem| self.simplify_element(elem)).collect()
    }

    /// 简化元素 / Simplify element
    fn simplify_element(&self, element: &GrammarElement) -> GrammarElement {
        match element {
            GrammarElement::Expr(expr) => {
                GrammarElement::Expr(Box::new(self.simplify_expr(expr)))
            }
            GrammarElement::List(list) => {
                GrammarElement::List(list.iter().map(|e| self.simplify_element(e)).collect())
            }
            _ => element.clone(),
        }
    }

    /// 简化表达式 / Simplify expression
    fn simplify_expr(&self, expr: &Expr) -> Expr {
        match expr {
            Expr::Binary(op, left, right) => {
                // 尝试常量折叠 / Try constant folding
                if let (Expr::Literal(Literal::Int(a)), Expr::Literal(Literal::Int(b))) = (left.as_ref(), right.as_ref()) {
                    let result = match op {
                        BinOp::Add => a + b,
                        BinOp::Sub => a - b,
                        BinOp::Mul => a * b,
                        BinOp::Div => if *b != 0 { a / b } else { return expr.clone() },
                        _ => return expr.clone(),
                    };
                    return Expr::Literal(Literal::Int(result));
                }
                expr.clone()
            }
            Expr::If(cond, then_expr, else_expr) => {
                // 简化条件表达式 / Simplify conditional expression
                Expr::If(
                    Box::new(self.simplify_expr(cond)),
                    Box::new(self.simplify_expr(then_expr)),
                    Box::new(self.simplify_expr(else_expr)),
                )
            }
            Expr::Call(name, args) => {
                Expr::Call(name.clone(), args.iter().map(|a| self.simplify_expr(a)).collect())
            }
            _ => expr.clone(),
        }
    }

    /// 减少嵌套 / Reduce nesting
    fn reduce_nesting(&self, ast: &[GrammarElement]) -> Vec<GrammarElement> {
        // 简化版本：返回原代码 / Simplified version: return original code
        // 实际实现需要更复杂的逻辑 / Actual implementation needs more complex logic
        ast.to_vec()
    }

    /// 提取函数 / Extract functions
    fn extract_functions(&self, ast: &[GrammarElement]) -> Vec<GrammarElement> {
        // 简化版本：返回原代码 / Simplified version: return original code
        // 实际实现需要识别可提取的代码块 / Actual implementation needs to identify extractable code blocks
        ast.to_vec()
    }
}

impl Default for CodeRefactorer {
    fn default() -> Self {
        Self::new()
    }
}
