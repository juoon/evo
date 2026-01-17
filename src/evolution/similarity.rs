// 代码相似度检测器 / Code similarity detector
// 检测代码重复和相似模式
// Detect code duplication and similar patterns

use crate::evolution::analyzer::CodeAnalysis;
use crate::grammar::core::GrammarElement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 代码相似度检测器 / Code similarity detector
pub struct SimilarityDetector {
    /// 相似度阈值 / Similarity threshold
    similarity_threshold: f64,
    /// 检测历史 / Detection history
    detection_history: Vec<SimilarityRecord>,
}

/// 相似度记录 / Similarity record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityRecord {
    /// 时间戳 / Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// 检测到的相似代码对 / Detected similar code pairs
    pub similar_pairs: Vec<SimilarCodePair>,
    /// 重复代码块 / Duplicate code blocks
    pub duplicates: Vec<DuplicateBlock>,
}

/// 相似代码对 / Similar code pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarCodePair {
    /// 代码块1 / Code block 1
    pub block1: CodeBlock,
    /// 代码块2 / Code block 2
    pub block2: CodeBlock,
    /// 相似度 / Similarity score
    pub similarity: f64,
    /// 相似类型 / Similarity type
    pub similarity_type: SimilarityType,
}

/// 代码块 / Code block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    /// 代码内容 / Code content
    pub content: String,
    /// 位置 / Location
    pub location: String,
    /// 哈希值 / Hash value
    pub hash: String,
}

/// 重复代码块 / Duplicate code block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateBlock {
    /// 代码块 / Code block
    pub block: CodeBlock,
    /// 出现次数 / Occurrence count
    pub count: usize,
    /// 所有出现位置 / All occurrence locations
    pub locations: Vec<String>,
}

/// 相似类型 / Similarity type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimilarityType {
    /// 完全重复 / Exact duplicate
    ExactDuplicate,
    /// 结构相似 / Structural similarity
    StructuralSimilarity,
    /// 逻辑相似 / Logical similarity
    LogicalSimilarity,
    /// 命名相似 / Naming similarity
    NamingSimilarity,
}

/// 相似度分析结果 / Similarity analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityAnalysis {
    /// 相似度评分 / Similarity score
    pub similarity_score: f64,
    /// 相似代码对 / Similar code pairs
    pub similar_pairs: Vec<SimilarCodePair>,
    /// 重复代码块 / Duplicate code blocks
    pub duplicates: Vec<DuplicateBlock>,
    /// 建议 / Suggestions
    pub suggestions: Vec<SimilaritySuggestion>,
}

/// 相似度建议 / Similarity suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilaritySuggestion {
    /// 建议类型 / Suggestion type
    pub suggestion_type: String,
    /// 建议内容 / Suggestion content
    pub content: String,
    /// 优先级 / Priority
    pub priority: usize,
}

impl SimilarityDetector {
    /// 创建新相似度检测器 / Create new similarity detector
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.7, // 默认阈值70% / Default threshold 70%
            detection_history: Vec::new(),
        }
    }

    /// 设置相似度阈值 / Set similarity threshold
    pub fn set_threshold(&mut self, threshold: f64) {
        self.similarity_threshold = threshold.max(0.0).min(1.0);
    }

    /// 检测代码相似度 / Detect code similarity
    pub fn detect_similarity(
        &mut self,
        ast: &[GrammarElement],
        analysis: &CodeAnalysis,
    ) -> SimilarityAnalysis {
        // 提取代码块 / Extract code blocks
        let code_blocks = self.extract_code_blocks(ast);

        // 检测相似代码对 / Detect similar code pairs
        let similar_pairs = self.detect_similar_pairs(&code_blocks);

        // 检测重复代码块 / Detect duplicate blocks
        let duplicates = self.detect_duplicates(&code_blocks);

        // 生成建议 / Generate suggestions
        let suggestions = self.generate_suggestions(&similar_pairs, &duplicates);

        // 计算相似度评分 / Calculate similarity score
        let similarity_score =
            self.calculate_similarity_score(&similar_pairs, &duplicates, analysis);

        let result = SimilarityAnalysis {
            similarity_score,
            similar_pairs,
            duplicates,
            suggestions,
        };

        // 记录检测历史 / Record detection history
        let record = SimilarityRecord {
            timestamp: chrono::Utc::now(),
            similar_pairs: result.similar_pairs.clone(),
            duplicates: result.duplicates.clone(),
        };
        self.detection_history.push(record);

        result
    }

    /// 提取代码块 / Extract code blocks
    fn extract_code_blocks(&self, ast: &[GrammarElement]) -> Vec<CodeBlock> {
        let mut blocks = Vec::new();

        for (i, element) in ast.iter().enumerate() {
            let content = format!("{:?}", element);
            let location = format!("AST[{}]", i);
            let hash = self.calculate_hash(&content);

            blocks.push(CodeBlock {
                content,
                location,
                hash,
            });
        }

        blocks
    }

    /// 计算哈希值 / Calculate hash
    fn calculate_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// 检测相似代码对 / Detect similar code pairs
    fn detect_similar_pairs(&self, blocks: &[CodeBlock]) -> Vec<SimilarCodePair> {
        let mut pairs = Vec::new();

        for i in 0..blocks.len() {
            for j in (i + 1)..blocks.len() {
                let similarity = self.calculate_similarity(&blocks[i], &blocks[j]);

                if similarity >= self.similarity_threshold {
                    let similarity_type =
                        self.determine_similarity_type(&blocks[i], &blocks[j], similarity);

                    pairs.push(SimilarCodePair {
                        block1: blocks[i].clone(),
                        block2: blocks[j].clone(),
                        similarity,
                        similarity_type,
                    });
                }
            }
        }

        // 按相似度排序 / Sort by similarity
        pairs.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        pairs
    }

    /// 计算相似度 / Calculate similarity
    fn calculate_similarity(&self, block1: &CodeBlock, block2: &CodeBlock) -> f64 {
        // 如果哈希值相同，完全重复 / If hash is same, exact duplicate
        if block1.hash == block2.hash {
            return 1.0;
        }

        // 计算字符串相似度 / Calculate string similarity
        let string_similarity = self.string_similarity(&block1.content, &block2.content);

        // 计算结构相似度 / Calculate structural similarity
        let structural_similarity = self.structural_similarity(&block1.content, &block2.content);

        // 综合相似度 / Combined similarity
        (string_similarity * 0.4 + structural_similarity * 0.6).min(1.0)
    }

    /// 字符串相似度 / String similarity (简化的Levenshtein距离 / Simplified Levenshtein distance)
    fn string_similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }

        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let len1 = s1.chars().count();
        let len2 = s2.chars().count();
        let max_len = len1.max(len2);

        if max_len == 0 {
            return 1.0;
        }

        // 计算共同字符数 / Calculate common characters
        let common = self.count_common_chars(s1, s2);
        (common as f64 * 2.0) / (len1 + len2) as f64
    }

    /// 计算共同字符数 / Count common characters
    fn count_common_chars(&self, s1: &str, s2: &str) -> usize {
        let mut count = 0;
        let mut s2_chars: Vec<char> = s2.chars().collect();

        for c in s1.chars() {
            if let Some(pos) = s2_chars.iter().position(|&x| x == c) {
                s2_chars.remove(pos);
                count += 1;
            }
        }

        count
    }

    /// 结构相似度 / Structural similarity
    fn structural_similarity(&self, s1: &str, s2: &str) -> f64 {
        // 提取结构特征 / Extract structural features
        let features1 = self.extract_structural_features(s1);
        let features2 = self.extract_structural_features(s2);

        // 计算特征相似度 / Calculate feature similarity
        if features1.is_empty() && features2.is_empty() {
            return 1.0;
        }

        if features1.is_empty() || features2.is_empty() {
            return 0.0;
        }

        let common_features = features1.iter().filter(|f| features2.contains(f)).count();

        (common_features as f64 * 2.0) / (features1.len() + features2.len()) as f64
    }

    /// 提取结构特征 / Extract structural features
    fn extract_structural_features(&self, code: &str) -> Vec<String> {
        let mut features = Vec::new();

        // 提取括号 / Extract brackets
        let paren_count = code.matches('(').count();
        let bracket_count = code.matches('[').count();
        let brace_count = code.matches('{').count();

        features.push(format!("paren:{}", paren_count));
        features.push(format!("bracket:{}", bracket_count));
        features.push(format!("brace:{}", brace_count));

        // 提取关键字 / Extract keywords
        let keywords = vec!["def", "if", "let", "list", "dict", "function"];
        for keyword in keywords {
            if code.contains(keyword) {
                features.push(format!("keyword:{}", keyword));
            }
        }

        features
    }

    /// 确定相似类型 / Determine similarity type
    fn determine_similarity_type(
        &self,
        block1: &CodeBlock,
        block2: &CodeBlock,
        similarity: f64,
    ) -> SimilarityType {
        if similarity >= 0.99 {
            SimilarityType::ExactDuplicate
        } else if similarity >= 0.85 {
            SimilarityType::StructuralSimilarity
        } else if similarity >= 0.75 {
            SimilarityType::LogicalSimilarity
        } else {
            SimilarityType::NamingSimilarity
        }
    }

    /// 检测重复代码块 / Detect duplicate blocks
    fn detect_duplicates(&self, blocks: &[CodeBlock]) -> Vec<DuplicateBlock> {
        let mut hash_map: HashMap<String, Vec<usize>> = HashMap::new();

        // 按哈希值分组 / Group by hash
        for (i, block) in blocks.iter().enumerate() {
            hash_map
                .entry(block.hash.clone())
                .or_insert_with(Vec::new)
                .push(i);
        }

        let mut duplicates = Vec::new();

        // 找出出现多次的代码块 / Find blocks that appear multiple times
        for (hash, indices) in hash_map {
            if indices.len() > 1 {
                let block = blocks[indices[0]].clone();
                let locations: Vec<String> = indices
                    .iter()
                    .map(|&i| blocks[i].location.clone())
                    .collect();

                duplicates.push(DuplicateBlock {
                    block,
                    count: indices.len(),
                    locations,
                });
            }
        }

        // 按出现次数排序 / Sort by occurrence count
        duplicates.sort_by(|a, b| b.count.cmp(&a.count));

        duplicates
    }

    /// 生成建议 / Generate suggestions
    fn generate_suggestions(
        &self,
        similar_pairs: &[SimilarCodePair],
        duplicates: &[DuplicateBlock],
    ) -> Vec<SimilaritySuggestion> {
        let mut suggestions = Vec::new();

        // 基于重复代码生成建议 / Generate suggestions based on duplicate code
        if !duplicates.is_empty() {
            suggestions.push(SimilaritySuggestion {
                suggestion_type: "代码重构".to_string(),
                content: format!("发现 {} 个重复代码块，建议提取为公共函数", duplicates.len()),
                priority: 1,
            });
        }

        // 基于相似代码生成建议 / Generate suggestions based on similar code
        if similar_pairs.len() > 5 {
            suggestions.push(SimilaritySuggestion {
                suggestion_type: "代码优化".to_string(),
                content: format!(
                    "发现 {} 对相似代码，建议考虑使用模板或泛型",
                    similar_pairs.len()
                ),
                priority: 2,
            });
        }

        // 基于完全重复生成建议 / Generate suggestions based on exact duplicates
        let exact_duplicates = similar_pairs
            .iter()
            .filter(|p| matches!(p.similarity_type, SimilarityType::ExactDuplicate))
            .count();

        if exact_duplicates > 0 {
            suggestions.push(SimilaritySuggestion {
                suggestion_type: "代码去重".to_string(),
                content: format!("发现 {} 对完全重复的代码，建议立即重构", exact_duplicates),
                priority: 1,
            });
        }

        // 按优先级排序 / Sort by priority
        suggestions.sort_by_key(|s| s.priority);

        suggestions
    }

    /// 计算相似度评分 / Calculate similarity score
    fn calculate_similarity_score(
        &self,
        similar_pairs: &[SimilarCodePair],
        duplicates: &[DuplicateBlock],
        analysis: &CodeAnalysis,
    ) -> f64 {
        let mut score = 100.0;

        // 基于重复代码扣分 / Deduct based on duplicate code
        let duplicate_penalty = duplicates.len() as f64 * 5.0;
        score -= duplicate_penalty.min(30.0);

        // 基于相似代码扣分 / Deduct based on similar code
        let similar_penalty = similar_pairs.len() as f64 * 2.0;
        score -= similar_penalty.min(20.0);

        // 基于代码复杂度扣分 / Deduct based on code complexity
        if analysis.complexity > 100.0 {
            score -= ((analysis.complexity - 100.0) / 10.0).min(20.0);
        }

        score.max(0.0).min(100.0)
    }

    /// 获取检测历史 / Get detection history
    pub fn get_detection_history(&self) -> &[SimilarityRecord] {
        &self.detection_history
    }

    /// 获取相似度统计 / Get similarity statistics
    pub fn get_similarity_statistics(&self) -> serde_json::Value {
        if self.detection_history.is_empty() {
            return serde_json::json!({
                "total_detections": 0,
                "average_similar_pairs": 0.0,
            });
        }

        let total = self.detection_history.len();
        let total_similar_pairs: usize = self
            .detection_history
            .iter()
            .map(|r| r.similar_pairs.len())
            .sum();
        let total_duplicates: usize = self
            .detection_history
            .iter()
            .map(|r| r.duplicates.len())
            .sum();

        serde_json::json!({
            "total_detections": total,
            "average_similar_pairs": total_similar_pairs as f64 / total as f64,
            "average_duplicates": total_duplicates as f64 / total as f64,
            "total_similar_pairs": total_similar_pairs,
            "total_duplicates": total_duplicates,
        })
    }
}

impl Default for SimilarityDetector {
    fn default() -> Self {
        Self::new()
    }
}
