// 测试生成器 / Test generator
// 自动生成测试用例
// Automatically generate test cases

use crate::evolution::analyzer::CodeAnalysis;
use crate::grammar::core::GrammarElement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 测试生成器 / Test generator
pub struct TestGenerator {
    /// 测试策略库 / Test strategy library
    strategies: HashMap<String, TestStrategy>,
    /// 测试历史 / Test history
    test_history: Vec<TestRecord>,
}

/// 测试策略 / Test strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStrategy {
    /// 策略名称 / Strategy name
    pub name: String,
    /// 策略描述 / Strategy description
    pub description: String,
    /// 策略类型 / Strategy type
    pub strategy_type: TestStrategyType,
    /// 成功率 / Success rate
    pub success_rate: f64,
    /// 使用次数 / Usage count
    pub usage_count: usize,
}

/// 测试策略类型 / Test strategy type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestStrategyType {
    /// 单元测试 / Unit test
    UnitTest,
    /// 集成测试 / Integration test
    IntegrationTest,
    /// 边界测试 / Boundary test
    BoundaryTest,
    /// 性能测试 / Performance test
    PerformanceTest,
    /// 回归测试 / Regression test
    RegressionTest,
}

/// 测试用例 / Test case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    /// 测试ID / Test ID
    pub id: String,
    /// 测试名称 / Test name
    pub name: String,
    /// 测试代码 / Test code
    pub test_code: String,
    /// 预期结果 / Expected result
    pub expected_result: String,
    /// 测试类型 / Test type
    pub test_type: TestStrategyType,
    /// 描述 / Description
    pub description: String,
}

/// 测试记录 / Test record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRecord {
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 生成的测试数 / Tests generated
    pub tests_generated: usize,
    /// 测试通过数 / Tests passed
    pub tests_passed: usize,
    /// 测试失败数 / Tests failed
    pub tests_failed: usize,
}

/// 生成的测试套件 / Generated test suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// 测试用例列表 / Test cases
    pub test_cases: Vec<TestCase>,
    /// 测试统计 / Test statistics
    pub statistics: TestStatistics,
    /// 测试覆盖率 / Test coverage
    pub coverage: TestCoverage,
}

/// 测试统计 / Test statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStatistics {
    /// 总测试数 / Total tests
    pub total_tests: usize,
    /// 单元测试数 / Unit tests
    pub unit_tests: usize,
    /// 集成测试数 / Integration tests
    pub integration_tests: usize,
    /// 边界测试数 / Boundary tests
    pub boundary_tests: usize,
}

/// 测试覆盖率 / Test coverage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCoverage {
    /// 函数覆盖率 / Function coverage
    pub function_coverage: f64,
    /// 分支覆盖率 / Branch coverage
    pub branch_coverage: f64,
    /// 语句覆盖率 / Statement coverage
    pub statement_coverage: f64,
    /// 总体覆盖率 / Overall coverage
    pub overall_coverage: f64,
}

impl TestGenerator {
    /// 创建新测试生成器 / Create new test generator
    pub fn new() -> Self {
        let mut generator = Self {
            strategies: HashMap::new(),
            test_history: Vec::new(),
        };
        generator.initialize_strategies();
        generator
    }

    /// 初始化测试策略 / Initialize test strategies
    fn initialize_strategies(&mut self) {
        // 单元测试策略 / Unit test strategy
        self.strategies.insert(
            "unit_test".to_string(),
            TestStrategy {
                name: "单元测试".to_string(),
                description: "为函数生成单元测试".to_string(),
                strategy_type: TestStrategyType::UnitTest,
                success_rate: 0.85,
                usage_count: 0,
            },
        );

        // 边界测试策略 / Boundary test strategy
        self.strategies.insert(
            "boundary_test".to_string(),
            TestStrategy {
                name: "边界测试".to_string(),
                description: "生成边界值测试用例".to_string(),
                strategy_type: TestStrategyType::BoundaryTest,
                success_rate: 0.80,
                usage_count: 0,
            },
        );

        // 集成测试策略 / Integration test strategy
        self.strategies.insert(
            "integration_test".to_string(),
            TestStrategy {
                name: "集成测试".to_string(),
                description: "生成集成测试用例".to_string(),
                strategy_type: TestStrategyType::IntegrationTest,
                success_rate: 0.75,
                usage_count: 0,
            },
        );
    }

    /// 生成测试套件 / Generate test suite
    pub fn generate_tests(&mut self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> TestSuite {
        let mut test_cases = Vec::new();

        // 为每个函数生成测试 / Generate tests for each function
        for element in ast {
            if let GrammarElement::List(list) = element {
                if let Some(GrammarElement::Atom(first)) = list.first() {
                    if first == "def" || first == "function" {
                        if list.len() >= 3 {
                            if let GrammarElement::Atom(name) = &list[1] {
                                // 生成单元测试 / Generate unit tests
                                let unit_tests = self.generate_unit_tests(name, list);
                                test_cases.extend(unit_tests);

                                // 生成边界测试 / Generate boundary tests
                                let boundary_tests = self.generate_boundary_tests(name, list);
                                test_cases.extend(boundary_tests);
                            }
                        }
                    }
                }
            }
        }

        // 统计测试 / Count tests
        let unit_tests = test_cases
            .iter()
            .filter(|t| matches!(t.test_type, TestStrategyType::UnitTest))
            .count();
        let integration_tests = test_cases
            .iter()
            .filter(|t| matches!(t.test_type, TestStrategyType::IntegrationTest))
            .count();
        let boundary_tests = test_cases
            .iter()
            .filter(|t| matches!(t.test_type, TestStrategyType::BoundaryTest))
            .count();

        let statistics = TestStatistics {
            total_tests: test_cases.len(),
            unit_tests,
            integration_tests,
            boundary_tests,
        };

        // 计算测试覆盖率 / Calculate test coverage
        let coverage = self.calculate_coverage(&test_cases, analysis);

        // 记录测试生成历史 / Record test generation history
        let record = TestRecord {
            timestamp: chrono::Utc::now(),
            tests_generated: test_cases.len(),
            tests_passed: 0, // 将在实际运行后更新 / Will be updated after actual execution
            tests_failed: 0,
        };
        self.test_history.push(record);

        TestSuite {
            test_cases,
            statistics,
            coverage,
        }
    }

    /// 生成单元测试 / Generate unit tests
    fn generate_unit_tests(
        &self,
        function_name: &str,
        function_def: &[GrammarElement],
    ) -> Vec<TestCase> {
        let mut tests = Vec::new();

        // 根据函数名生成测试用例 / Generate test cases based on function name
        match function_name {
            "add" | "+" => {
                tests.push(TestCase {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: format!("test_{}_basic", function_name),
                    test_code: format!("({} 2 3)", function_name),
                    expected_result: "5".to_string(),
                    test_type: TestStrategyType::UnitTest,
                    description: "基本加法测试".to_string(),
                });
                tests.push(TestCase {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: format!("test_{}_zero", function_name),
                    test_code: format!("({} 0 5)", function_name),
                    expected_result: "5".to_string(),
                    test_type: TestStrategyType::UnitTest,
                    description: "零值测试".to_string(),
                });
            }
            "multiply" | "*" => {
                tests.push(TestCase {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: format!("test_{}_basic", function_name),
                    test_code: format!("({} 3 4)", function_name),
                    expected_result: "12".to_string(),
                    test_type: TestStrategyType::UnitTest,
                    description: "基本乘法测试".to_string(),
                });
            }
            _ => {
                // 通用测试用例 / Generic test case
                tests.push(TestCase {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: format!("test_{}_basic", function_name),
                    test_code: format!("({} 1 2)", function_name),
                    expected_result: "结果待验证".to_string(),
                    test_type: TestStrategyType::UnitTest,
                    description: format!("{} 函数基本测试", function_name),
                });
            }
        }

        tests
    }

    /// 生成边界测试 / Generate boundary tests
    fn generate_boundary_tests(
        &self,
        function_name: &str,
        _function_def: &[GrammarElement],
    ) -> Vec<TestCase> {
        let mut tests = Vec::new();

        // 生成边界值测试 / Generate boundary value tests
        tests.push(TestCase {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("test_{}_boundary_zero", function_name),
            test_code: format!("({} 0 0)", function_name),
            expected_result: "0".to_string(),
            test_type: TestStrategyType::BoundaryTest,
            description: "零值边界测试".to_string(),
        });

        tests.push(TestCase {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("test_{}_boundary_negative", function_name),
            test_code: format!("({} -1 1)", function_name),
            expected_result: "结果待验证".to_string(),
            test_type: TestStrategyType::BoundaryTest,
            description: "负值边界测试".to_string(),
        });

        tests
    }

    /// 计算测试覆盖率 / Calculate test coverage
    fn calculate_coverage(&self, test_cases: &[TestCase], analysis: &CodeAnalysis) -> TestCoverage {
        // 函数覆盖率：测试覆盖的函数比例 / Function coverage: ratio of functions covered by tests
        let function_coverage = if analysis.statistics.function_count > 0 {
            let covered_functions = test_cases.len().min(analysis.statistics.function_count);
            (covered_functions as f64 / analysis.statistics.function_count as f64) * 100.0
        } else {
            0.0
        };

        // 分支覆盖率：简化版本 / Branch coverage: simplified version
        let branch_coverage = if analysis.complexity > 0.0 {
            (100.0 - (analysis.complexity / 10.0).min(10.0) * 10.0).max(0.0)
        } else {
            100.0
        };

        // 语句覆盖率：基于测试数量 / Statement coverage: based on test count
        let statement_coverage = if test_cases.len() > 0 {
            (test_cases.len() as f64 / (analysis.statistics.function_count.max(1) as f64 * 3.0))
                .min(1.0)
                * 100.0
        } else {
            0.0
        };

        // 总体覆盖率 / Overall coverage
        let overall_coverage =
            (function_coverage * 0.4 + branch_coverage * 0.3 + statement_coverage * 0.3).min(100.0);

        TestCoverage {
            function_coverage,
            branch_coverage,
            statement_coverage,
            overall_coverage,
        }
    }

    /// 记录测试结果 / Record test results
    pub fn record_test_results(&mut self, passed: usize, failed: usize) {
        if let Some(record) = self.test_history.last_mut() {
            record.tests_passed = passed;
            record.tests_failed = failed;
        }
    }

    /// 获取测试历史 / Get test history
    pub fn get_test_history(&self) -> &[TestRecord] {
        &self.test_history
    }

    /// 获取测试统计 / Get test statistics
    pub fn get_test_statistics(&self) -> serde_json::Value {
        if self.test_history.is_empty() {
            return serde_json::json!({
                "total_tests_generated": 0,
                "average_tests_per_generation": 0.0,
            });
        }

        let total_generated: usize = self.test_history.iter().map(|r| r.tests_generated).sum();
        let total_passed: usize = self.test_history.iter().map(|r| r.tests_passed).sum();
        let total_failed: usize = self.test_history.iter().map(|r| r.tests_failed).sum();
        let avg_generated = total_generated as f64 / self.test_history.len() as f64;

        serde_json::json!({
            "total_tests_generated": total_generated,
            "total_tests_passed": total_passed,
            "total_tests_failed": total_failed,
            "average_tests_per_generation": avg_generated,
            "pass_rate": if total_generated > 0 {
                (total_passed as f64 / total_generated as f64) * 100.0
            } else {
                0.0
            },
        })
    }
}

impl Default for TestGenerator {
    fn default() -> Self {
        Self::new()
    }
}
