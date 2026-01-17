// 代码依赖分析器 / Code dependency analyzer
// 分析代码依赖关系，检测循环依赖
// Analyze code dependencies and detect circular dependencies

use crate::evolution::analyzer::CodeAnalysis;
use crate::grammar::core::GrammarElement;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// 代码依赖分析器 / Code dependency analyzer
pub struct DependencyAnalyzer {
    /// 依赖图 / Dependency graph
    dependency_graph: HashMap<String, Vec<String>>,
    /// 分析历史 / Analysis history
    analysis_history: Vec<DependencyRecord>,
}

/// 依赖记录 / Dependency record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRecord {
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 依赖关系 / Dependencies
    pub dependencies: Vec<Dependency>,
    /// 循环依赖 / Circular dependencies
    pub circular_dependencies: Vec<CircularDependency>,
}

/// 依赖关系 / Dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// 依赖者 / Dependent
    pub dependent: String,
    /// 被依赖者 / Dependency
    pub dependency: String,
    /// 依赖类型 / Dependency type
    pub dependency_type: DependencyType,
    /// 位置 / Location
    pub location: String,
}

/// 依赖类型 / Dependency type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    /// 函数调用 / Function call
    FunctionCall,
    /// 变量引用 / Variable reference
    VariableReference,
    /// 模块导入 / Module import
    ModuleImport,
    /// 类型依赖 / Type dependency
    TypeDependency,
}

/// 循环依赖 / Circular dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularDependency {
    /// 循环路径 / Circular path
    pub path: Vec<String>,
    /// 严重程度 / Severity
    pub severity: Severity,
    /// 描述 / Description
    pub description: String,
}

/// 严重程度 / Severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    /// 严重 / Critical
    Critical,
    /// 高 / High
    High,
    /// 中 / Medium
    Medium,
    /// 低 / Low
    Low,
}

/// 依赖分析结果 / Dependency analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysis {
    /// 依赖关系列表 / Dependency list
    pub dependencies: Vec<Dependency>,
    /// 循环依赖列表 / Circular dependencies
    pub circular_dependencies: Vec<CircularDependency>,
    /// 依赖图统计 / Dependency graph statistics
    pub statistics: DependencyStatistics,
    /// 建议 / Suggestions
    pub suggestions: Vec<DependencySuggestion>,
}

/// 依赖统计 / Dependency statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyStatistics {
    /// 总依赖数 / Total dependencies
    pub total_dependencies: usize,
    /// 函数依赖数 / Function dependencies
    pub function_dependencies: usize,
    /// 变量依赖数 / Variable dependencies
    pub variable_dependencies: usize,
    /// 模块依赖数 / Module dependencies
    pub module_dependencies: usize,
    /// 循环依赖数 / Circular dependencies count
    pub circular_count: usize,
    /// 最大依赖深度 / Max dependency depth
    pub max_depth: usize,
}

/// 依赖建议 / Dependency suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySuggestion {
    /// 建议类型 / Suggestion type
    pub suggestion_type: String,
    /// 建议内容 / Suggestion content
    pub content: String,
    /// 优先级 / Priority
    pub priority: usize,
}

impl DependencyAnalyzer {
    /// 创建新依赖分析器 / Create new dependency analyzer
    pub fn new() -> Self {
        Self {
            dependency_graph: HashMap::new(),
            analysis_history: Vec::new(),
        }
    }

    /// 分析代码依赖 / Analyze code dependencies
    pub fn analyze_dependencies(
        &mut self,
        ast: &[GrammarElement],
        analysis: &CodeAnalysis,
    ) -> DependencyAnalysis {
        // 构建依赖图 / Build dependency graph
        self.build_dependency_graph(ast);

        // 提取依赖关系 / Extract dependencies
        let dependencies = self.extract_dependencies(ast);

        // 检测循环依赖 / Detect circular dependencies
        let circular_dependencies = self.detect_circular_dependencies();

        // 计算统计信息 / Calculate statistics
        let statistics = self.calculate_statistics(&dependencies, &circular_dependencies);

        // 生成建议 / Generate suggestions
        let suggestions =
            self.generate_suggestions(&dependencies, &circular_dependencies, statistics.max_depth);

        let result = DependencyAnalysis {
            dependencies: dependencies.clone(),
            circular_dependencies: circular_dependencies.clone(),
            statistics,
            suggestions,
        };

        // 记录分析历史 / Record analysis history
        let record = DependencyRecord {
            timestamp: chrono::Utc::now(),
            dependencies: result.dependencies.clone(),
            circular_dependencies: result.circular_dependencies.clone(),
        };
        self.analysis_history.push(record);

        result
    }

    /// 构建依赖图 / Build dependency graph
    fn build_dependency_graph(&mut self, ast: &[GrammarElement]) {
        self.dependency_graph.clear();

        for element in ast {
            if let GrammarElement::List(list) = element {
                if let Some(GrammarElement::Atom(first)) = list.first() {
                    if first == "def" || first == "function" {
                        if list.len() >= 3 {
                            if let GrammarElement::Atom(function_name) = &list[1] {
                                let mut deps = Vec::new();

                                // 分析函数体中的依赖 / Analyze dependencies in function body
                                if list.len() > 3 {
                                    self.analyze_element_dependencies(&list[3], &mut deps);
                                }

                                self.dependency_graph.insert(function_name.clone(), deps);
                            }
                        }
                    }
                }
            }
        }
    }

    /// 分析元素依赖 / Analyze element dependencies
    fn analyze_element_dependencies(&self, element: &GrammarElement, deps: &mut Vec<String>) {
        match element {
            GrammarElement::List(list) => {
                if let Some(GrammarElement::Atom(first)) = list.first() {
                    // 检查是否是函数调用 / Check if it's a function call
                    if first != "def" && first != "let" && first != "if" {
                        deps.push(first.clone());
                    }

                    // 递归分析列表中的其他元素 / Recursively analyze other elements in list
                    for item in list.iter().skip(1) {
                        self.analyze_element_dependencies(item, deps);
                    }
                }
            }
            GrammarElement::Atom(name) => {
                // 可能是变量引用 / Could be a variable reference
                deps.push(name.clone());
            }
            _ => {}
        }
    }

    /// 提取依赖关系 / Extract dependencies
    fn extract_dependencies(&self, ast: &[GrammarElement]) -> Vec<Dependency> {
        let mut dependencies = Vec::new();

        for (i, element) in ast.iter().enumerate() {
            if let GrammarElement::List(list) = element {
                if let Some(GrammarElement::Atom(first)) = list.first() {
                    if first == "def" || first == "function" {
                        if list.len() >= 3 {
                            if let GrammarElement::Atom(function_name) = &list[1] {
                                // 获取该函数的依赖 / Get dependencies for this function
                                if let Some(deps) = self.dependency_graph.get(function_name) {
                                    for dep in deps {
                                        dependencies.push(Dependency {
                                            dependent: function_name.clone(),
                                            dependency: dep.clone(),
                                            dependency_type: DependencyType::FunctionCall,
                                            location: format!("AST[{}]", i),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        dependencies
    }

    /// 检测循环依赖 / Detect circular dependencies
    fn detect_circular_dependencies(&self) -> Vec<CircularDependency> {
        let mut circular = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in self.dependency_graph.keys() {
            if !visited.contains(node) {
                let mut path = Vec::new();
                self.dfs_detect_cycle(node, &mut visited, &mut rec_stack, &mut path, &mut circular);
            }
        }

        circular
    }

    /// 深度优先搜索检测循环 / DFS to detect cycles
    fn dfs_detect_cycle(
        &self,
        node: &String,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        circular: &mut Vec<CircularDependency>,
    ) {
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        path.push(node.clone());

        if let Some(deps) = self.dependency_graph.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    self.dfs_detect_cycle(dep, visited, rec_stack, path, circular);
                } else if rec_stack.contains(dep) {
                    // 发现循环 / Found cycle
                    let cycle_start = path.iter().position(|x| x == dep).unwrap_or(0);
                    let mut cycle_path: Vec<String> = path[cycle_start..].to_vec();
                    cycle_path.push(dep.clone());

                    let severity = if cycle_path.len() <= 2 {
                        Severity::Critical
                    } else if cycle_path.len() <= 3 {
                        Severity::High
                    } else if cycle_path.len() <= 5 {
                        Severity::Medium
                    } else {
                        Severity::Low
                    };

                    circular.push(CircularDependency {
                        path: cycle_path.clone(),
                        severity,
                        description: format!("检测到循环依赖: {}", cycle_path.join(" -> ")),
                    });
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
    }

    /// 计算统计信息 / Calculate statistics
    fn calculate_statistics(
        &self,
        dependencies: &[Dependency],
        circular: &[CircularDependency],
    ) -> DependencyStatistics {
        let function_deps = dependencies
            .iter()
            .filter(|d| matches!(d.dependency_type, DependencyType::FunctionCall))
            .count();
        let variable_deps = dependencies
            .iter()
            .filter(|d| matches!(d.dependency_type, DependencyType::VariableReference))
            .count();
        let module_deps = dependencies
            .iter()
            .filter(|d| matches!(d.dependency_type, DependencyType::ModuleImport))
            .count();

        // 计算最大深度 / Calculate max depth
        let max_depth = self.calculate_max_depth();

        DependencyStatistics {
            total_dependencies: dependencies.len(),
            function_dependencies: function_deps,
            variable_dependencies: variable_deps,
            module_dependencies: module_deps,
            circular_count: circular.len(),
            max_depth,
        }
    }

    /// 计算最大依赖深度 / Calculate max dependency depth
    fn calculate_max_depth(&self) -> usize {
        let mut max_depth = 0;

        for node in self.dependency_graph.keys() {
            let depth = self.dfs_depth(node, &mut HashSet::new(), 0);
            max_depth = max_depth.max(depth);
        }

        max_depth
    }

    /// 深度优先搜索计算深度 / DFS to calculate depth
    fn dfs_depth(
        &self,
        node: &String,
        visited: &mut HashSet<String>,
        current_depth: usize,
    ) -> usize {
        if visited.contains(node) {
            return current_depth;
        }

        visited.insert(node.clone());
        let mut max_child_depth = current_depth;

        if let Some(deps) = self.dependency_graph.get(node) {
            for dep in deps {
                let child_depth = self.dfs_depth(dep, visited, current_depth + 1);
                max_child_depth = max_child_depth.max(child_depth);
            }
        }

        max_child_depth
    }

    /// 生成建议 / Generate suggestions
    fn generate_suggestions(
        &self,
        dependencies: &[Dependency],
        circular: &[CircularDependency],
        max_depth: usize,
    ) -> Vec<DependencySuggestion> {
        let mut suggestions = Vec::new();

        // 基于循环依赖生成建议 / Generate suggestions based on circular dependencies
        if !circular.is_empty() {
            suggestions.push(DependencySuggestion {
                suggestion_type: "循环依赖修复".to_string(),
                content: format!("发现 {} 个循环依赖，建议重构代码结构", circular.len()),
                priority: 1,
            });
        }

        // 基于依赖深度生成建议 / Generate suggestions based on dependency depth
        if max_depth > 10 {
            suggestions.push(DependencySuggestion {
                suggestion_type: "依赖深度优化".to_string(),
                content: format!("依赖深度较深 ({} 层)，建议减少依赖层级", max_depth),
                priority: 2,
            });
        }

        // 基于依赖数量生成建议 / Generate suggestions based on dependency count
        if dependencies.len() > 50 {
            suggestions.push(DependencySuggestion {
                suggestion_type: "依赖简化".to_string(),
                content: format!("依赖关系较多 ({} 个)，建议模块化设计", dependencies.len()),
                priority: 2,
            });
        }

        // 按优先级排序 / Sort by priority
        suggestions.sort_by_key(|s| s.priority);

        suggestions
    }

    /// 获取分析历史 / Get analysis history
    pub fn get_analysis_history(&self) -> &[DependencyRecord] {
        &self.analysis_history
    }

    /// 获取依赖统计 / Get dependency statistics
    pub fn get_dependency_statistics(&self) -> serde_json::Value {
        if self.analysis_history.is_empty() {
            return serde_json::json!({
                "total_analyses": 0,
                "average_dependencies": 0.0,
            });
        }

        let total = self.analysis_history.len();
        let total_deps: usize = self
            .analysis_history
            .iter()
            .map(|r| r.dependencies.len())
            .sum();
        let total_circular: usize = self
            .analysis_history
            .iter()
            .map(|r| r.circular_dependencies.len())
            .sum();

        serde_json::json!({
            "total_analyses": total,
            "average_dependencies": total_deps as f64 / total as f64,
            "average_circular_dependencies": total_circular as f64 / total as f64,
            "total_dependencies": total_deps,
            "total_circular_dependencies": total_circular,
        })
    }
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
