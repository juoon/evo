// 代码文档生成器 / Code documentation generator
// 基于代码分析自动生成代码文档
// Automatically generate code documentation based on code analysis

use crate::evolution::analyzer::CodeAnalysis;
use crate::grammar::core::GrammarElement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 代码文档生成器 / Code documentation generator
pub struct DocumentationGenerator {
    /// 文档模板库 / Documentation template library
    templates: HashMap<String, DocTemplate>,
    /// 文档历史 / Documentation history
    doc_history: Vec<DocRecord>,
}

/// 文档模板 / Documentation template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocTemplate {
    /// 模板名称 / Template name
    pub name: String,
    /// 模板格式 / Template format
    pub format: DocFormat,
    /// 模板内容 / Template content
    pub content: String,
    /// 使用次数 / Usage count
    pub usage_count: usize,
}

/// 文档格式 / Documentation format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocFormat {
    /// Markdown格式 / Markdown format
    Markdown,
    /// HTML格式 / HTML format
    Html,
    /// 纯文本格式 / Plain text format
    PlainText,
    /// API文档格式 / API documentation format
    ApiDoc,
}

/// 文档记录 / Documentation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocRecord {
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 文档类型 / Document type
    pub doc_type: String,
    /// 文档长度 / Document length
    pub doc_length: usize,
    /// 覆盖的函数数 / Functions covered
    pub functions_covered: usize,
}

/// 生成的文档 / Generated documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedDocumentation {
    /// 文档内容 / Document content
    pub content: String,
    /// 文档格式 / Document format
    pub format: DocFormat,
    /// 文档统计 / Document statistics
    pub statistics: DocStatistics,
    /// 文档质量 / Document quality
    pub quality: DocQuality,
}

/// 文档统计 / Documentation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocStatistics {
    /// 总行数 / Total lines
    pub total_lines: usize,
    /// 函数文档数 / Function docs count
    pub function_docs: usize,
    /// 变量文档数 / Variable docs count
    pub variable_docs: usize,
    /// 示例代码数 / Example code count
    pub example_count: usize,
}

/// 文档质量 / Documentation quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocQuality {
    /// 完整性 / Completeness
    pub completeness: f64,
    /// 清晰度 / Clarity
    pub clarity: f64,
    /// 准确性 / Accuracy
    pub accuracy: f64,
    /// 总体质量 / Overall quality
    pub overall: f64,
}

impl DocumentationGenerator {
    /// 创建新文档生成器 / Create new documentation generator
    pub fn new() -> Self {
        let mut generator = Self {
            templates: HashMap::new(),
            doc_history: Vec::new(),
        };
        generator.initialize_templates();
        generator
    }

    /// 初始化文档模板 / Initialize documentation templates
    fn initialize_templates(&mut self) {
        // Markdown函数文档模板 / Markdown function documentation template
        self.templates.insert(
            "function_markdown".to_string(),
            DocTemplate {
                name: "函数文档(Markdown)".to_string(),
                format: DocFormat::Markdown,
                content: r#"## {function_name}

### 描述 / Description
{description}

### 参数 / Parameters
{parameters}

### 返回值 / Returns
{returns}

### 示例 / Example
```aevo
{example}
```

### 复杂度 / Complexity
{complexity}
"#
                .to_string(),
                usage_count: 0,
            },
        );

        // API文档模板 / API documentation template
        self.templates.insert(
            "api_doc".to_string(),
            DocTemplate {
                name: "API文档".to_string(),
                format: DocFormat::ApiDoc,
                content: r#"# {module_name} API

## 函数列表 / Function List

{functions}

## 变量列表 / Variable List

{variables}
"#
                .to_string(),
                usage_count: 0,
            },
        );

        // 代码注释模板 / Code comment template
        self.templates.insert(
            "code_comment".to_string(),
            DocTemplate {
                name: "代码注释".to_string(),
                format: DocFormat::PlainText,
                content: r#";; {description}
;; 参数: {parameters}
;; 返回: {returns}
"#
                .to_string(),
                usage_count: 0,
            },
        );
    }

    /// 生成代码文档 / Generate code documentation
    pub fn generate_documentation(
        &mut self,
        ast: &[GrammarElement],
        analysis: &CodeAnalysis,
        format: DocFormat,
    ) -> GeneratedDocumentation {
        let mut content = String::new();
        let mut function_docs = 0;
        let mut variable_docs = 0;
        let mut example_count = 0;

        // 根据格式选择模板 / Select template based on format
        let template_key = match format {
            DocFormat::Markdown => "function_markdown",
            DocFormat::ApiDoc => "api_doc",
            _ => "code_comment",
        };

        // 生成文档内容 / Generate document content
        match format {
            DocFormat::Markdown => {
                content = self.generate_markdown_doc(ast, analysis);
                function_docs = analysis.statistics.function_count;
                variable_docs = analysis.statistics.variable_count;
            }
            DocFormat::ApiDoc => {
                content = self.generate_api_doc(ast, analysis);
                function_docs = analysis.statistics.function_count;
                variable_docs = analysis.statistics.variable_count;
            }
            DocFormat::PlainText => {
                content = self.generate_plain_doc(ast, analysis);
                function_docs = analysis.statistics.function_count;
                variable_docs = analysis.statistics.variable_count;
            }
            _ => {
                content = self.generate_markdown_doc(ast, analysis);
            }
        }

        // 统计文档信息 / Calculate document statistics
        let total_lines = content.lines().count();
        let statistics = DocStatistics {
            total_lines,
            function_docs,
            variable_docs,
            example_count,
        };

        // 评估文档质量 / Assess documentation quality
        let quality = self.assess_doc_quality(&statistics, analysis);

        // 记录文档生成历史 / Record documentation generation history
        let record = DocRecord {
            timestamp: chrono::Utc::now(),
            doc_type: format!("{:?}", format),
            doc_length: total_lines,
            functions_covered: function_docs,
        };
        self.doc_history.push(record);

        GeneratedDocumentation {
            content,
            format,
            statistics,
            quality,
        }
    }

    /// 生成Markdown文档 / Generate Markdown documentation
    fn generate_markdown_doc(&self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> String {
        let mut doc = String::from("# 代码文档 / Code Documentation\n\n");
        doc.push_str(&format!("## 概览 / Overview\n\n"));
        doc.push_str(&format!(
            "- 函数数量 / Function Count: {}\n",
            analysis.statistics.function_count
        ));
        doc.push_str(&format!(
            "- 变量数量 / Variable Count: {}\n",
            analysis.statistics.variable_count
        ));
        doc.push_str(&format!(
            "- 复杂度 / Complexity: {:.2}\n\n",
            analysis.complexity
        ));

        doc.push_str("## 函数文档 / Function Documentation\n\n");

        // 遍历AST生成函数文档 / Traverse AST to generate function documentation
        for element in ast {
            if let GrammarElement::List(list) = element {
                if let Some(GrammarElement::Atom(first)) = list.first() {
                    if first == "def" || first == "function" {
                        if list.len() >= 3 {
                            if let GrammarElement::Atom(name) = &list[1] {
                                doc.push_str(&format!("### {}\n\n", name));
                                doc.push_str(&format!("**描述 / Description**: 函数定义\n\n"));

                                // 提取参数 / Extract parameters
                                if let GrammarElement::List(params) = &list[2] {
                                    doc.push_str("**参数 / Parameters**:\n");
                                    for param in params {
                                        if let GrammarElement::Atom(p) = param {
                                            doc.push_str(&format!("- `{}`\n", p));
                                        }
                                    }
                                    doc.push_str("\n");
                                }

                                doc.push_str("**示例 / Example**:\n");
                                doc.push_str("```aevo\n");
                                doc.push_str(&format!("({} ...)\n", name));
                                doc.push_str("```\n\n");
                            }
                        }
                    }
                }
            }
        }

        doc.push_str("## 代码模式 / Code Patterns\n\n");
        for pattern in &analysis.patterns {
            doc.push_str(&format!(
                "- **{:?}**: {}\n",
                pattern.pattern_type, pattern.description
            ));
        }

        doc
    }

    /// 生成API文档 / Generate API documentation
    fn generate_api_doc(&self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> String {
        let mut doc = String::from("# API 文档 / API Documentation\n\n");

        doc.push_str("## 函数 / Functions\n\n");
        for element in ast {
            if let GrammarElement::List(list) = element {
                if let Some(GrammarElement::Atom(first)) = list.first() {
                    if first == "def" || first == "function" {
                        if list.len() >= 3 {
                            if let GrammarElement::Atom(name) = &list[1] {
                                doc.push_str(&format!("### `{}`\n\n", name));
                                doc.push_str("函数定义\n\n");
                            }
                        }
                    }
                }
            }
        }

        doc
    }

    /// 生成纯文本文档 / Generate plain text documentation
    fn generate_plain_doc(&self, ast: &[GrammarElement], analysis: &CodeAnalysis) -> String {
        let mut doc = String::new();

        for element in ast {
            if let GrammarElement::List(list) = element {
                if let Some(GrammarElement::Atom(first)) = list.first() {
                    if first == "def" || first == "function" {
                        if list.len() >= 3 {
                            if let GrammarElement::Atom(name) = &list[1] {
                                doc.push_str(&format!(";; 函数: {}\n", name));
                                doc.push_str(";; 描述: 函数定义\n");
                                doc.push_str("\n");
                            }
                        }
                    }
                }
            }
        }

        doc
    }

    /// 评估文档质量 / Assess documentation quality
    fn assess_doc_quality(&self, stats: &DocStatistics, analysis: &CodeAnalysis) -> DocQuality {
        // 完整性：文档覆盖的函数和变量比例 / Completeness: ratio of documented functions and variables
        let total_items = analysis.statistics.function_count + analysis.statistics.variable_count;
        let documented_items = stats.function_docs + stats.variable_docs;
        let completeness = if total_items > 0 {
            (documented_items as f64 / total_items as f64) * 100.0
        } else {
            100.0
        };

        // 清晰度：基于文档长度和内容 / Clarity: based on document length and content
        let clarity = if stats.total_lines > 0 {
            (stats.total_lines as f64 / (total_items.max(1) as f64 * 5.0)).min(1.0) * 100.0
        } else {
            0.0
        };

        // 准确性：基于代码分析结果 / Accuracy: based on code analysis results
        let accuracy = if analysis.complexity > 0.0 {
            (100.0 - (analysis.complexity / 10.0).min(10.0) * 10.0).max(0.0)
        } else {
            100.0
        };

        // 总体质量 / Overall quality
        let overall = (completeness * 0.4 + clarity * 0.3 + accuracy * 0.3).min(100.0);

        DocQuality {
            completeness,
            clarity,
            accuracy,
            overall,
        }
    }

    /// 获取文档历史 / Get documentation history
    pub fn get_doc_history(&self) -> &[DocRecord] {
        &self.doc_history
    }

    /// 获取文档统计 / Get documentation statistics
    pub fn get_doc_statistics(&self) -> serde_json::Value {
        if self.doc_history.is_empty() {
            return serde_json::json!({
                "total_docs": 0,
                "average_length": 0.0,
            });
        }

        let total_docs = self.doc_history.len();
        let total_length: usize = self.doc_history.iter().map(|r| r.doc_length).sum();
        let avg_length = total_length as f64 / total_docs as f64;

        serde_json::json!({
            "total_docs": total_docs,
            "total_length": total_length,
            "average_length": avg_length,
            "average_functions_covered": self.doc_history.iter().map(|r| r.functions_covered).sum::<usize>() as f64 / total_docs as f64,
        })
    }
}

impl Default for DocumentationGenerator {
    fn default() -> Self {
        Self::new()
    }
}
